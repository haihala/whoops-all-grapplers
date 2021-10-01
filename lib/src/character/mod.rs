mod ryan;
use std::time::Instant;

pub use ryan::{register_ryan_moves, ryan_executor, Ryan};

use crate::{physics::PhysicsObject, player::PlayerState};

use bevy::prelude::*;
use input_parsing::{InputReader, StickPosition};

pub fn movement_executor(mut query: Query<(&mut PhysicsObject, &InputReader, &PlayerState)>) {
    for (mut physics_object, reader, state) in query.iter_mut() {
        let run_speed = crate::PLAYER_INITIAL_RUN_SPEED.max(
            crate::PLAYER_TOP_SPEED
                .min(physics_object.velocity.x.abs() + crate::PLAYER_ACCELERATION),
        );

        let change = match reader.get_stick_position() {
            StickPosition::E => move_right(run_speed, state),
            StickPosition::W => move_left(run_speed, state),
            StickPosition::N => neutral_jump(state),
            StickPosition::NW => left_jump(state),
            StickPosition::NE => right_jump(state),
            _ => None,
        };

        physics_object.desired_velocity = change;
    }
}

fn move_right(run_speed: f32, state: &PlayerState) -> Option<Vec3> {
    if state.grounded {
        Some(Vec3::new(
            if state.flipped {
                crate::PLAYER_WALK_SPEED
            } else {
                run_speed
            },
            0.0,
            0.0,
        ))
    } else {
        None
    }
}

fn move_left(run_speed: f32, state: &PlayerState) -> Option<Vec3> {
    if state.grounded {
        Some(Vec3::new(
            if state.flipped {
                -run_speed
            } else {
                -crate::PLAYER_WALK_SPEED
            },
            0.0,
            0.0,
        ))
    } else {
        None
    }
}
fn neutral_jump(state: &PlayerState) -> Option<Vec3> {
    if state.grounded {
        dbg!("Jump");
        dbg!(Instant::now());

        Some(crate::PLAYER_JUMP_VECTOR.into())
    } else {
        None
    }
}
fn right_jump(state: &PlayerState) -> Option<Vec3> {
    if state.grounded {
        Some(crate::PLAYER_RIGHT_JUMP_VECTOR.into())
    } else {
        None
    }
}
fn left_jump(state: &PlayerState) -> Option<Vec3> {
    if state.grounded {
        Some(crate::PLAYER_LEFT_JUMP_VECTOR.into())
    } else {
        None
    }
}
