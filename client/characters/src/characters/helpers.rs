use std::f32::consts::PI;

use crate::{Action, ActionBlock, CancelCategory, CancelPolicy, ContinuationRequirement, Movement};

use bevy::prelude::*;
use wag_core::{ActionId, Animation};

pub fn jumps(
    height: f32,
    duration: f32,
    animation: Animation,
) -> (impl Iterator<Item = (ActionId, Action)>, f32) {
    /*
    // Math for gravity
    x = x0 + v0*t + 1/2*a*t^2

    From the apex down
    x0 = jump height,
    x = 0
    v0 = 0

    0 = -h + 1/2*a*t^2
    1/2*a*t^2 = h
    a = 2*h/t^2
    */
    let gravity_force: f32 = 2.0 * height / (duration / 2.0).powf(2.0);
    let gravity_per_frame: f32 = gravity_force / wag_core::FPS;

    /*
    Math for initial jump velocity
    x = x0 + v0*t + 1/2*a*t^2
    From start to end

    x0 = 0
    x = 0
    t and a = known, solve v0

    0 = v0*t + 1/2*a*t^2
    v0 = -1/2*a*t
    */
    let neutral_jump_y: f32 = 0.5 * gravity_force * duration;

    const DIAGONAL_JUMP_ANGLE: f32 = 60.0 * PI / 180.0;
    let diagonal_jump_x: f32 = neutral_jump_y * DIAGONAL_JUMP_ANGLE.cos();
    let diagonal_jump_y: f32 = neutral_jump_y * DIAGONAL_JUMP_ANGLE.sin();

    const SUPERJUMP_HEIGHT_MULTIPLIER: f32 = 1.2;
    let neutral_superjump_y: f32 = SUPERJUMP_HEIGHT_MULTIPLIER * neutral_jump_y;
    let diagonal_superjump_x: f32 = SUPERJUMP_HEIGHT_MULTIPLIER * diagonal_jump_x;
    let diagonal_superjump_y: f32 = SUPERJUMP_HEIGHT_MULTIPLIER * diagonal_jump_y;

    let jumps = vec![
        (
            ActionId::BackJump,
            jump("7", Vec2::new(-diagonal_jump_x, diagonal_jump_y), animation),
        ),
        (
            ActionId::NeutralJump,
            jump("8", Vec2::Y * neutral_jump_y, animation),
        ),
        (
            ActionId::ForwardJump,
            jump("9", Vec2::new(diagonal_jump_x, diagonal_jump_y), animation),
        ),
        (
            ActionId::BackSuperJump,
            jump(
                "[123]7",
                Vec2::new(-diagonal_superjump_x, diagonal_superjump_y),
                animation,
            ),
        ),
        (
            ActionId::NeutralSuperJump,
            jump("[123]8", Vec2::Y * neutral_superjump_y, animation),
        ),
        (
            ActionId::ForwardSuperJump,
            jump(
                "[123]9",
                Vec2::new(diagonal_superjump_x, diagonal_superjump_y),
                animation,
            ),
        ),
    ]
    .into_iter();

    (jumps, gravity_per_frame)
}

fn jump(input: &'static str, impulse: Vec2, animation: impl Into<Animation>) -> Action {
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
