use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JumpType {
    Basic,
    Air,
    Super,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JumpDirection {
    Neutral,
    Forward,
    Back,
}

const DIAGONAL_JUMP_ANGLE: f32 = 70.0;

impl JumpDirection {
    pub fn base_vec(self) -> Vec2 {
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

    pub fn input(&self, jump_type: JumpType) -> &'static str {
        if jump_type == JumpType::Super {
            self.super_input()
        } else {
            self.base_input()
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
}

#[macro_export]
macro_rules! jump {
    ($height:expr, $duration:expr, $animation:expr, $dir:expr, $type:expr) => {{
        use wag_core::{StatusCondition, StatusFlag, VfxRequest, VisualEffect};
        use $crate::characters::helpers::{JumpDirection, JumpType};

        let input = $dir.input($type);
        let requirements = match $type {
            JumpType::Basic => vec![ActionRequirement::Grounded],
            JumpType::Air => vec![
                ActionRequirement::Airborne,
                ActionRequirement::ItemsOwned(vec![ItemId::PigeonWing]),
                ActionRequirement::StatusNotActive(StatusFlag::DoubleJumped),
            ],
            JumpType::Super => vec![
                ActionRequirement::Grounded,
                ActionRequirement::ItemsOwned(vec![ItemId::FeatheredBoots]),
            ],
        };

        Action {
            input: Some(input),
            category: ActionCategory::Jump,
            script: Box::new(|situation: &Situation| {
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
                let gravity_force: f32 = 2.0 * $height / (($duration / 2.0) as f32).powf(2.0);

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
                let base_impulse = 0.5 * gravity_force * $duration;
                let impulse = $dir.base_vec()
                    * base_impulse
                    * situation.stats.jump_force_multiplier
                    * match $type {
                        JumpType::Basic => 1.0,
                        JumpType::Air => 0.7,
                        JumpType::Super => 1.2,
                    };

                let mut initial_events = vec![Into::<Animation>::into($animation).into()];

                if $type == JumpType::Air {
                    initial_events.extend(vec![
                        ActionEvent::ClearMovement,
                        ActionEvent::Condition(StatusCondition {
                            flag: StatusFlag::DoubleJumped,
                            ..default()
                        }),
                    ]);
                } else {
                    // This prevents accidental immediate double jump (odd low jump)
                    initial_events.push(ActionEvent::Condition(StatusCondition {
                        flag: StatusFlag::DoubleJumped,
                        expiration: Some(10),
                        ..default()
                    }))
                }

                if situation.elapsed() == 0 {
                    return initial_events;
                }

                let delay = if $type == JumpType::Air { 1 } else { 3 };
                if situation.elapsed() == delay {
                    return vec![
                        Movement::impulse(impulse).into(),
                        VfxRequest {
                            effect: VisualEffect::SpeedLines,
                            position: Vec3::new(-0.5, 1.3, 0.0),
                            rotation: if impulse.x == 0.0 {
                                Some(std::f32::consts::PI)
                            } else {
                                Some(-impulse.x)
                            },
                        }
                        .into(),
                    ];
                }

                situation.end_at(delay + 5)
            }),
            requirements,
        }
    }};
}

#[macro_export]
macro_rules! jumps {
    ($height:expr, $duration:expr, $animation:expr) => {{
        use $crate::jump;

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
        let gravity_force: f32 = 2.0 * $height / (($duration / 2.0) as f32).powf(2.0);
        let gravity_per_frame: f32 = gravity_force / wag_core::FPS;

        let jumps = vec![
            (
                ActionId::BackJump,
                jump!(
                    $height,
                    $duration,
                    $animation,
                    JumpDirection::Back,
                    JumpType::Basic
                ),
            ),
            (
                ActionId::NeutralJump,
                jump!(
                    $height,
                    $duration,
                    $animation,
                    JumpDirection::Neutral,
                    JumpType::Basic
                ),
            ),
            (
                ActionId::ForwardJump,
                jump!(
                    $height,
                    $duration,
                    $animation,
                    JumpDirection::Forward,
                    JumpType::Basic
                ),
            ),
            (
                ActionId::BackSuperJump,
                jump!(
                    $height,
                    $duration,
                    $animation,
                    JumpDirection::Back,
                    JumpType::Super
                ),
            ),
            (
                ActionId::NeutralSuperJump,
                jump!(
                    $height,
                    $duration,
                    $animation,
                    JumpDirection::Neutral,
                    JumpType::Super
                ),
            ),
            (
                ActionId::ForwardSuperJump,
                jump!(
                    $height,
                    $duration,
                    $animation,
                    JumpDirection::Forward,
                    JumpType::Super
                ),
            ),
            (
                ActionId::BackAirJump,
                jump!(
                    $height,
                    $duration,
                    $animation,
                    JumpDirection::Back,
                    JumpType::Air
                ),
            ),
            (
                ActionId::NeutralAirJump,
                jump!(
                    $height,
                    $duration,
                    $animation,
                    JumpDirection::Neutral,
                    JumpType::Air
                ),
            ),
            (
                ActionId::ForwardAirJump,
                jump!(
                    $height,
                    $duration,
                    $animation,
                    JumpDirection::Forward,
                    JumpType::Air
                ),
            ),
        ]
        .into_iter();

        (jumps, gravity_per_frame)
    }};
}

#[macro_export]
macro_rules! dash_script {
    ($input:expr, $startup_duration:expr, $total_duration:expr, $first_impulse:expr, $second_impulse:expr, $animation:expr, $backdash:expr, $track_spikes:expr) => {{
        use wag_core::{
            Animation, StatusCondition, StatusFlag, VfxRequest, VisualEffect,
            TRACK_SPIKES_FLASH_COLOR,
        };
        use $crate::{ActionEvent::*, FlashRequest, Movement, ResourceType, Situation};

        Box::new(|situation: &Situation| {
            if situation.elapsed() == 0 {
                let mut initial_events = vec![
                    Into::<Animation>::into($animation).into(),
                    VfxRequest {
                        effect: VisualEffect::SpeedLines,
                        position: Vec3::new(-0.5, 1.3, 0.0),
                        rotation: Some(-situation.facing.to_signum() * std::f32::consts::PI / 2.0),
                    }
                    .into(),
                ];

                initial_events.push(Movement::impulse($first_impulse).into());

                if $track_spikes {
                    initial_events.extend(vec![
                        ModifyResource(ResourceType::Meter, -40),
                        Flash(FlashRequest {
                            color: TRACK_SPIKES_FLASH_COLOR,
                            ..default()
                        }),
                    ]);
                }

                if $backdash && situation.stats.backdash_invuln > 0 {
                    initial_events.push(ActionEvent::Condition(StatusCondition {
                        flag: StatusFlag::Intangible,
                        effect: None,
                        expiration: Some(situation.stats.backdash_invuln as usize),
                    }));
                }

                return initial_events;
            }

            if situation.elapsed() == $startup_duration && $second_impulse != Vec2::ZERO {
                return vec![Movement::impulse($second_impulse).into()];
            }

            situation.end_at($total_duration)
        })
    }};
}

#[macro_export]
macro_rules! dash {
    ($input:expr, $startup_duration:expr, $total_duration:expr, $first_impulse:expr, $second_impulse:expr, $animation:expr, $backdash:expr, $track_spikes:expr) => {{
        use wag_core::ActionId;
        use $crate::dash_script;
        use $crate::{Action, ActionRequirement, ResourceType};

        let mut requirements = vec![ActionRequirement::Grounded];

        if $track_spikes {
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

        Action {
            input: Some($input),
            category: if $track_spikes {
                ActionCategory::Super
            } else {
                ActionCategory::Dash
            },
            script: dash_script!(
                $input,
                $startup_duration,
                $total_duration,
                $first_impulse,
                $second_impulse,
                $animation,
                $backdash,
                $track_spikes
            ),
            requirements,
        }
    }};
}

#[macro_export]
macro_rules! dashes {
    ($forward_animation:expr, $back_animation:expr) => {{
        use $crate::dash;
        vec![
            (
                ActionId::DashForward,
                dash!(
                    "5656",
                    6,
                    20,
                    Vec2::X * 5.0,
                    Vec2::new(2.0, 4.0),
                    $forward_animation,
                    false,
                    false
                ),
            ),
            (
                ActionId::DashBack,
                dash!(
                    "5454",
                    0,
                    20,
                    Vec2::X * -7.0,
                    Vec2::ZERO,
                    $back_animation,
                    true,
                    false
                ),
            ),
            (
                ActionId::TrackSpikesDashForward,
                dash!(
                    "5656",
                    6,
                    20,
                    Vec2::X * 5.0,
                    Vec2::new(2.0, 4.0),
                    $forward_animation,
                    false,
                    true
                ),
            ),
            (
                ActionId::TrackSpikesDashBack,
                dash!(
                    "5454",
                    0,
                    20,
                    Vec2::ZERO,
                    Vec2::new(-7.0, 0.0),
                    $back_animation,
                    true,
                    true
                ),
            ),
        ]
        .into_iter()
    }};
}
