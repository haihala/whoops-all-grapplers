use bevy::prelude::*;

use super::PlayerState;
use crate::physics::PhysicsObject;
use input_parsing::{InputReader, StickPosition};

pub fn movement(mut query: Query<(&mut PhysicsObject, &InputReader, &mut PlayerState)>) {
    for (mut physics_object, reader, mut state) in query.iter_mut() {
        let run_speed = crate::PLAYER_INITIAL_RUN_SPEED.max(
            crate::PLAYER_TOP_SPEED
                .min(physics_object.velocity.x.abs() + crate::PLAYER_ACCELERATION),
        );

        let change = if *state == PlayerState::Standing {
            match reader.get_stick_position() {
                StickPosition::E => move_right(run_speed, reader),
                StickPosition::W => move_left(run_speed, reader),
                StickPosition::N => jump(&mut state, crate::PLAYER_JUMP_VECTOR),
                StickPosition::NW => jump(&mut state, crate::PLAYER_LEFT_JUMP_VECTOR),
                StickPosition::NE => jump(&mut state, crate::PLAYER_RIGHT_JUMP_VECTOR),
                _ => None,
            }
        } else {
            None
        };

        physics_object.desired_velocity = change;
    }
}

fn move_right(run_speed: f32, inputs: &InputReader) -> Option<Vec3> {
    Some(Vec3::new(
        if inputs.flipped {
            crate::PLAYER_WALK_SPEED
        } else {
            run_speed
        },
        0.0,
        0.0,
    ))
}

fn move_left(run_speed: f32, inputs: &InputReader) -> Option<Vec3> {
    Some(Vec3::new(
        if inputs.flipped {
            -run_speed
        } else {
            -crate::PLAYER_WALK_SPEED
        },
        0.0,
        0.0,
    ))
}

fn jump(state: &mut PlayerState, direction: (f32, f32, f32)) -> Option<Vec3> {
    *state = PlayerState::Air;
    Some(direction.into())
}
