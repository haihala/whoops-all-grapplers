use crate::{
    moves::{Action, FlowControl, MoveType, Situation},
    Move,
};
use bevy::prelude::*;

pub fn jump(input: &'static str, impulse: Vec2) -> Move {
    Move {
        input: Some(input),
        move_type: MoveType::Normal,
        can_start: |situation: Situation| situation.grounded,
        phases: vec![FlowControl::Wait(5, false), Action::Impulse(impulse).into()],
    }
}

pub fn dash(input: &'static str, duration: usize, impulse: f32) -> Move {
    Move {
        input: Some(input),
        move_type: MoveType::Special,
        can_start: |situation: Situation| situation.grounded,
        phases: vec![
            FlowControl::Wait(5, false),
            Action::Impulse(Vec2::X * impulse).into(),
            FlowControl::Wait(duration - 5, true),
        ],
    }
}
