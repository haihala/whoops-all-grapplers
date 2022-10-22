use crate::{
    moves::{Action, FlowControl, MoveType, Movement},
    Move,
};
use bevy::prelude::*;
use core::{Animation, DummyAnimation};

pub fn jump(input: &'static str, impulse: Vec2) -> Move {
    Move {
        input: Some(input),
        phases: vec![
            Action::Animation(Animation::Dummy(DummyAnimation::Jump)).into(),
            FlowControl::Wait(5, false),
            Action::Movement(Movement::impulse(impulse)).into(),
            FlowControl::Wait(5, false),
        ],
        ..default()
    }
}

pub fn dash(input: &'static str, duration: usize, impulse: f32, animation: Animation) -> Move {
    Move {
        input: Some(input),
        move_type: MoveType::Special,
        phases: vec![
            Action::Animation(animation).into(),
            FlowControl::Wait(5, false),
            Action::Movement(Movement::impulse(Vec2::X * impulse)).into(),
            FlowControl::Wait(duration - 5, true),
        ],
        ..default()
    }
}
