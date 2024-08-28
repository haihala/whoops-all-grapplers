use std::f32::consts::PI;

use crate::{
    actions::ActionCategory, Action, ActionBlock, ActionEvent, ActionRequirement, CancelRule,
    ContinuationRequirement, FlashRequest, Movement, ResourceType,
};

use bevy::prelude::*;
use wag_core::{
    ActionId, Animation, ItemId, StatusCondition, StatusFlag, VfxRequest, VisualEffect,
    TRACK_SPIKES_FLASH_COLOR,
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

const DIAGONAL_JUMP_ANGLE: f32 = 70.0;

impl JumpDirection {
    fn base_vec(self) -> Vec2 {
        let diagonal_jump_angle = DIAGONAL_JUMP_ANGLE * PI / 180.0;
        let cos = diagonal_jump_angle.cos();

        Vec2::new(
            match self {
                JumpDirection::Neutral => 0.0,
                JumpDirection::Forward => cos,
                JumpDirection::Back => -cos,
            },
            1.0,
        )
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
            dir.base_vec() * self.base_impulse,
            self.animation,
            false,
            vec![ActionRequirement::Grounded],
        )
    }

    fn high(&self, dir: JumpDirection) -> Action {
        jump(
            dir.super_input(),
            dir.base_vec() * self.base_impulse * 1.2,
            self.animation,
            false,
            vec![
                ActionRequirement::Grounded,
                ActionRequirement::ItemsOwned(vec![ItemId::FeatheredBoots]),
            ],
        )
    }

    fn air(&self, dir: JumpDirection) -> Action {
        jump(
            dir.base_input(),
            dir.base_vec() * self.base_impulse * 0.7,
            self.animation,
            true,
            vec![
                ActionRequirement::Airborne,
                ActionRequirement::ItemsOwned(vec![ItemId::PigeonWing]),
                ActionRequirement::StatusNotActive(StatusFlag::DoubleJumped),
            ],
        )
    }
}

