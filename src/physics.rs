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

fn gravity(mut query: Query<(&mut PhysicsObject, &PlayerState)>, time: Res<Time>) {
    for (mut object, state) in query.iter_mut() {
        if !state.grounded {
            object.velocity.y -= crate::constants::PLAYER_GRAVITY * time.delta_seconds();
        }
    }
}

fn tick(mut query: Query<(&mut PhysicsObject, &mut Transform, &mut PlayerState)>, time: Res<Time>) {
    for (mut object, mut transform, mut player) in query.iter_mut() {
        let drag = if player.grounded {
            crate::constants::GROUND_DRAG
        } else {
            crate::constants::AIR_DRAG
        };

        let mut speed = object.velocity.length() - drag;
        if speed < crate::constants::PLAYER_MIN_SPEED {
            speed = 0.0;
        }

        object.velocity = object.velocity.normalize_or_zero() * speed;

        transform.translation += object.velocity * time.delta_seconds();

        if transform.translation.y < crate::constants::GROUND_PLANE_HEIGHT {
            object.velocity.y = clamp(object.velocity.y, 0.0, f32::MAX);
            transform.translation.y = crate::constants::GROUND_PLANE_HEIGHT;
            player.grounded = true;
        } else if transform.translation.y > crate::constants::GROUND_PLANE_HEIGHT {
            player.grounded = false;
        }

        if transform.translation.x > crate::constants::ARENA_WIDTH {
            object.velocity.x = 0.0;
            transform.translation.x = crate::constants::ARENA_WIDTH;
        } else if transform.translation.x < -crate::constants::ARENA_WIDTH {
            object.velocity.x = 0.0;
            transform.translation.x = -crate::constants::ARENA_WIDTH;
        }
    }
}
