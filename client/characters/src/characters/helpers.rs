use crate::{Action, ActionBlock, CancelCategory, CancelPolicy, Movement, Requirement};

use bevy::prelude::*;
use wag_core::{Animation, DummyAnimation};

pub fn jump(input: &'static str, impulse: Vec2) -> Action {
    Action::grounded(
        Some(input),
        CancelCategory::Jump,
        vec![
            ActionBlock {
                events: vec![DummyAnimation::Jump.into()],
                exit_requirement: Requirement::Time(5),
                cancel_policy: CancelPolicy::never(),
                mutator: None,
            },
            ActionBlock {
                events: vec![Movement::impulse(impulse).into()],
                exit_requirement: Requirement::Time(5),
                cancel_policy: CancelPolicy::never(),
                mutator: None,
            },
        ],
    )
}

pub fn dash(input: &'static str, duration: usize, impulse: f32, animation: Animation) -> Action {
    Action::grounded(
        Some(input),
        CancelCategory::Dash,
        vec![
            ActionBlock {
                events: vec![animation.into()],
                exit_requirement: Requirement::Time(5),
                cancel_policy: CancelPolicy::never(),
                mutator: None,
            },
            ActionBlock {
                events: vec![Movement::impulse(Vec2::X * impulse).into()],
                exit_requirement: Requirement::Time(duration - 5),
                cancel_policy: CancelPolicy::any(),
                mutator: None,
            },
        ],
    )
}
