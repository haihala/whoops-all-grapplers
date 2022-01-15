use bevy::prelude::*;
use types::{HeightWindow, Hitbox, Hurtbox, Player};

mod health;
pub use health::Health;

use crate::physics::rect_collision;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(health::apply_hits.system())
            .add_system(register_hits.system());
    }
}

pub fn register_hits(
    mut commands: Commands,
    mut hitboxes: Query<(Entity, &Hitbox, &GlobalTransform)>,
    mut hurtboxes: Query<(&Hurtbox, &Sprite, &GlobalTransform, &mut Health, &Player)>,
) {
    for (entity, hitbox, tf1) in hitboxes.iter_mut() {
        for (hurtbox, sprite, tf2, mut health, defending_player) in hurtboxes.iter_mut() {
            if hitbox.owner.unwrap() == *defending_player {
                // You can't hit yourself
                continue;
            }

            if rect_collision(
                tf2.translation + hurtbox.offset,
                sprite.size,
                tf1.translation,
                hitbox.size,
            ) {
                let top = tf1.translation.y + hitbox.size.y;
                let bottom = tf1.translation.y - hitbox.size.y;

                health.hit(hitbox.hit, HeightWindow { top, bottom });
                commands.entity(entity).despawn()
            }
        }
    }
}
