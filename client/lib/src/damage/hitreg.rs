use bevy::{ecs::query::WorldQuery, prelude::*};
use strum::IntoEnumIterator;

use characters::{
    Action, Attack, AttackHeight, BlockType, Character, HitTracker, Hitbox, Hurtbox, Resources,
};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{
    Area, Clock, Facing, Owner, Player, Players, SoundEffect, StickPosition, VisualEffect,
    CLASH_PARRY_METER_GAIN,
};

use crate::{
    assets::{ParticleRequest, Particles, Sounds},
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

#[derive(Debug, PartialEq, Clone)]
pub(super) struct Hit {
    attacker: Entity,
    defender: Entity,
    hitbox: Entity,
    overlap: Area,
    hit_type: HitType,
    attack: Attack,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
/// Used for querying all the components that are required when a player is hit.
pub struct HitPlayerQuery<'a> {
    defense: &'a mut Defense,
    tf: &'a mut Transform,
    health: &'a mut Health,
    resources: &'a mut Resources,
    player: &'a Player,
    parser: &'a InputParser,
    state: &'a mut PlayerState,
    velocity: &'a mut PlayerVelocity,
    facing: &'a Facing,
    spawner: &'a mut HitboxSpawner,
    pushbox: &'a Pushbox,
}

pub(super) fn clash_parry(
    mut commands: Commands,
    clock: Res<Clock>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
    mut hitboxes: Query<(Entity, &Owner, &GlobalTransform, &Hitbox, &mut HitTracker)>,
    mut owners: Query<(&mut HitboxSpawner, &mut Resources)>,
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

            for (mut tracker, entity, owner) in
                [(tracker1, entity1, owner1), (tracker2, entity2, owner2)]
            {
                let (mut spawner, mut resources) = owners.get_mut(players.get(**owner)).unwrap();

                // Pay up
                let is_projectile = spawner
                    .is_projectile(entity)
                    .expect("to only check projectiles that have been spawned by this spawner");

                if !is_projectile {
                    resources.meter.gain(CLASH_PARRY_METER_GAIN);
                }

                // Despawn projectiles and consume hits
                if tracker.hits <= 1 {
                    spawner.despawn(&mut commands, entity);
                } else {
                    tracker.register_hit(clock.frame);
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
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
    hurtboxes: Query<(&Hurtbox, &Owner)>,
    defenders: Query<(&Transform, &PlayerState, &Character, &InputParser)>,
    mut spawners: Query<&mut HitboxSpawner>,
) -> Vec<Hit> {
    hitboxes
        .iter_mut()
        .filter_map(
            |(hitbox_entity, hit_owner, attack, hitbox_tf, hitbox, mut hit_tracker)| {
                if !hit_tracker.active(clock.frame) {
                    return None;
                }

                let attacking_player = **hit_owner;
                let defending_player = hit_owner.other();

                let defender = players.get(defending_player);
                let attacker = players.get(**hit_owner);
                let (defender_tf, state, character, parser) = defenders.get(defender).unwrap();

                let offset_hitbox = hitbox.with_offset(hitbox_tf.translation().truncate());

                // This technically doesn't get the actual overlap, as it just gets some overlap with one of the hitboxes
                let Some(overlap) = hurtboxes.iter().find_map(|(hurtbox, hurt_owner)| {
                    if **hurt_owner == **hit_owner{
                        None
                    } else {
                        // Different owners, hit can register
                        hurtbox.with_offset(defender_tf.translation.truncate()).intersection(&offset_hitbox)
                    }
                }) else {
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

                if hit_tracker.hits <= 1 {
                    spawners
                        .get_mut(attacker)
                        .unwrap()
                        .despawn(&mut commands, hitbox_entity);
                } else {
                    hit_tracker.register_hit(clock.frame)
                }

                let (hit_type, notification) = if !state.is_free() {
                    (
                        match attack.to_hit.block_type {
                            BlockType::Constant(_) | BlockType::Dynamic => HitType::Strike,
                            BlockType::Grab => HitType::Throw,
                        },
                        "Busy".into(),
                    )
                } else {
                    match attack.to_hit.block_type {
                        BlockType::Constant(height) => {
                            handle_blocking(height, parser.get_relative_stick_position())
                        }
                        BlockType::Grab => {
                            if teched(parser) {
                                notifications.add(defending_player, "Teched".into());

                                return None;
                            }

                            (HitType::Throw, "Grappled".into())
                        }
                        BlockType::Dynamic => handle_blocking(
                            if overlap.bottom() > character.high_block_height {
                                AttackHeight::High
                            } else if overlap.top() > character.low_block_height {
                                AttackHeight::Mid
                            } else {
                                AttackHeight::Low
                            },
                            parser.get_relative_stick_position(),
                        ),
                    }
                };

                if combo.is_none() {
                    notifications.add(defending_player, notification);
                }

                Some(Hit {
                    defender,
                    attacker,
                    hitbox: hitbox_entity,
                    overlap,
                    hit_type,
                    attack: attack.to_owned(),
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
    mut players: Query<HitPlayerQuery>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
) {
    for hit in hits {
        let [mut attacker, mut defender] =
            players.get_many_mut([hit.attacker, hit.defender]).unwrap();
        let blocked = hit.hit_type == HitType::Block;

        // Hit has happened
        if combo.is_none() {
            commands.insert_resource(Combo);
        }

        // Handle blocking and state transitions here
        attacker.state.register_hit();
        attacker.state.add_actions(if blocked {
            hit.attack.self_on_block
        } else {
            hit.attack.self_on_hit
        });
        defender.state.add_actions(if blocked {
            hit.attack.target_on_block
        } else {
            hit.attack.target_on_hit
        });

        // Defense
        if blocked {
            defender.defense.bump_streak(clock.frame);
            defender.resources.meter.gain(defender.defense.get_reward());
        } else {
            defender.defense.reset()
        }

        // Effects
        let (sound, particle) = match hit.hit_type {
            HitType::Block => (SoundEffect::Block, VisualEffect::Block),
            HitType::Strike => (SoundEffect::Hit, VisualEffect::Hit),
            HitType::Throw => (SoundEffect::Hit, VisualEffect::Hit), // TODO custom effects
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

fn handle_blocking(height: AttackHeight, stick: StickPosition) -> (HitType, String) {
    let blocking_high = stick == StickPosition::W;
    let blocking_low = stick == StickPosition::SW;

    if !(blocking_high || blocking_low) {
        (HitType::Strike, "Not blocking".into())
    } else if match height {
        AttackHeight::Low => blocking_low,
        AttackHeight::Mid => blocking_low || blocking_high,
        AttackHeight::High => blocking_high,
    } {
        (HitType::Block, "Blocked!".into())
    } else {
        (HitType::Strike, format!("Hit {:?}", height))
    }
}

fn teched(parser: &InputParser) -> bool {
    parser.head_is_clear()
}

pub(super) fn snap_and_switch(
    mut query: Query<(
        &mut PlayerState,
        &mut Transform,
        &Pushbox,
        &mut PlayerVelocity,
    )>,
    players: Res<Players>,
) {
    for player in Player::iter() {
        let [(mut state, mut self_tf, self_pushbox, mut self_velocity), (_, other_tf, other_pushbox, other_velocity)] =
            query
                .get_many_mut([players.get(player), players.get(player.other())])
                .unwrap();
        let actions = state.drain_matching_actions(|action| {
            if matches!(*action, Action::SnapToOpponent | Action::SideSwitch) {
                Some(action.to_owned())
            } else {
                None
            }
        });

        if actions.contains(&Action::SnapToOpponent) {
            let switch = actions.contains(&Action::SideSwitch) as i32 as f32;

            let raw_diff = self_tf.translation.x - other_tf.translation.x; // This ought to be positive when attacker is on the left
            let width_between = (self_pushbox.width() + other_pushbox.width()) / 2.0;

            let new_position = other_tf.translation
                + Vec3::X * raw_diff.signum() * width_between * (1.0 - (2.0 * switch));

            self_tf.translation = new_position;
            self_velocity.sync_with(&other_velocity);
        }
    }
}

pub(super) fn stun_actions(mut query: Query<&mut PlayerState>, clock: Res<Clock>) {
    for mut state in &mut query {
        for action in state.drain_matching_actions(|action| {
            if matches!(
                *action,
                Action::Launch | Action::HitStun(_) | Action::BlockStun(_)
            ) {
                Some(action.to_owned())
            } else {
                None
            }
        }) {
            match action {
                Action::HitStun(frames) => state.stun(clock.frame + frames),
                Action::BlockStun(frames) => state.block(clock.frame + frames),
                Action::Launch => state.launch(),
                _ => panic!("Leaking"),
            }
        }
    }
}
