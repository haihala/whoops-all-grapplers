use crate::{
    moves::{CancelPolicy::*, FlowControl::*, MoveType::*, Movement},
    Move,
};
use bevy::prelude::*;
use wag_core::{Animation, DummyAnimation};

pub fn jump(input: &'static str, impulse: Vec2) -> Move {
    Move {
        input: Some(input),
        phases: vec![
            Animation::Dummy(DummyAnimation::Jump).into(),
            Wait(5, Never),
            Movement::impulse(impulse).into(),
            Wait(5, Never),
        ],
        ..default()
    }
}

pub fn dash(input: &'static str, duration: usize, impulse: f32, animation: Animation) -> Move {
    Move {
        input: Some(input),
        move_type: Special,
        phases: vec![
            animation.into(),
            Wait(5, Never),
            Movement::impulse(Vec2::X * impulse).into(),
            Wait(duration - 5, Always),
        ],
        ..default()
    }
}
