use crate::{
    moves::{Action, FlowControl, MoveType, Movement, Situation},
    Move,
};
use bevy::prelude::*;

pub fn jump(input: &'static str, impulse: Vec2) -> Move {
    Move {
        input: Some(input),
        move_type: MoveType::Normal,
        requirement: |situation: Situation| situation.grounded,
        phases: vec![
            FlowControl::Wait(5, false),
            Action::Movement(Movement::impulse(impulse)).into(),
        ],
    }
}

pub fn dash(input: &'static str, duration: usize, impulse: f32) -> Move {
    Move {
        input: Some(input),
        move_type: MoveType::Special,
        requirement: |situation: Situation| situation.grounded,
        phases: vec![
            FlowControl::Wait(5, false),
            Action::Movement(Movement::impulse(Vec2::X * impulse)).into(),
            FlowControl::Wait(duration - 5, true),
        ],
    }
}
