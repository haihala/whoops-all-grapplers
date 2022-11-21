use bevy::{ecs::query::WorldQuery, prelude::*};

use characters::{
    Attack, AttackHeight, BlockType, Character, HitTracker, Hitbox, Hurtbox, OnHitEffect, Resources,
};
use input_parsing::InputParser;
use player_state::PlayerState;
use time::Clock;
use wag_core::{Area, Facing, Owner, Player, Players, SoundEffect, StickPosition, VisualEffect};

use crate::{
    assets::{AnimationHelper, AnimationRequest, ParticleRequest, Particles, Sounds},
    physics::{PlayerVelocity, Pushbox},
    ui::Notifications,
};

use super::{Combo, Defense, Health, HitboxSpawner};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum HitType {
    Strike,
    Block,
    Throw,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) struct Hit {
    attacker: Entity,
    defender: Entity,
    hitbox: Entity,
    overlap: Area,
    hit_type: HitType,
    effect: OnHitEffect,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PlayerQuery<'a> {
    character: &'a Character,
    defense: &'a mut Defense,
    hurtbox: &'a Hurtbox,
    tf: &'a mut Transform,
    health: &'a mut Health,
    resources: &'a mut Resources,
    player: &'a Player,
    parser: &'a InputParser,
    state: &'a mut PlayerState,
    velocity: &'a mut PlayerVelocity,
    facing: &'a Facing,
    spawner: &'a mut HitboxSpawner,
    animation_helper: &'a mut AnimationHelper,
    pushbox: &'a Pushbox,
}

pub(super) fn clash_parry(
    mut commands: Commands,
    clock: Res<Clock>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
    mut hitboxes: Query<(Entity, &Owner, &GlobalTransform, &Hitbox, &mut HitTracker)>,
    mut owners: Query<&mut HitboxSpawner>,
    players: Res<Players>,
) {
    let mut iter = hitboxes.iter_combinations_mut();
    while let Some(
        [(entity1, owner1, gtf1, hitbox1, tracker1), (entity2, owner2, gtf2, hitbox2, tracker2)],
    ) = iter.fetch_next()
    {
        if **owner1 == **owner2 {
            // Can't clash with your own boxes
            continue;
        }

        if !tracker1.active(clock.frame) || !tracker2.active(clock.frame) {
            continue;
        }

        if let Some(overlap) = hitbox1
            .with_offset(gtf1.translation().truncate())
            .intersection(&hitbox2.with_offset(gtf2.translation().truncate()))
        {
            // Hitboxes collide
            sounds.play(SoundEffect::Clash);
            particles.spawn(ParticleRequest {
                effect: VisualEffect::Clash,
                position: overlap.center().extend(0.0),
            });

            // Despawn projectiles
            for (mut tracker, entity, owner) in
                [(tracker1, entity1, owner1), (tracker2, entity2, owner2)]
            {
                if tracker.hits <= 1 {
                    owners
                        .get_mut(players.get(**owner))
                        .unwrap()
                        .despawn(&mut commands, entity);
                } else {
                    tracker.register_hit(clock.frame);
                }
            }
        }
    }
}

pub(super) fn detect_hits(
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    mut commands: Commands,
    combo: Option<Res<Combo>>,
    mut hitboxes: Query<(
        Entity,
        &Owner,
        &Attack,
        &GlobalTransform,
        &Hitbox,
        &mut HitTracker,
    )>,
    players: Res<Players>,
    mut hurtboxes: Query<(
        &mut HitboxSpawner,
        &Transform,
        &Hurtbox,
        &PlayerState,
        &Character,
        &InputParser,
    )>,
) -> Vec<Hit> {
    hitboxes
        .iter_mut()
        .filter_map(
            |(hitbox_entity, owner, attack, hitbox_tf, hitbox, mut hit_tracker)| {
                if !hit_tracker.active(clock.frame) {
                    return None;
                }

                let attacking_player = **owner;
                let defending_player = owner.other();

                let defender = players.get(defending_player);
                let (_, defender_tf, hurtbox, state, character, parser) =
                    hurtboxes.get(defender).unwrap();

                let Some(overlap) = hurtbox
                    .with_offset(defender_tf.translation.truncate())
                    .intersection(&hitbox.with_offset(hitbox_tf.translation().truncate()))
                else {
                    return None;
                };

                if state.is_intangible() {
                    if !hit_tracker.hit_intangible {
                        // Only send the notification once
                        hit_tracker.hit_intangible = true;
                        notifications.add(defending_player, "Intangible".to_owned());
                    }
                    return None;
                } else if hit_tracker.hit_intangible {
                    // Just a nice notification for now.
                    notifications.add(attacking_player, "Meaty!".to_owned());
                }

                let (hit_type, effect, notification) = if !state.is_free() {
                    (
                        match attack.to_hit.block_type {
                            BlockType::Constant(_) | BlockType::Dynamic => HitType::Strike,
                            BlockType::Grab => HitType::Throw,
                        },
                        attack.on_hit,
                        "Busy".into(),
                    )
                } else {
                    match attack.to_hit.block_type {
                        BlockType::Constant(height) => {
                            handle_blocking(height, attack, parser.get_relative_stick_position())
                        }
                        BlockType::Grab => {
                            if teched(parser) {
                                notifications.add(defending_player, "Teched".into());
                                return None;
                            }

                            (HitType::Throw, attack.on_hit, "Grappled".into())
                        }
                        BlockType::Dynamic => handle_blocking(
                            if overlap.bottom() > character.high_block_height {
                                AttackHeight::High
                            } else if overlap.top() > character.low_block_height {
                                AttackHeight::Mid
                            } else {
                                AttackHeight::Low
                            },
                            attack,
                            parser.get_relative_stick_position(),
                        ),
                    }
                };

                if combo.is_none() {
                    notifications.add(defending_player, notification);
                }

                if hit_tracker.hits <= 1 {
                    hurtboxes
                        .get_mut(players.get(attacking_player))
                        .unwrap()
                        .0
                        .despawn(&mut commands, hitbox_entity);
                } else {
                    hit_tracker.register_hit(clock.frame)
                }

                // Collision is happening
                Some(Hit {
                    defender,
                    attacker: players.get(**owner),
                    hitbox: hitbox_entity,
                    overlap,
                    hit_type,
                    effect,
                })
            },
        )
        .collect()
}

