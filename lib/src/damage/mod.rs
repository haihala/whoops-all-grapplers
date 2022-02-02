use bevy::prelude::*;

use input_parsing::InputParser;
use player_state::PlayerState;
use time::Clock;
use types::{Hurtbox, LRDirection, OnHitEffect, Player};

mod health;
pub use health::Health;

use crate::{
    meter::Meter,
    physics::{rect_collision, PlayerVelocity},
    spawner::Spawner,
};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(register_hits).add_system(health::check_dead);
    }
}

#[allow(clippy::type_complexity)]
pub fn register_hits(
    mut commands: Commands,
    clock: Res<Clock>,
    mut hitboxes: Query<(&OnHitEffect, &GlobalTransform, &Sprite)>,
    mut hurtboxes: Query<(
        &Hurtbox,
        &Sprite,
        &GlobalTransform,
        &mut Health,
        &mut Meter,
        &Player,
        &InputParser,
        &mut PlayerState,
        &mut PlayerVelocity,
        &LRDirection,
        &mut Spawner,
    )>,
) {
    for (effect, tf1, hitbox_sprite) in hitboxes.iter_mut() {
        // TODO: May have a problem with item_combinations_mut not giving combinations both ways.
        let mut players = hurtboxes.iter_combinations_mut();
        while let Some(
            [(
                hurtbox,
                hurtbox_sprite,
                tf2,
                mut health,
                _,
                defending_player,
                parser,
                mut state,
                mut defender_velocity,
                facing,
                mut defender_spawner,
            ), (
                _,
                _,
                _,
                _,
                mut attacker_meter,
                _,
                _,
                _,
                mut attacker_velocity,
                _,
                mut attacker_spawner,
            )],
        ) = players.fetch_next()
        {
            if effect.owner == *defending_player {
                // You can't hit yourself
                continue;
            }

            if rect_collision(
                tf2.translation + hurtbox.offset,
                hurtbox_sprite.custom_size.unwrap(),
                tf1.translation,
                hitbox_sprite.custom_size.unwrap(),
            ) {
                // Hit has happened
                // Handle blocking and state transitions here

                let blocked = state.blocked(
                    effect.fixed_height,
                    tf1.translation.y + hitbox_sprite.custom_size.unwrap().y,
                    parser.get_relative_stick_position(),
                );

                // Damage and meter gain
                if let Some(damage_prop) = effect.damage {
                    let amount = damage_prop.get(blocked);
                    health.apply_damage(amount);
                    attacker_meter.add_combo_meter(amount);
                }

                // Knockback
                let knockback_impulse = effect
                    .knockback
                    // Knockback is positive aka away from attacker, so defender must flip it the other way
                    .map(|knockback_prop| facing.opposite().mirror_vec(knockback_prop.get(blocked)))
                    .unwrap_or_default();
                defender_velocity.add_impulse(knockback_impulse);

                // Pushback
                if let Some(pushback_prop) = effect.pushback {
                    attacker_velocity.add_impulse(facing.mirror_vec(pushback_prop.get(blocked)));
                }

                // Stun
                if let Some(stun_prop) = effect.stun {
                    if knockback_impulse.y > 0.0 {
                        state.launch();
                    } else {
                        state.stun(stun_prop.get(blocked) + clock.frame);
                    }
                }

                // Despawns
                defender_spawner.despawn_on_hit(&mut commands);
                attacker_spawner.despawn(&mut commands, vec![effect.id]);
            }
        }
    }
}
