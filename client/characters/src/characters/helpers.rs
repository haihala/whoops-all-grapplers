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
    let (jg, gravity_per_frame) = JumpGenerator::new(animation, height, duration);

    let jumps = vec![
        (ActionId::BackJump, jg.basic(Back)),
        (ActionId::NeutralJump, jg.basic(Neutral)),
        (ActionId::ForwardJump, jg.basic(Forward)),
        (ActionId::BackSuperJump, jg.high(Back)),
        (ActionId::NeutralSuperJump, jg.high(Neutral)),
        (ActionId::ForwardSuperJump, jg.high(Forward)),
        // TODO: Test to see if input parser handles this, probably not
        (ActionId::BackShortHop, jg.short(Back)),
        (ActionId::NeutralShortHop, jg.short(Neutral)),
        (ActionId::ForwardShortHop, jg.short(Forward)),
        (ActionId::BackAirJump, jg.air(Back)),
        (ActionId::NeutralAirJump, jg.air(Neutral)),
        (ActionId::ForwardAirJump, jg.air(Forward)),
    ]
    .into_iter();

    (jumps, gravity_per_frame)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JumpDirection {
    Neutral,
    Forward,
    Back,
}
use JumpDirection::*;
impl JumpDirection {
    fn direction(self) -> Vec2 {
        let diagonal_jump_angle = 60.0 * PI / 180.0;
        let sin = diagonal_jump_angle.sin();
        let cos = diagonal_jump_angle.cos();

        match self {
            JumpDirection::Neutral => Vec2::Y,
            JumpDirection::Forward => Vec2::new(cos, sin),
            JumpDirection::Back => Vec2::new(-cos, sin),
        }
    }

    fn base_input(self) -> &'static str {
        match self {
            JumpDirection::Neutral => "[123456]8",
            JumpDirection::Forward => "[123456]9",
            JumpDirection::Back => "[123456]7",
        }
    }

    fn super_input(self) -> &'static str {
        match self {
            JumpDirection::Neutral => "[123]8",
            JumpDirection::Forward => "[123]9",
            JumpDirection::Back => "[123]7",
        }
    }

    fn short_input(self) -> &'static str {
        match self {
            JumpDirection::Neutral => "[123456]8[123]",
            JumpDirection::Forward => "[123456]9[123]",
            JumpDirection::Back => "[123456]7[123]",
        }
    }
}

struct JumpGenerator {
    animation: Animation,
    base_impulse: f32,
}
impl JumpGenerator {
    fn new(animation: Animation, height: f32, duration: f32) -> (Self, f32) {
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
        let base_impulse = 0.5 * gravity_force * duration;

        (
            Self {
                animation,
                base_impulse,
            },
            gravity_per_frame,
        )
    }

    fn basic(&self, dir: JumpDirection) -> Action {
        jump(
            dir.base_input(),
            dir.direction() * self.base_impulse,
            self.animation,
            false,
        )
    }

    fn high(&self, dir: JumpDirection) -> Action {
        jump(
            dir.super_input(),
            dir.direction() * self.base_impulse * 1.3,
            self.animation,
            false,
        )
    }

    fn short(&self, dir: JumpDirection) -> Action {
        jump(
            dir.short_input(),
            dir.direction() * self.base_impulse * 0.3,
            self.animation,
            false,
        )
    }

    fn air(&self, dir: JumpDirection) -> Action {
        jump(
            dir.base_input(),
            dir.direction() * self.base_impulse * 0.7,
            self.animation,
            true,
        )
    }
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
            vec![
                animation.into().into(),
                // This prevents accidental immediate double jump (odd low jump)
                ActionEvent::Condition(StatusCondition {
                    flag: StatusFlag::DoubleJumped,
                    expiration: Some(20),
                    ..default()
                }),
            ],
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
