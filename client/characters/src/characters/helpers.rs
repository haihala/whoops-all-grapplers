use std::f32::consts::PI;

use crate::{
    Action, ActionBlock, ActionEvent, ActionRequirement, AnimationRequest, CancelCategory,
    CancelPolicy, ContinuationRequirement, FlashRequest, Movement, ResourceType,
};

use bevy::prelude::*;
use wag_core::{
    ActionId, Animation, ItemId, StatusCondition, StatusFlag, TRACK_SPIKES_FLASH_COLOR,
};

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
            jump(
                "7",
                Vec2::new(-diagonal_jump_x, diagonal_jump_y),
                animation,
                false,
            ),
        ),
        (
            ActionId::NeutralJump,
            jump("8", Vec2::Y * neutral_jump_y, animation, false),
        ),
        (
            ActionId::ForwardJump,
            jump(
                "9",
                Vec2::new(diagonal_jump_x, diagonal_jump_y),
                animation,
                false,
            ),
        ),
        (
            ActionId::BackSuperJump,
            jump(
                "[123]7",
                Vec2::new(-diagonal_superjump_x, diagonal_superjump_y),
                animation,
                false,
            ),
        ),
        (
            ActionId::NeutralSuperJump,
            jump("[123]8", Vec2::Y * neutral_superjump_y, animation, false),
        ),
        (
            ActionId::ForwardSuperJump,
            jump(
                "[123]9",
                Vec2::new(diagonal_superjump_x, diagonal_superjump_y),
                animation,
                false,
            ),
        ),
        (
            ActionId::BackAirJump,
            jump(
                "[123456]7",
                Vec2::new(-diagonal_jump_x, diagonal_jump_y),
                animation,
                true,
            ),
        ),
        (
            ActionId::NeutralAirJump,
            jump("[123456]8", Vec2::Y * neutral_jump_y, animation, true),
        ),
        (
            ActionId::ForwardAirJump,
            jump(
                "[123456]9",
                Vec2::new(diagonal_jump_x, diagonal_jump_y),
                animation,
                true,
            ),
        ),
    ]
    .into_iter();

    (jumps, gravity_per_frame)
}

fn jump(
    input: &'static str,
    impulse: Vec2,
    animation: impl Into<Animation> + Clone,
    air_jump: bool,
) -> Action {
    let (initial_events, initial_exit_requirement, requirements, impulse_modifier) = if air_jump {
        (
            vec![
                AnimationRequest {
                    animation: animation.into(),
                    time_offset: 3,
                    ..default()
                }
                .into(),
                ActionEvent::ClearMovement,
                ActionEvent::Condition(StatusCondition {
                    flag: StatusFlag::DoubleJumped,
                    ..default()
                }),
            ],
            ContinuationRequirement::None,
            vec![
                ActionRequirement::Airborne,
                ActionRequirement::ItemsOwned(vec![ItemId::WingedBoots]),
                ActionRequirement::StatusNotActive(StatusFlag::DoubleJumped),
            ],
            0.7,
        )
    } else {
        (
            vec![animation.into().into()],
            ContinuationRequirement::Time(3),
            vec![ActionRequirement::Grounded],
            1.0,
        )
    };

    Action::new(
        Some(input),
        CancelCategory::Jump,
        vec![
            ActionBlock {
                events: initial_events,
                exit_requirement: initial_exit_requirement,
                cancel_policy: CancelPolicy::any(),
                mutator: None,
            },
            ActionBlock {
                events: vec![Movement::impulse(impulse * impulse_modifier).into()],
                exit_requirement: ContinuationRequirement::Time(5),
                cancel_policy: CancelPolicy::any(),
                mutator: Some(|mut original, situation| {
                    original.events = original
                        .events
                        .into_iter()
                        .map(|event| match event {
                            ActionEvent::Movement(base_jump) => ActionEvent::Movement(Movement {
                                amount: base_jump.amount * situation.stats.jump_force_multiplier,
                                ..base_jump
                            }),
                            other => other,
                        })
                        .collect();

                    original
                }),
            },
        ],
        requirements,
    )
}

// TODO: Values should come in as parameters
pub fn dashes(
    forward_animation: impl Into<Animation> + Clone,
    back_animation: impl Into<Animation> + Clone,
) -> impl Iterator<Item = (ActionId, Action)> {
    vec![
        (
            ActionId::DashForward,
            dash("5656", 4, 19, 7.0, forward_animation.clone(), false, false),
        ),
        (
            ActionId::DashBack,
            dash("5454", 5, 20, -7.0, back_animation.clone(), true, false),
        ),
        (
            ActionId::TrackSpikesDashForward,
            dash("5656", 4, 19, 7.0, forward_animation, false, true),
        ),
        (
            ActionId::TrackSpikesDashBack,
            dash("5454", 5, 20, -7.0, back_animation, true, true),
        ),
    ]
    .into_iter()
}

fn dash(
    input: &'static str,
    startup_duration: usize,
    total_duration: usize,
    impulse: f32,
    animation: impl Into<Animation>,
    backdash: bool,
    track_spikes: bool,
) -> Action {
    let mut initial_events = vec![animation.into().into()];
    let mut requirements = vec![ActionRequirement::Grounded];

    if track_spikes {
        initial_events.extend(vec![
            ActionEvent::ModifyResource(ResourceType::Meter, -50),
            ActionEvent::Flash(FlashRequest {
                color: TRACK_SPIKES_FLASH_COLOR,
                ..default()
            }),
        ]);

        requirements.extend(vec![
            ActionRequirement::ResourceValue(ResourceType::Meter, 50),
            ActionRequirement::ItemsOwned(vec![ItemId::TrackSpikes]),
        ]);
    }

    Action::new(
        Some(input),
        if track_spikes {
            CancelCategory::Special
        } else {
            CancelCategory::Dash
        },
        vec![
            ActionBlock {
                events: initial_events,
                exit_requirement: ContinuationRequirement::Time(startup_duration),
                cancel_policy: CancelPolicy::never(),
                mutator: if backdash {
                    Some(|mut original, situation| {
                        if situation.stats.backdash_invuln > 0 {
                            original
                                .events
                                .push(ActionEvent::Condition(StatusCondition {
                                    flag: StatusFlag::Intangible,
                                    effect: None,
                                    // There should probably be a cap here
                                    expiration: Some(situation.stats.backdash_invuln as usize),
                                }));
                        }
                        original
                    })
                } else {
                    None
                },
            },
            ActionBlock {
                events: vec![Movement::impulse(Vec2::X * impulse).into()],
                exit_requirement: ContinuationRequirement::Time(total_duration - startup_duration),
                cancel_policy: CancelPolicy::any(),
                mutator: None,
            },
        ],
        requirements,
    )
}
