use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use input_parsing::InputReader;
use num::clamp;

use crate::player::{Player, PlayerState};

#[derive(Debug, Default, Inspectable)]
pub struct PhysicsObject {
    pub velocity: Vec3,
    pub desired_velocity: Option<Vec3>,
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(gravity.system())
            .add_system(player_drag.system())
            .add_system(incorporate_desired_velocity.system())
            .add_system(sideswitcher.system())
            .add_system(move_objects.system());
    }
}

fn gravity(mut query: Query<&mut PhysicsObject>, time: Res<Time>) {
    for mut object in query.iter_mut() {
        object.velocity.y -= crate::PLAYER_GRAVITY * time.delta_seconds();
    }
}

fn player_drag(mut query: Query<(&mut PhysicsObject, &PlayerState)>, time: Res<Time>) {
    for (mut object, player) in query.iter_mut() {
        let drag = player.drag_multiplier
            * time.delta_seconds()
            * if player.grounded {
                crate::GROUND_DRAG
            } else {
                crate::AIR_DRAG
            };
        let speed = (object.velocity.length() - drag).max(0.0);
        object.velocity = object.velocity.normalize_or_zero() * speed;
    }
}

fn incorporate_desired_velocity(mut query: Query<(&mut PhysicsObject, &mut PlayerState)>) {
    for (mut object, mut state) in query.iter_mut() {
        if let Some(desired) = object.desired_velocity {
            let desired_direction = desired.x.signum();
            let current_direction = object.velocity.x.signum();

            object.velocity.y = desired.y;

            #[allow(clippy::float_cmp)]
            if object.velocity.x == 0.0 || current_direction == desired_direction {
                object.velocity.x =
                    desired_direction * object.velocity.x.abs().max(desired.x.abs());
                state.drag_multiplier = 0.0;
            } else {
                state.drag_multiplier = crate::REVERSE_DRAG_MULTIPLIER;
            }
        } else {
            state.drag_multiplier = 1.0;
        }
    }
}

fn sideswitcher(
    mut players: Query<(Entity, &Transform, &mut PlayerState, &mut InputReader), With<Player>>,
    others: Query<(Entity, &Transform), With<Player>>,
) {
    for (entity, transform, mut player, mut reader) in players.iter_mut() {
        for (e, tf) in others.iter() {
            if e == entity {
                continue;
            }

            let flipped = transform.translation.x > tf.translation.x;
            player.flipped = flipped;
            reader.set_flipped(flipped);
        }
    }
}

fn move_objects(
    mut query: Query<(&mut PhysicsObject, &mut Transform, &mut PlayerState)>,
    time: Res<Time>,
) {
    for (mut object, mut transform, mut state) in query.iter_mut() {
        transform.translation += object.velocity * time.delta_seconds();

        if transform.translation.y < crate::GROUND_PLANE_HEIGHT {
            object.velocity.y = clamp(object.velocity.y, 0.0, f32::MAX);
            transform.translation.y = crate::GROUND_PLANE_HEIGHT;
            state.grounded = true;
        } else if transform.translation.y > crate::GROUND_PLANE_HEIGHT {
            state.grounded = false;
        }

        if transform.translation.x.abs() > crate::ARENA_WIDTH {
            object.velocity.x = 0.0;
            transform.translation.x = transform.translation.x.signum() * crate::ARENA_WIDTH;
        }
    }
}
