use crate::{CancelLevel, Move, MoveMobility, Phase, PhaseKind, Requirements};
use bevy::prelude::*;

pub fn jump(input: &'static str, impulse: impl Into<Vec3>) -> Move {
    Move {
        input: Some(input),
        requirements: Requirements {
            grounded: Some(true),
            cancel_level: Some(CancelLevel::Jump),
            ..default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 5,
                mobility: Some(MoveMobility::Impulse(impulse.into())),
                ..default()
            }
            .into(),
            Phase {
                kind: PhaseKind::Animation,
                duration: 5,
                ..default()
            }
            .into(),
        ],
    }
}

pub fn dash(input: &'static str, duration: usize, impulse: f32) -> Move {
    Move {
        input: Some(input),
        requirements: Requirements {
            cancel_level: Some(CancelLevel::Dash),
            grounded: Some(true),
            ..default()
        },
        phases: vec![Phase {
            duration,
            kind: PhaseKind::Animation,
            mobility: Some(MoveMobility::Impulse(Vec3::X * impulse)),
            cancellable: true,
        }
        .into()],
    }
}
