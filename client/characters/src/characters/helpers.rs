use std::f32::consts::PI;

use bevy::prelude::*;
use wag_core::{
    ActionCategory, ActionId, Animation, ItemId, StatusCondition, StatusFlag, VfxRequest,
    VisualEffect,
};

use crate::{Action, ActionEvent, ActionRequirement, Movement, Situation};

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

    pub fn input(&self, jump_type: JumpType) -> String {
        if jump_type == JumpType::Super {
            self.super_input()
        } else {
            self.base_input()
        }
        .into()
    }

    fn base_input(self) -> &'static str {
        match self {
            JumpDirection::Neutral => "8",
            JumpDirection::Forward => "9",
            JumpDirection::Back => "7",
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

fn jump(
    gravity_force: f32,
    duration: f32,
    animation: Animation,
    jump_dir: JumpDirection,
    jump_type: JumpType,
) -> Action {
    Action {
        input: Some(jump_dir.input(jump_type)),
        script: Box::new(move |situation: &Situation| {
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
            let impulse = jump_dir.base_vec()
                * base_impulse
                * situation.stats.jump_force_multiplier
                * match jump_type {
                    JumpType::Basic => 1.0,
                    JumpType::Air => 0.7,
                    JumpType::Super => 1.2,
                };

            let mut initial_events = vec![animation.into()];

            if jump_type == JumpType::Air {
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

            if situation.on_frame(0) {
                return initial_events;
            }

            let delay = if jump_type == JumpType::Air { 1 } else { 3 };
            if situation.on_frame(delay) {
                return vec![
                    Movement::impulse(impulse).into(),
                    VfxRequest {
                        effect: VisualEffect::SpeedLines,
                        tf: Transform {
                            translation: Vec3::new(0.0, 1.3, 0.0),
                            rotation: Quat::from_rotation_z(if impulse.x == 0.0 {
                                std::f32::consts::PI
                            } else {
                                -situation.facing.to_signum()
                                    * jump_dir.base_vec().x.signum()
                                    * std::f32::consts::PI
                                    / 4.0
                            }),
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                ];
            }

            situation.end_at(delay + 5)
        }),
        requirement: ActionRequirement::And(match jump_type {
            JumpType::Basic => vec![
                ActionRequirement::Grounded,
                ActionRequirement::Starter(ActionCategory::Jump),
            ],
            JumpType::Air => vec![
                ActionRequirement::Airborne,
                ActionRequirement::ItemOwned(ItemId::FeatheredBoots),
                ActionRequirement::StatusNotActive(StatusFlag::DoubleJumped),
                ActionRequirement::Starter(ActionCategory::Jump),
            ],
            JumpType::Super => vec![
                ActionRequirement::Grounded,
                ActionRequirement::ItemOwned(ItemId::MoonBoots),
                ActionRequirement::Starter(ActionCategory::Jump),
            ],
        }),
    }
}

pub fn jumps(
    height: f32,
    duration: f32,
    anim: impl Into<Animation>,
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
    let gravity_force = 2.0 * height / (duration / 2.0).powf(2.0);
    let gravity_per_frame = gravity_force / wag_core::FPS;

    let animation = Into::<Animation>::into(anim);

    let jumps = vec![
        (
            ActionId::BackJump,
            jump(
                gravity_force,
                duration,
                animation,
                JumpDirection::Back,
                JumpType::Basic,
            ),
        ),
        (
            ActionId::NeutralJump,
            jump(
                gravity_force,
                duration,
                animation,
                JumpDirection::Neutral,
                JumpType::Basic,
            ),
        ),
        (
            ActionId::ForwardJump,
            jump(
                gravity_force,
                duration,
                animation,
                JumpDirection::Forward,
                JumpType::Basic,
            ),
        ),
        (
            ActionId::BackSuperJump,
            jump(
                gravity_force,
                duration,
                animation,
                JumpDirection::Back,
                JumpType::Super,
            ),
        ),
        (
            ActionId::NeutralSuperJump,
            jump(
                gravity_force,
                duration,
                animation,
                JumpDirection::Neutral,
                JumpType::Super,
            ),
        ),
        (
            ActionId::ForwardSuperJump,
            jump(
                gravity_force,
                duration,
                animation,
                JumpDirection::Forward,
                JumpType::Super,
            ),
        ),
        (
            ActionId::BackAirJump,
            jump(
                gravity_force,
                duration,
                animation,
                JumpDirection::Back,
                JumpType::Air,
            ),
        ),
        (
            ActionId::NeutralAirJump,
            jump(
                gravity_force,
                duration,
                animation,
                JumpDirection::Neutral,
                JumpType::Air,
            ),
        ),
        (
            ActionId::ForwardAirJump,
            jump(
                gravity_force,
                duration,
                animation,
                JumpDirection::Forward,
                JumpType::Air,
            ),
        ),
    ]
    .into_iter();

    (jumps, gravity_per_frame)
}