fn jump(
    input: &'static str,
    impulse: Vec2,
    animation: impl Into<Animation> + Clone,
    air_jump: bool,
    requirements: Vec<ActionRequirement>,
) -> Action {
    let (initial_events, initial_exit_requirement) = if air_jump {
        (
            vec![
                animation.into().into(),
                ActionEvent::ClearMovement,
                ActionEvent::Condition(StatusCondition {
                    flag: StatusFlag::DoubleJumped,
                    ..default()
                }),
            ],
            ContinuationRequirement::None,
        )
    } else {
        (
            vec![
                animation.into().into(),
                // This prevents accidental immediate double jump (odd low jump)
                ActionEvent::Condition(StatusCondition {
                    flag: StatusFlag::DoubleJumped,
                    expiration: Some(10),
                    ..default()
                }),
            ],
            ContinuationRequirement::Time(3),
        )
    };

    Action::new(
        Some(input),
        ActionCategory::Jump,
        vec![
            ActionBlock {
                events: initial_events,
                exit_requirement: initial_exit_requirement,
                cancel_policy: CancelRule::jump(),
                mutator: None,
            },
            ActionBlock {
                events: vec![
                    Movement::impulse(impulse).into(),
                    VfxRequest {
                        effect: VisualEffect::SpeedLines,
                        position: Vec3::ZERO,
                        rotation: if impulse.x != 0.0 {
                            Some(-impulse.x)
                        } else {
                            Some(std::f32::consts::PI)
                        },
                    }
                    .into(),
                ],
                exit_requirement: ContinuationRequirement::Time(5),
                cancel_policy: CancelRule::jump(),
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

                    // This has to go here so that it gets the position.
                    for ev in original.events.iter_mut() {
                        if let ActionEvent::VisualEffect(vfx_request) = ev {
                            vfx_request.position = situation.position + Vec3::new(-0.5, 1.3, 0.0);
                        }
                    }

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
            dash(
                "5656",
                6,
                20,
                Vec2::X * 5.0,
                Vec2::new(2.0, 4.0),
                forward_animation.clone(),
                false,
                false,
            ),
        ),
        (
            ActionId::DashBack,
            dash(
                "5454",
                0,
                20,
                Vec2::ZERO,
                Vec2::new(-7.0, 0.0),
                back_animation.clone(),
                true,
                false,
            ),
        ),
        (
            ActionId::TrackSpikesDashForward,
            dash(
                "5656",
                6,
                20,
                Vec2::X * 5.0,
                Vec2::new(2.0, 4.0),
                forward_animation,
                false,
                true,
            ),
        ),
        (
            ActionId::TrackSpikesDashBack,
            dash(
                "5454",
                0,
                20,
                Vec2::ZERO,
                Vec2::new(-7.0, 0.0),
                back_animation,
                true,
                true,
            ),
        ),
    ]
    .into_iter()
}

#[allow(clippy::too_many_arguments)]
fn dash(
    input: &'static str,
    startup_duration: usize,
    total_duration: usize,
    first_impulse: Vec2,
    second_impulse: Vec2,
    animation: impl Into<Animation>,
    backdash: bool,
    track_spikes: bool,
) -> Action {
    let mut initial_events = vec![
        animation.into().into(),
        VfxRequest {
            effect: VisualEffect::SpeedLines,
            position: Vec3::ZERO,
            rotation: None,
        }
        .into(),
    ];

    if first_impulse != Vec2::ZERO {
        initial_events.push(Movement::impulse(first_impulse).into());
    }

    let mut requirements = vec![ActionRequirement::Grounded];

    if track_spikes {
        initial_events.extend(vec![
            ActionEvent::ModifyResource(ResourceType::Meter, -40),
            ActionEvent::Flash(FlashRequest {
                color: TRACK_SPIKES_FLASH_COLOR,
                ..default()
            }),
        ]);

        requirements.extend(vec![
            ActionRequirement::ResourceValue(ResourceType::Meter, 40),
            ActionRequirement::ItemsOwned(vec![ItemId::TrackSpikes]),
            ActionRequirement::AnyActionOngoing,
            ActionRequirement::ActionNotOngoing(vec![
                ActionId::DashForward,
                ActionId::DashBack,
                ActionId::TrackSpikesDashForward,
                ActionId::TrackSpikesDashBack,
            ]),
        ]);
    }

    Action::new(
        Some(input),
        if track_spikes {
            ActionCategory::Super
        } else {
            ActionCategory::Dash
        },
        vec![
            ActionBlock {
                events: initial_events,
                exit_requirement: ContinuationRequirement::Time(startup_duration),
                cancel_policy: CancelRule::never(),
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

                        // This has to go here so that it gets the position.
                        for ev in original.events.iter_mut() {
                            if let ActionEvent::VisualEffect(vfx_request) = ev {
                                vfx_request.position =
                                    situation.position + Vec3::new(-0.5, 1.3, 0.0);
                                let rot =
                                    -situation.facing.to_signum() * std::f32::consts::PI / 2.0;
                                vfx_request.rotation = Some(rot);
                            }
                        }

                        original
                    })
                } else {
                    // This is a bit retarded, but you fn doesn't let you store the bool "backdash"
                    // TODO: Try to get real big boy closures to work with this
                    Some(|mut original, situation| {
                        // This has to go here so that it gets the position.
                        for ev in original.events.iter_mut() {
                            if let ActionEvent::VisualEffect(vfx_request) = ev {
                                vfx_request.position =
                                    situation.position + Vec3::new(-0.5, 1.3, 0.0);
                                let rot =
                                    -situation.facing.to_signum() * std::f32::consts::PI / 2.0;
                                vfx_request.rotation = Some(rot);
                            }
                        }

                        original
                    })
                },
            },
            ActionBlock {
                events: vec![Movement::impulse(second_impulse).into()],
                exit_requirement: ContinuationRequirement::Time(total_duration - startup_duration),
                cancel_policy: if track_spikes {
                    CancelRule::dash()
                } else {
                    CancelRule::never()
                },
                mutator: None,
            },
        ],
        requirements,
    )
}
