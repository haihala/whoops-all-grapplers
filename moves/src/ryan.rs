use bevy::prelude::*;
use types::{AttackDescriptor, AttackHeight, GrabDescription, Hitbox, Lifetime};

use crate::{move_bank::MoveBank, moves, universal, CancelLevel, Move, Phase, PhaseKind};

// Dashing
const DASH_START_DURATION_SECONDS: f32 = 0.1;
const DASH_RECOVERY_DURATION_SECONDS: f32 = 0.2;
const DASH_DISTANCE: f32 = 3.0;
const DASH_START_DISTANCE_FRACTION: f32 = 0.5;

const SHIFT_DURING_DASH_START: f32 = DASH_DISTANCE * DASH_START_DISTANCE_FRACTION;
const SHIFT_DURING_DASH_RECOVERY: f32 = DASH_DISTANCE * (1.0 - DASH_START_DISTANCE_FRACTION);

const DASH_START_SPEED: f32 = SHIFT_DURING_DASH_START / DASH_START_DURATION_SECONDS;
const DASH_RECOVERY_SPEED: f32 = SHIFT_DURING_DASH_RECOVERY / DASH_RECOVERY_DURATION_SECONDS;
const DASH_START_FRAMES: usize = (DASH_START_DURATION_SECONDS * constants::FPS) as usize;
const DASH_RECOVERY_FRAMES: usize = (DASH_RECOVERY_DURATION_SECONDS * constants::FPS) as usize;

moves!(
    2usize,
    (
        GRAB,
        HEAVY_HADOUKEN,
        HADOUKEN,
        AIR_PUNCH,
        COMMAND_PUNCH,
        PUNCH
    )
);

pub fn ryan_bank() -> MoveBank {
    MoveBank::new(
        vec![
            (
                universal::DASH_FORWARD,
                Move {
                    input: "656",
                    cancel_level: CancelLevel::Dash,
                    ground_ok: true,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: DASH_START_FRAMES,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::X * DASH_START_SPEED,
                        },
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: DASH_RECOVERY_FRAMES,
                            cancel_requirement: CancelLevel::LightNormal,
                            mobility: Vec3::X * DASH_RECOVERY_SPEED,
                        },
                    ],
                    ..Default::default()
                },
            ),
            (
                universal::DASH_BACK,
                Move {
                    input: "454",
                    cancel_level: CancelLevel::Dash,
                    ground_ok: true,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: DASH_START_FRAMES,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::new(-DASH_START_SPEED, 0.0, 0.0),
                        },
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: DASH_RECOVERY_FRAMES,
                            cancel_requirement: CancelLevel::LightNormal,
                            mobility: Vec3::new(-DASH_RECOVERY_SPEED, 0.0, 0.0),
                        },
                    ],
                    ..Default::default()
                },
            ),
            (
                PUNCH,
                Move {
                    input: "l",
                    cancel_level: CancelLevel::LightNormal,
                    ground_ok: true,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Attack(AttackDescriptor {
                                hitbox: Hitbox::new(Vec2::new(1.0, 0.5), Vec2::new(0.2, 0.3)),
                                attached_to_player: true,
                                damage: Some(20.into()),
                                ..Default::default()
                            }),
                            duration: 10,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::LightSpecial,
                            mobility: Vec3::ZERO,
                        },
                    ],
                    ..Default::default()
                },
            ),
            (
                COMMAND_PUNCH,
                Move {
                    input: "6l",
                    cancel_level: CancelLevel::LightNormal,
                    ground_ok: true,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::new(1.0, 0.0, 0.0),
                        },
                        Phase {
                            kind: PhaseKind::Attack(AttackDescriptor {
                                hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(1.0, 1.0)),
                                attached_to_player: true,
                                ..Default::default()
                            }),
                            duration: 20,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::new(5.0, 0.0, 0.0),
                        },
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 20,
                            cancel_requirement: CancelLevel::LightSpecial,
                            mobility: Vec3::ZERO,
                        },
                    ],
                    ..Default::default()
                },
            ),
            (
                HADOUKEN,
                Move {
                    input: "236l",
                    cancel_level: CancelLevel::LightSpecial,
                    ground_ok: true,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 30,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Attack(AttackDescriptor {
                                hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.3, 0.2)),
                                speed: Some(1.0 * Vec3::X),
                                lifetime: Lifetime::Forever,
                                ..Default::default()
                            }),
                            duration: 4,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::HeavyNormal,
                            mobility: Vec3::ZERO,
                        },
                    ],
                    ..Default::default()
                },
            ),
            (
                HEAVY_HADOUKEN,
                Move {
                    input: "236h",
                    cancel_level: CancelLevel::HeavySpecial,
                    meter_cost: 10,
                    ground_ok: true,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 30,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Attack(AttackDescriptor {
                                hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.7, 0.7)),
                                speed: Some(2.0 * Vec3::X),
                                lifetime: Lifetime::Forever,
                                ..Default::default()
                            }),
                            duration: 4,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::Jump,
                            mobility: Vec3::ZERO,
                        },
                    ],
                    ..Default::default()
                },
            ),
            (
                AIR_PUNCH,
                Move {
                    input: "l",
                    cancel_level: CancelLevel::LightNormal,
                    air_ok: true,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Attack(AttackDescriptor {
                                hitbox: Hitbox::new(Vec2::new(0.4, -1.2), Vec2::new(0.2, 0.3)),
                                knockback: Some(Vec3::new(2.0, 2.0, 0.0).into()),
                                fixed_height: Some(AttackHeight::High),
                                attached_to_player: true,
                                ..Default::default()
                            }),
                            duration: 10,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::LightSpecial,
                            mobility: Vec3::ZERO,
                        },
                    ],
                    ..Default::default()
                },
            ),
            (
                GRAB,
                Move {
                    input: "g",
                    cancel_level: CancelLevel::Grab,
                    ground_ok: true,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 1,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Grab(GrabDescription {
                                damage: 40,
                                impulse: Vec3::Y * 2.0,
                                range: 1.0,
                                ..Default::default()
                            }),
                            duration: 1,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                    ],
                    ..Default::default()
                },
            ),
        ]
        .into_iter()
        .collect(),
    )
}
