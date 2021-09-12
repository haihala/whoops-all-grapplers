use bevy::prelude::*;

use crate::input::{ActionButton, InputBuffer, SpecialMove, StickPosition};

use super::CharacterAction;

pub struct Ryan;

pub struct RyanMoveBuffer(pub Option<RyanAction>, pub Option<super::CharacterAction>);

#[derive(Debug)]
pub enum RyanAction {
    Hadouken,
}

pub fn ryan_parser(
    mut query: Query<
        (
            &InputBuffer,
            &crate::player::PlayerState,
            &mut RyanMoveBuffer,
        ),
        With<Ryan>,
    >,
) {
    for (input_buffer, state, mut move_buffer) in query.iter_mut() {
        move_buffer.0 = None;

        for special in input_buffer.interpreted.iter() {
            match special {
                SpecialMove::QuarterCircle => todo!(),
                SpecialMove::BackwardQuarterCircle => todo!(),
            }
        }

        move_buffer.1 = super::parse_character_action(input_buffer, state.grounded);
    }
}

pub fn ryan_executor(
    mut query: Query<
        (
            &RyanMoveBuffer,
            &crate::player::PlayerState,
            &mut crate::physics::PhysicsObject,
        ),
        With<Ryan>,
    >,
    time: Res<Time>,
) {
    for (move_buffer, state, mut physics_object) in query.iter_mut() {
        if let Some(action) = &move_buffer.0 {
            match action {
                RyanAction::Hadouken => todo!(),
            }
        }

        super::handle_character_action(
            &move_buffer.1,
            &state,
            &mut physics_object,
            time.delta_seconds(),
        );
    }
}
