use crate::{
    moves::{CancelPolicy, FlowControl, MoveType, Movement},
    Move,
};
use bevy::prelude::*;
use wag_core::{Animation, DummyAnimation};

pub fn jump(input: &'static str, impulse: Vec2) -> Move {
    Move {
        input: Some(input),
        phases: vec![
            Animation::Dummy(DummyAnimation::Jump).into(),
            FlowControl::Wait(5, CancelPolicy::Never),
            Movement::impulse(impulse).into(),
            FlowControl::Wait(5, CancelPolicy::Never),
        ],
        ..default()
    }
}

pub fn dash(input: &'static str, duration: usize, impulse: f32, animation: Animation) -> Move {
    Move {
        input: Some(input),
        move_type: MoveType::Special,
        phases: vec![
            animation.into(),
            FlowControl::Wait(5, CancelPolicy::Never),
            Movement::impulse(Vec2::X * impulse).into(),
            FlowControl::Wait(duration - 5, CancelPolicy::Always),
        ],
        ..default()
    }
}
