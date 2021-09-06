use bevy::prelude::*;
use num::clamp;

use crate::player::PlayerState;

pub struct PhysicsObject {
    pub velocity: Vec3,
}
impl Default for PhysicsObject {
    fn default() -> Self {
        Self {
            velocity: Default::default(),
        }
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(gravity.system()).add_system(tick.system());
    }
}

fn gravity(mut query: Query<&mut PhysicsObject>, time: Res<Time>) {
    for mut object in query.iter_mut() {
        object.velocity.y -= crate::constants::PLAYER_GRAVITY * time.delta_seconds();
    }
}

fn tick(mut query: Query<(&mut PhysicsObject, &mut Transform, &mut PlayerState)>, time: Res<Time>) {
    for (mut physics_object, mut transform, mut player) in query.iter_mut() {
        let drag = if player.decelerating {
            if player.grounded {
                crate::constants::GROUND_DRAG * time.delta_seconds()
            } else {
                crate::constants::AIR_DRAG * time.delta_seconds()
            }
        } else {
            0.0
        };

        let speed = clamp(physics_object.velocity.length() - drag, 0.0, f32::MAX);

        physics_object.velocity = physics_object.velocity.normalize_or_zero() * speed;
        physics_object.velocity.x = clamp(
            physics_object.velocity.x,
            -crate::constants::PLAYER_TOP_SPEED,
            crate::constants::PLAYER_TOP_SPEED,
        );

        transform.translation += physics_object.velocity * time.delta_seconds();

        if transform.translation.y < crate::constants::GROUND_PLANE_HEIGHT {
            physics_object.velocity.y = clamp(physics_object.velocity.y, 0.0, f32::MAX);
            transform.translation.y = crate::constants::GROUND_PLANE_HEIGHT;
            player.grounded = true;
        } else if transform.translation.y > crate::constants::GROUND_PLANE_HEIGHT {
            player.grounded = false;
        }

        if transform.translation.x > crate::constants::ARENA_WIDTH {
            physics_object.velocity.x = 0.0;
            transform.translation.x = crate::constants::ARENA_WIDTH;
        } else if transform.translation.x < -crate::constants::ARENA_WIDTH {
            physics_object.velocity.x = 0.0;
            transform.translation.x = -crate::constants::ARENA_WIDTH;
        }
    }
}
