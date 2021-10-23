use bevy::prelude::*;

use input_parsing::InputReader;
use types::StickPosition;

use super::PlayerState;
use crate::physics::PhysicsObject;

pub use moves::universal::{DASH_BACK, DASH_FORWARD};

pub fn movement(mut query: Query<(&mut PhysicsObject, &mut InputReader, &mut PlayerState)>) {
    for (mut physics_object, mut reader, mut state) in query.iter_mut() {
        if reader.is_active() {
            let run_speed = constants::PLAYER_INITIAL_RUN_SPEED.max(
                constants::PLAYER_TOP_SPEED
                    .min(physics_object.velocity.x.abs() + constants::PLAYER_ACCELERATION),
            );

            let events = reader.get_events();

            let change = if state.can_act() && state.is_grounded() {
                if events.contains(&DASH_FORWARD) {
                    reader.consume_event(&DASH_FORWARD);
                    Some(state.forward() * constants::PLAYER_DASH_SPEED)
                } else if events.contains(&DASH_BACK) {
                    reader.consume_event(&DASH_BACK);
                    Some(-state.forward() * constants::PLAYER_DASH_SPEED)
                } else {
                    // Basic movement
                    match reader.get_absolute_stick_position() {
                        StickPosition::E => move_right(run_speed, state.flipped()),
                        StickPosition::W => move_left(run_speed, state.flipped()),
                        StickPosition::N => jump(&mut state, constants::PLAYER_JUMP_VECTOR),
                        StickPosition::NW => jump(&mut state, constants::PLAYER_LEFT_JUMP_VECTOR),
                        StickPosition::NE => jump(&mut state, constants::PLAYER_RIGHT_JUMP_VECTOR),
                        _ => None,
                    }
                }
            } else {
                None
            };
            physics_object.desired_velocity = change;
        }
    }
}

fn move_right(run_speed: f32, flipped: bool) -> Option<Vec3> {
    Some(Vec3::new(
        if flipped {
            constants::PLAYER_WALK_SPEED
        } else {
            run_speed
        },
        0.0,
        0.0,
    ))
}

fn move_left(run_speed: f32, flipped: bool) -> Option<Vec3> {
    Some(Vec3::new(
        if flipped {
            -run_speed
        } else {
            -constants::PLAYER_WALK_SPEED
        },
        0.0,
        0.0,
    ))
}

fn jump(state: &mut PlayerState, direction: (f32, f32, f32)) -> Option<Vec3> {
    state.jump();
    Some(direction.into())
}