pub(super) fn apply_hits(
    In(hits): In<Vec<Hit>>,
    mut commands: Commands,
    combo: Option<Res<Combo>>,
    clock: Res<Clock>,
    mut players: Query<PlayerQuery>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
) {
    for hit in hits {
        let [mut attacker, mut defender] =
            players.get_many_mut([hit.attacker, hit.defender]).unwrap();
        let blocked = hit.hit_type == HitType::Block;
        let effect = hit.effect;

        // Hit has happened
        if combo.is_none() {
            commands.insert_resource(Combo);
        }

        // Handle blocking and state transitions here
        attacker.state.register_hit();

        defender.health.apply_damage(effect.damage);

        // Pushback
        attacker
            .velocity
            // More intuitive to think of it from the defenders perspective
            .add_impulse(
                defender
                    .facing
                    .mirror_vec2(defender.facing.mirror_vec2(effect.pushback)),
            );

        let knockback_impulse = attacker.facing.mirror_vec2(effect.knockback);

        // Stun
        if knockback_impulse.y > 0.0 {
            defender.state.launch();
        } else {
            let end_frame = effect.stun + clock.frame;
            if blocked {
                defender.state.block(end_frame);
            } else {
                defender.state.stun(end_frame);
            }
        }

        // Has to be after the stun, as state transitions would have a window of invalidity otherwise
        defender.velocity.add_impulse(knockback_impulse);

        // Defense
        if blocked {
            defender.defense.bump_streak(clock.frame);
            defender.resources.meter.gain(defender.defense.get_reward());
        } else {
            defender.defense.reset()
        }

        // Forced animation (throw)
        if let Some(forced_animation) = effect.forced_animation {
            // Snap players together
            let raw_diff = defender.tf.translation.x - attacker.tf.translation.x; // This ougth to be positive when attacker is on the left
            let width_between = (attacker.pushbox.width() + defender.pushbox.width()) / 2.0;

            let new_position = attacker.tf.translation
                + Vec3::X
                    * raw_diff.signum()
                    * width_between
                    * (1.0 - (2.0 * effect.side_switch as i32 as f32));

            defender.tf.translation = new_position;

            let position_offset = (attacker.tf.translation - new_position).truncate();
            defender.animation_helper.play(AnimationRequest {
                invert: true,
                position_offset,
                ..forced_animation.into()
            });
        }

        // Effects
        let (sound, particle) = match hit.hit_type {
            HitType::Block => (SoundEffect::Block, VisualEffect::Block),
            HitType::Strike => (SoundEffect::Hit, VisualEffect::Hit),
            HitType::Throw => todo!(),
        };

        sounds.play(sound);
        particles.spawn(ParticleRequest {
            effect: particle,
            // TODO: This can be refined more
            position: hit.overlap.center().extend(0.0),
        });

        defender.spawner.despawn_on_hit(&mut commands);
    }
}

fn handle_blocking(
    height: AttackHeight,
    attack: &Attack,
    stick: StickPosition,
) -> (HitType, OnHitEffect, String) {
    let blocking_high = stick == StickPosition::W;
    let blocking_low = stick == StickPosition::SW;

    if !(blocking_high || blocking_low) {
        (HitType::Strike, attack.on_hit, "Not blocking".into())
    } else if match dbg!(height) {
        AttackHeight::Low => blocking_low,
        AttackHeight::Mid => blocking_low || blocking_high,
        AttackHeight::High => blocking_high,
    } {
        (
            HitType::Block,
            attack.on_block.unwrap_or_default(), // This kinda fucks up how moves are created, but that's fixable
            "Blocked!".into(),
        )
    } else {
        (HitType::Strike, attack.on_hit, format!("Hit {:?}", height))
    }
}

fn teched(parser: &InputParser) -> bool {
    parser.head_is_clear()
}
