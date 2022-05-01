use crate::{CancelLevel, Move, MoveMobility, Phase, PhaseKind, Requirements};
use bevy::prelude::*;

pub fn jump(input: &'static str, impulse: impl Into<Vec3>) -> Move {
    Move {
        input: Some(input),
        requirements: Requirements {
            grounded: Some(true),
            cancel_level: Some(CancelLevel::Jump),
            ..Default::default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 5,
                mobility: Some(MoveMobility::Impulse(impulse.into())),
                ..Default::default()
            }
            .into(),
            Phase {
                kind: PhaseKind::Animation,
                duration: 5,
                ..Default::default()
            }
            .into(),
        ],
    }
}

pub fn dash(
    input: &'static str,
    start_speed: f32,
    recovery_speed: f32,
    start_duration: usize,
    recovery_duration: usize,
) -> Move {
    Move {
        input: Some(input),
        requirements: Requirements {
            cancel_level: Some(CancelLevel::Dash),
            grounded: Some(true),
            ..Default::default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: start_duration,
                mobility: Some(MoveMobility::Perpetual(Vec3::X * start_speed)),
                ..Default::default()
            }
            .into(),
            Phase {
                kind: PhaseKind::Animation,
                duration: recovery_duration,
                cancellable: true,
                mobility: Some(MoveMobility::Perpetual(Vec3::X * recovery_speed)),
            }
            .into(),
        ],
    }
}
