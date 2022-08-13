use bevy::{ecs::query::WorldQuery, prelude::*};

use characters::{Character, HitTracker, Hitbox, Hurtbox, OnHitEffect, Resources};
use input_parsing::InputParser;
use player_state::PlayerState;
use time::Clock;
use types::{Area, Facing, Owner, Player, Players, SoundEffect, VisualEffect};

use crate::{
    assets::{ParticleRequest, Particles, Sounds},
    physics::PlayerVelocity,
};

use super::{Combo, Defense, Health, HitboxSpawner};

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PlayerQuery<'a> {
    character: &'a Character,
    defense: &'a mut Defense,
    hurtbox: &'a Hurtbox,
    tf: &'a Transform,
    health: &'a mut Health,
    resources: &'a mut Resources,
    player: &'a Player,
    parser: &'a InputParser,
    state: &'a mut PlayerState,
    velocity: &'a mut PlayerVelocity,
    facing: &'a Facing,
    spawner: &'a mut HitboxSpawner,
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

        // TODO: Prettify
        if let Some(last_hit_frame) = tracker1.last_hit_frame {
            if last_hit_frame + FRAMES_BETWEEN_HITS > clock.frame {
                continue;
            }
        }
        if let Some(last_hit_frame) = tracker2.last_hit_frame {
            if last_hit_frame + FRAMES_BETWEEN_HITS > clock.frame {
                continue;
            }
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
                    tracker.hits -= 1;
                    tracker.last_hit_frame = Some(clock.frame);
                }
            }
        }
    }
}

pub(super) fn register_hits(
    mut commands: Commands,
    clock: Res<Clock>,
    combo: Option<Res<Combo>>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
    mut hitboxes: Query<(
        Entity,
        &Owner,
        &OnHitEffect,
        &GlobalTransform,
        &Hitbox,
        &mut HitTracker,
    )>,
    mut hurtboxes: Query<PlayerQuery>,
    players: Res<Players>,
) {
    for (entity, owner, effect, hitbox_tf, hitbox, mut hit_tracker) in &mut hitboxes {
        if let Ok([mut p1, mut p2]) = hurtboxes.get_many_mut([players.one, players.two]) {
            let (attacker, defender) = if owner.0 == Player::One {
                (&mut p1, &mut p2)
            } else {
                (&mut p2, &mut p1)
            };

            handle_hit(
                &mut commands,
                combo.is_some(),
                clock.frame,
                &mut sounds,
                &mut particles,
                effect,
                &mut hit_tracker,
                hitbox.with_offset(hitbox_tf.translation().truncate()),
                entity,
                attacker,
                defender,
            );
        }
    }
}

const FRAMES_BETWEEN_HITS: usize = 10;

#[allow(clippy::too_many_arguments)]
fn handle_hit(
    commands: &mut Commands,
    combo_ongoing: bool,
    frame: usize,
    sounds: &mut Sounds,
    particles: &mut Particles,
    effect: &OnHitEffect,
    hit_tracker: &mut HitTracker,
    hitbox: Area,
    hitbox_entity: Entity,
    attacker: &mut PlayerQueryItem,
    defender: &mut PlayerQueryItem,
) {
    if let Some(last_hit_frame) = hit_tracker.last_hit_frame {
        if last_hit_frame + FRAMES_BETWEEN_HITS > frame {
            return;
        }
    }

    if let Some(overlap) = defender
        .hurtbox
        .with_offset(defender.tf.translation.truncate())
        .intersection(&hitbox)
    {
        // Hit has happened
        if !combo_ongoing {
            commands.insert_resource(Combo);
        }

        // Handle blocking and state transitions here
        attacker.state.register_hit();

        let blocked = defender.state.blocked(
            effect.fixed_height,
            hitbox,
            defender.character.low_block_height,
            defender.character.high_block_height,
            defender.parser.get_relative_stick_position(),
        );

        // Damage
        let damage = effect.damage.get(blocked);
        defender.health.apply_damage(damage);

        // Knockback
        let knockback_impulse = attacker.facing.mirror_vec(effect.knockback.get(blocked));
        defender.velocity.add_impulse(knockback_impulse);

        // Pushback
        attacker
            .velocity
            // More intuitive to think of it from the defenders perspective
            .add_impulse(
                defender
                    .facing
                    .mirror_vec(defender.facing.mirror_vec(effect.pushback.get(blocked))),
            );

        // Stun
        if knockback_impulse.y > 0.0 {
            defender.state.launch();
        } else {
            let end_frame = effect.stun.get(blocked) + frame;
            if blocked {
                defender.state.block(end_frame);
            } else {
                defender.state.stun(end_frame);
            }
        }

        // Defense
        if blocked {
            defender.defense.bump_streak(frame);
            defender.resources.meter.gain(defender.defense.get_reward());
        } else {
            defender.defense.reset()
        }

        // Sound effect
        sounds.play(if blocked {
            SoundEffect::Block
        } else {
            SoundEffect::Hit
        });

        // Visual effect
        particles.spawn(ParticleRequest {
            effect: if blocked {
                VisualEffect::Block
            } else {
                VisualEffect::Hit
            },
            // TODO: This can be refined more
            position: overlap.center().extend(0.0),
        });

        hit_tracker.last_hit_frame = Some(frame);

        if !blocked {
            defender.spawner.despawn_on_hit(commands);
        }

        // Despawns
        if hit_tracker.hits <= 1 {
            attacker.spawner.despawn(commands, hitbox_entity);
        } else {
            hit_tracker.hits -= 1;
        }
    }
}
