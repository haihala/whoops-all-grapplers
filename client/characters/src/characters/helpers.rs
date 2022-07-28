use crate::{moves::MoveType, Move, MoveMobility, Phase, PhaseKind, Requirements};
use bevy::prelude::*;

pub fn jump(input: &'static str, impulse: Vec2) -> Move {
    Move {
        input: Some(input),
        move_type: MoveType::Normal,
        requirements: Requirements {
            grounded: Some(true),
            ..default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 5,
                mobility: Some(MoveMobility::Impulse(impulse.extend(0.0))),
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
        ..default()
    }
}

pub fn dash(input: &'static str, duration: usize, impulse: f32) -> Move {
    Move {
        input: Some(input),
        move_type: MoveType::Special,
        requirements: Requirements {
            grounded: Some(true),
            ..default()
        },
        phases: vec![Phase {
            duration,
            kind: PhaseKind::Animation,
            mobility: Some(MoveMobility::Impulse(Vec3::X * impulse)),
            cancellable: true,
            ..default()
        }
        .into()],
        ..default()
    }
}
