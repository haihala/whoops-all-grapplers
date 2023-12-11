use crate::{Action, ActionBlock, CancelCategory, CancelPolicy, ContinuationRequirement, Movement};

use bevy::prelude::*;
use wag_core::Animation;

pub fn jump(input: &'static str, impulse: Vec2, animation: impl Into<Animation>) -> Action {
    Action::grounded(
        Some(input),
        CancelCategory::Jump,
        vec![
            ActionBlock {
                events: vec![animation.into().into()],
                exit_requirement: ContinuationRequirement::Time(3),
                cancel_policy: CancelPolicy::any(),
                mutator: None,
            },
            ActionBlock {
                events: vec![Movement::impulse(impulse).into()],
                exit_requirement: ContinuationRequirement::Time(5),
                cancel_policy: CancelPolicy::any(),
                mutator: None,
            },
        ],
    )
}

pub fn dash(
    input: &'static str,
    duration: usize,
    impulse: f32,
    animation: impl Into<Animation>,
) -> Action {
    Action::grounded(
        Some(input),
        CancelCategory::Dash,
        vec![
            ActionBlock {
                events: vec![animation.into().into()],
                exit_requirement: ContinuationRequirement::Time(5),
                cancel_policy: CancelPolicy::never(),
                mutator: None,
            },
            ActionBlock {
                events: vec![Movement::impulse(Vec2::X * impulse).into()],
                exit_requirement: ContinuationRequirement::Time(duration - 5),
                cancel_policy: CancelPolicy::any(),
                mutator: None,
            },
        ],
    )
}
