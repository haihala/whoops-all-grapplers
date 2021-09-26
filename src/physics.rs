use bevy::prelude::*;
use num::clamp;

use crate::player::{Player, PlayerState};

#[derive(Debug, Default)]
pub struct PhysicsObject {
    pub velocity: Vec3,
    pub ground_speed: f32,
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(gravity.system())
            .add_system(combine_speeds.system())
            .add_system(sideswitcher.system())
            .add_system(tick.system());
    }
}

fn gravity(mut query: Query<&mut PhysicsObject>, time: Res<Time>) {
    for mut object in query.iter_mut() {
        object.velocity.y -= crate::PLAYER_GRAVITY * time.delta_seconds();
    }
}
fn combine_speeds(mut query: Query<(&mut PhysicsObject, &mut PlayerState)>) {
    for (mut physics_object, mut state) in query.iter_mut() {
        state.decelerating = true;
        if state.grounded && physics_object.ground_speed != 0.0 {
            physics_object.velocity.x = physics_object.ground_speed;
            state.decelerating = false;
        }
    }
}

fn sideswitcher(
    mut players: Query<(Entity, &Transform, &mut PlayerState), With<Player>>,
    others: Query<(Entity, &Transform), With<Player>>,
) {
    for (entity, transform, mut player) in players.iter_mut() {
        for (e, tf) in others.iter() {
            if e == entity {
                continue;
            }

            player.flipped = transform.translation.x > tf.translation.x;
        }
    }
}

fn tick(mut query: Query<(&mut PhysicsObject, &mut Transform, &mut PlayerState)>, time: Res<Time>) {
    for (mut physics_object, mut transform, mut player) in query.iter_mut() {
        if player.decelerating {
            let drag = time.delta_seconds()
                * if player.grounded {
                    crate::GROUND_DRAG
                } else {
                    crate::AIR_DRAG
                };

            let speed = (physics_object.velocity.length() - drag).max(0.0);
            physics_object.velocity = physics_object.velocity.normalize_or_zero() * speed;
        };

        transform.translation += physics_object.velocity * time.delta_seconds();

        if transform.translation.y < crate::GROUND_PLANE_HEIGHT {
            physics_object.velocity.y = clamp(physics_object.velocity.y, 0.0, f32::MAX);
            transform.translation.y = crate::GROUND_PLANE_HEIGHT;
            player.grounded = true;
        } else if transform.translation.y > crate::GROUND_PLANE_HEIGHT {
            player.grounded = false;
        }

        if transform.translation.x.abs() > crate::ARENA_WIDTH {
            physics_object.velocity.x = 0.0;
            transform.translation.x =
                transform.translation.x.signum() * crate::ARENA_WIDTH;
        }
    }
}
