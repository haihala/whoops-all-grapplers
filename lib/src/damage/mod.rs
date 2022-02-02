use bevy::{prelude::*, utils::HashMap};
use input_parsing::InputParser;
use moves::CancelLevel;
use player_state::{PlayerState, StateEvent};
use types::{
    GrabDescription, Grabable, HeightWindow, Hurtbox, LRDirection, Player, OnHitEffect,
};

mod health;
pub use health::Health;

use crate::{
    clock::Clock,
    physics::{rect_collision, PlayerVelocity},
    spawner::Spawner,
};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(health::refill_meter.system())
            .add_system(register_hits.system())
            .add_system(throwing.system());
    }
}

#[allow(clippy::type_complexity)]
pub fn register_hits(
    mut commands: Commands,
    clock: Res<Clock>,
    mut hitboxes: Query<(Entity, &OnHitEffect, &GlobalTransform, &Sprite)>,
    mut hurtboxes: Query<(
        &Hurtbox,
        &Sprite,
        &GlobalTransform,
        &mut Health,
        &Player,
        &InputParser,
        &mut PlayerState,
        &mut PlayerVelocity,
        &LRDirection,
        &mut Spawner,
    )>,
) {
    // TODO: When migrated to bevy 0.6, use the permutations tool
    let mut pushbacks: HashMap<Player, Vec3> = vec![].into_iter().collect();

    for (entity, effect, tf1, hitbox_sprite) in hitboxes.iter_mut() {
        for (
            hurtbox,
            hurtbox_sprite,
            tf2,
            mut health,
            defending_player,
            parser,
            mut state,
            mut velocity,
            facing,
            mut spawner,
        ) in hurtboxes.iter_mut()
        {
            if effect.owner == *defending_player {
                // You can't hit yourself
                continue;
            }

            if rect_collision(
                tf2.translation + hurtbox.offset,
                hurtbox_sprite.size,
                tf1.translation,
                hitbox_sprite.size,
            ) {
                // Hit has happened
                // Handle blocking and state transitions here
                let top = tf1.translation.y + hitbox_sprite.size.y;
                let bottom = tf1.translation.y - hitbox_sprite.size.y;

                let blocked = state.blocked(
                    effect.fixed_height,
                    HeightWindow { top, bottom },
                    parser.get_relative_stick_position(),
                );

                // Damage
                if let Some(damage_prop) = effect.damage {
                    health.apply_damage(damage_prop.get(blocked));
                }

                // Knockback
                let knockback_impulse = effect
                    .knockback
                    // Knockback is positive aka away from attacker, so defender must flip it the other way
                    .map(|knockback_prop| facing.opposite().mirror_vec(knockback_prop.get(blocked)))
                    .unwrap_or_default();
                velocity.add_impulse(knockback_impulse);

                // Stun
                if let Some(stun_prop) = effect.stun {
                    if knockback_impulse.y > 0.0 {
                        state.launch();
                    } else {
                        state.hit(stun_prop.get(blocked) + clock.frame);
                    }
                }

                if let Some(pushback_prop) = effect.pushback {
                    pushbacks.insert(effect.owner, facing.mirror_vec(pushback_prop.get(blocked)));
                }

                // Despawn entities on hit
                spawner.despawn_on_hit(&mut commands);
                commands.entity(entity).despawn()
            }
        }
    }

    for (_, _, _, _, player, _, _, mut velocity, _, _) in hurtboxes.iter_mut() {
        if let Some(pushback_impulse) = pushbacks.get(player) {
            velocity.add_impulse(pushback_impulse.to_owned());
        }
    }
}

#[allow(clippy::type_complexity)]
fn throwing(
    mut query: QuerySet<(
        Query<(&Transform, &Player, &mut PlayerState)>, // Thrower
        Query<(
            // Target
            &Transform,
            &Player,
            &Grabable,
            &InputParser,
            &mut PlayerState,
            &mut PlayerVelocity,
            &mut Health,
        )>,
    )>,
) {
    let mut throws: HashMap<Player, (GrabDescription, Vec3)> = vec![].into_iter().collect();

    for (tf, player, mut state) in query.q0_mut().iter_mut() {
        for (event, description) in state.get_events().iter().filter_map(|ev| {
            let owned = ev.to_owned();
            if let StateEvent::Grab(description) = owned {
                Some((owned, description))
            } else {
                None
            }
        }) {
            state.consume_event(event);
            assert!(throws
                .insert(player.other(), (description, tf.translation))
                .is_none()); // If this is not none, it means we had two throw events lined up between frames which is a bug
        }
    }

    for (tf, player, throwable, parser, mut state, mut velocity, mut health) in
        query.q1_mut().iter_mut()
    {
        if let Some((description, origin)) = throws.get(player) {
            let distance = (*origin - tf.translation).length();
            let max_distance = throwable.size + description.range;
            let in_range = distance <= max_distance;

            let teched =
                state.cancel_requirement() < CancelLevel::LightNormal && parser.clear_head();

            if in_range && !teched {
                state.throw();
                velocity.add_impulse(description.impulse);
                health.apply_damage(description.damage);
            }
        }
    }
}
