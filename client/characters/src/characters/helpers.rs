use crate::{
    moves::{CancelCategory, CancelPolicy, FlowControl::*, Movement},
    Move,
};
use bevy::prelude::*;
use wag_core::{Animation, DummyAnimation};

pub fn jump(input: &'static str, impulse: Vec2) -> Move {
    Move::grounded(
        Some(input),
        CancelCategory::Jump,
        vec![
            Animation::Dummy(DummyAnimation::Jump).into(),
            Wait(5, CancelPolicy::never()),
            Movement::impulse(impulse).into(),
            Wait(5, CancelPolicy::never()),
        ],
    )
}

pub fn dash(input: &'static str, duration: usize, impulse: f32, animation: Animation) -> Move {
    Move::grounded(
        Some(input),
        CancelCategory::Dash,
        vec![
            animation.into(),
            Wait(5, CancelPolicy::never()),
            Movement::impulse(Vec2::X * impulse).into(),
            Wait(duration - 5, CancelPolicy::any()),
        ],
    )
}
