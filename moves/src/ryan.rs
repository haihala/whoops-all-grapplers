use bevy::prelude::*;
use types::{Hit, Hitbox, Player};

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

moves!(2usize, (HADOUKEN, AIR_PUNCH, COMMAND_PUNCH, PUNCH));

pub fn ryan_bank(player: Player) -> MoveBank {
    MoveBank::new(
        player,
        vec![
            (
                universal::DASH_FORWARD,
                Move {
                    input: "656",
                    cancel_level: CancelLevel::Dash,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: DASH_START_FRAMES,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::new(DASH_START_SPEED, 0.0, 0.0),
                        },
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: DASH_RECOVERY_FRAMES,
                            cancel_requirement: CancelLevel::LightNormal,
                            mobility: Vec3::new(DASH_RECOVERY_SPEED, 0.0, 0.0),
                        },
                    ],
                    ground_ok: true,
                    ..Default::default()
                },
            ),
            (
                universal::DASH_BACK,
                Move {
                    input: "454",
                    cancel_level: CancelLevel::Dash,
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
                    ground_ok: true,
                    ..Default::default()
                },
            ),
            (
                PUNCH,
                Move {
                    input: "l",
                    cancel_level: CancelLevel::LightNormal,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Hitbox(Hitbox::new(
                                Vec2::new(1.0, 0.5),
                                Vec2::new(0.2, 0.3),
                                Hit {
                                    hit_knockback: Vec3::new(2.0, 2.0, 0.0),
                                    ..Default::default()
                                },
                            )),
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
                    ground_ok: true,
                    ..Default::default()
                },
            ),
            (
                COMMAND_PUNCH,
                Move {
                    input: "6l",
                    cancel_level: CancelLevel::LightNormal,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::new(1.0, 0.0, 0.0),
                        },
                        Phase {
                            kind: PhaseKind::Hitbox(Hitbox::new(
                                Vec2::new(0.5, 0.5),
                                Vec2::new(1.0, 1.0),
                                Hit::default(),
                            )),
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
                    ground_ok: true,
                    ..Default::default()
                },
            ),
            (
                HADOUKEN,
                Move {
                    input: "236l",
                    cancel_level: CancelLevel::LightSpecial,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 30,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Projectile {
                                speed: 1.0,
                                hitbox: Hitbox::new(
                                    Vec2::new(0.5, 0.5),
                                    Vec2::new(0.3, 0.2),
                                    Hit::default(),
                                ),
                                lifetime: None,
                            },
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
                    ground_ok: true,
                    ..Default::default()
                },
            ),
            (
                AIR_PUNCH,
                Move {
                    input: "l",
                    cancel_level: CancelLevel::LightNormal,
                    phases: vec![
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 10,
                            cancel_requirement: CancelLevel::Uncancellable,
                            mobility: Vec3::ZERO,
                        },
                        Phase {
                            kind: PhaseKind::Hitbox(Hitbox::new(
                                Vec2::new(0.4, -1.2),
                                Vec2::new(0.2, 0.3),
                                Hit {
                                    hit_knockback: Vec3::new(2.0, 2.0, 0.0),
                                    ..Default::default()
                                },
                            )),
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
                    air_ok: true,
                    ..Default::default()
                },
            ),
        ]
        .into_iter()
        .collect(),
    )
}
