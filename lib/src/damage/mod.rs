use bevy::{prelude::*, utils::HashMap};
use input_parsing::InputParser;
use player_state::PlayerState;
use types::{
    AttackHeight, Damage, HeightWindow, Hurtbox, Knockback, LRDirection, Player,
    PlayerCollisionTrigger, Pushback, Stun,
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
            .add_system(register_hits.system());
    }
}

#[allow(clippy::type_complexity)]
pub fn register_hits(
    mut commands: Commands,
    clock: Res<Clock>,
    mut hitboxes: Query<(
        Entity,
        &PlayerCollisionTrigger,
        &GlobalTransform,
        &Sprite,
        Option<&Damage>,
        Option<&Stun>,
        Option<&Knockback>,
        Option<&Pushback>,
        Option<&AttackHeight>,
    )>,
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

    for (entity, pct, tf1, hitbox_sprite, damage, stun, knockback, pushback, fixed_height) in
        hitboxes.iter_mut()
    {
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
            if pct.owner == *defending_player {
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
                    fixed_height,
                    HeightWindow { top, bottom },
                    parser.get_relative_stick_position(),
                );

                // Damage
                if let Some(damage_prop) = damage {
                    health.apply_damage(damage_prop.get(blocked));
                }

                // Knockback
                let knockback_impulse = knockback
                    .map(|knockback_prop| facing.mirror_vec(knockback_prop.get(blocked)))
                    .unwrap_or_default();
                velocity.add_impulse(knockback_impulse);

                // Stun
                if let Some(stun_prop) = stun {
                    state.hit(
                        stun_prop.get(blocked) + clock.frame,
                        knockback_impulse.y > 0.0,
                    );
                }

                if let Some(pushback_prop) = pushback {
                    pushbacks.insert(
                        pct.owner.other(),
                        facing.mirror_vec(pushback_prop.get(blocked)),
                    );
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
