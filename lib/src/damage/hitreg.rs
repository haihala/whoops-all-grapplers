use bevy::{
    ecs::query::{Fetch, WorldQuery},
    prelude::*,
};

use characters::{Character, Grabable, HitTracker, Hitbox, Hurtbox, OnHitEffect, Resources};
use input_parsing::InputParser;
use player_state::PlayerState;
use time::Clock;
use types::{Area, Facing, Owner, Player, Players, SoundEffect, VisualEffect};

use crate::{
    assets::{ParticleRequest, Particles, Sounds},
    physics::PlayerVelocity,
};

use super::{Health, HitboxSpawner};

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PlayerQuery<'a> {
    character: &'a Character,
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
    mut sounds: Option<ResMut<Sounds>>,
    mut particles: Option<ResMut<Particles>>,
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
            .with_offset(gtf1.translation.truncate())
            .intersection(&hitbox2.with_offset(gtf2.translation.truncate()))
        {
            // Hitboxes collide

            // Sound effect
            if let Some(ref mut sounds) = sounds {
                sounds.play(SoundEffect::Clash);
            }

            // Visual effect
            if let Some(ref mut particles) = particles {
                particles.spawn(ParticleRequest {
                    effect: VisualEffect::Clash,
                    // TODO: This can be refined more
                    position: overlap.center().extend(0.0),
                });
            }

            // Despawn projectiles
            for (mut tracker, entity, owner) in
                [(tracker1, entity1, owner1), (tracker2, entity2, owner2)]
            {
                tracker.hits -= 1;
                tracker.last_hit_frame = Some(clock.frame);

                if tracker.hits <= 0 {
                    owners
                        .get_mut(players.get(**owner))
                        .unwrap()
                        .despawn(&mut commands, entity);
                }
            }
        }
    }
}

pub(super) fn register_hits(
    mut commands: Commands,
    clock: Res<Clock>,
    mut sounds: Option<ResMut<Sounds>>,
    mut particles: Option<ResMut<Particles>>,
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
    for (entity, owner, effect, hitbox_tf, hitbox, mut hit_tracker) in hitboxes.iter_mut() {
        if let Ok([mut p1, mut p2]) = hurtboxes.get_many_mut([players.one, players.two]) {
            let (attacker, defender) = if owner.0 == Player::One {
                (&mut p1, &mut p2)
            } else {
                (&mut p2, &mut p1)
            };

            handle_hit(
                &mut commands,
                clock.frame,
                &mut sounds,
                &mut particles,
                effect,
                &mut hit_tracker,
                hitbox.with_offset(hitbox_tf.translation.truncate()),
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
    frame: usize,
    sounds: &mut Option<ResMut<Sounds>>,
    particles: &mut Option<ResMut<Particles>>,
    effect: &OnHitEffect,
    hit_tracker: &mut HitTracker,
    hitbox: Area,
    hitbox_entity: Entity,
    attacker: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
    defender: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
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
        // Handle blocking and state transitions here
        attacker.state.register_hit();

        let blocked = defender.state.blocked(
            effect.fixed_height,
            hitbox,
            defender.character.low_block_height,
            defender.character.high_block_height,
            defender.parser.get_relative_stick_position(),
        );

        // Damage and meter gain
        let amount = effect.damage.get(blocked);
        defender.health.apply_damage(amount);
        attacker.resources.meter.add_combo_meter(amount);

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
            defender.state.stun(effect.stun.get(blocked) + frame);
        }

        // Sound effect
        if let Some(ref mut sounds) = sounds {
            sounds.play(if blocked {
                SoundEffect::Block
            } else {
                SoundEffect::Hit
            });
        }

        // Visual effect
        if let Some(ref mut particles) = particles {
            particles.spawn(ParticleRequest {
                effect: if blocked {
                    VisualEffect::Block
                } else {
                    VisualEffect::Hit
                },
                // TODO: This can be refined more
                position: overlap.center().extend(0.0),
            });
        }

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

pub(super) fn handle_grabs(
    mut commands: Commands,
    mut query: Query<(
        &mut Grabable,
        &mut PlayerState,
        &mut HitboxSpawner,
        &mut PlayerVelocity,
        &mut Health,
        &Facing,
    )>,
) {
    for (mut grab_target, mut state, mut spawner, mut velocity, mut health, &facing) in
        query.iter_mut()
    {
        for descriptor in grab_target.queue.drain(..).collect::<Vec<_>>().into_iter() {
            state.throw();
            spawner.despawn_on_hit(&mut commands);
            // Facing is from the one being thrown, but we want to write the vector from the attacker's perspective
            velocity.add_impulse(facing.opposite().mirror_vec(descriptor.impulse));
            health.apply_damage(descriptor.damage);
        }
    }
}
