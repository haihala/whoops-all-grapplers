use bevy::prelude::*;

use crate::input::special_moves::{MotionMapping, SpecialMoveName};
use crate::input::{ActionButton, InputStore};

pub struct Ryan;

pub struct RyanMoveBuffer(pub Option<RyanAction>, pub Option<super::CharacterAction>);

#[derive(Debug)]
pub enum RyanAction {
    Hadouken,
}

pub fn ryan_parser(
    mut query: Query<
        (
            &InputStore,
            &crate::player::PlayerState,
            &mut RyanMoveBuffer,
        ),
        With<Ryan>,
    >,
    motion_mappings: Res<MotionMapping>,
) {
    for (input_store, state, mut move_buffer) in query.iter_mut() {
        move_buffer.0 = None;
        let quarter_circle = motion_mappings
            .get(&SpecialMoveName::QuarterCircleForward)
            .unwrap();

        if input_store.contains(quarter_circle.requirements(state.flipped))
            && input_store.recently_pressed.contains(&ActionButton::Fast)
        {
            move_buffer.0 = Some(RyanAction::Hadouken);
            continue;
        }

        move_buffer.1 = super::parse_character_action(input_store, state.grounded);
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
            state,
            &mut physics_object,
            time.delta_seconds(),
        );
    }
}
