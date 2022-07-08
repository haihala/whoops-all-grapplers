use bevy::{
    ecs::query::{Fetch, WorldQuery},
    prelude::*,
};

use input_parsing::InputParser;
use kits::{Grabable, Hitbox, Hurtbox, Kit, OnHitEffect, Resources};
use player_state::PlayerState;
use time::Clock;
use types::{Area, Facing, Owner, Player, Players, SoundEffect, VisualEffect};

use crate::{
    assets::{ParticleRequest, Particles, Sounds},
    physics::PlayerVelocity,
    spawner::Spawner,
};

use super::Health;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PlayerQuery<'a> {
    kit: &'a Kit,
    hurtbox: &'a Hurtbox,
    tf: &'a Transform,
    health: &'a mut Health,
    resources: &'a mut Resources,
    player: &'a Player,
    parser: &'a InputParser,
    state: &'a mut PlayerState,
    velocity: &'a mut PlayerVelocity,
    facing: &'a Facing,
    spawner: &'a mut Spawner,
}

pub fn register_hits(
    mut commands: Commands,
    clock: Res<Clock>,
    mut sounds: ResMut<Sounds>,
    mut particles: ResMut<Particles>,
    mut hitboxes: Query<(&Owner, &OnHitEffect, &GlobalTransform, &Hitbox)>,
    mut hurtboxes: Query<PlayerQuery>,
    players: Res<Players>,
) {
    for (owner, effect, hitbox_tf, hitbox) in hitboxes.iter_mut() {
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
                hitbox.with_offset(hitbox_tf.translation.truncate()),
                attacker,
                defender,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_hit(
    commands: &mut Commands,
    frame: usize,
    sounds: &mut ResMut<Sounds>,
    particles: &mut ResMut<Particles>,
    effect: &OnHitEffect,
    hitbox: Area,
    attacker: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
    defender: &mut <<PlayerQuery as WorldQuery>::Fetch as Fetch>::Item,
) {
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
            defender.kit.low_block_height,
            defender.kit.high_block_height,
            defender.parser.get_relative_stick_position(),
        );

        // Damage and meter gain
        if let Some(damage_prop) = effect.damage {
            let amount = damage_prop.get(blocked);
            defender.health.apply_damage(amount);
            attacker.resources.meter.add_combo_meter(amount);
        }

        // Knockback
        let knockback_impulse = effect
            .knockback
            // Knockback is positive aka away from attacker, so defender must flip it the other way
            .map(|knockback_prop| {
                defender
                    .facing
                    .opposite()
                    .mirror_vec(knockback_prop.get(blocked))
            })
            .unwrap_or_default();
        defender.velocity.add_impulse(knockback_impulse);

        // Pushback
        if let Some(pushback_prop) = effect.pushback {
            attacker
                .velocity
                // More intuitive to think of it from the defenders perspective
                .add_impulse(defender.facing.mirror_vec(pushback_prop.get(blocked)));
        }

        // Stun
        if let Some(stun_prop) = effect.stun {
            if knockback_impulse.y > 0.0 {
                defender.state.launch();
            } else {
                defender.state.stun(stun_prop.get(blocked) + frame);
            }
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

        // Despawns
        defender.spawner.despawn_on_hit(commands);
        attacker.spawner.despawn_for_move(commands, effect.id);
    }
}

pub fn handle_grabs(
    mut commands: Commands,
    mut query: Query<(
        &mut Grabable,
        &mut PlayerState,
        &mut Spawner,
        &mut PlayerVelocity,
        &mut Health,
    )>,
) {
    for (mut grab_target, mut state, mut spawner, mut velocity, mut health) in query.iter_mut() {
        for descriptor in grab_target.queue.drain(..).collect::<Vec<_>>().into_iter() {
            state.throw();
            spawner.despawn_on_hit(&mut commands);
            velocity.add_impulse(descriptor.impulse);
            health.apply_damage(descriptor.damage);
        }
    }
}
