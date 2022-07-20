use bevy::prelude::*;
use map_macro::set;
use types::{Area, GameButton};

use crate::{
    moves::MoveType, Branch, Cost, Hitbox, ItemId, Lifetime, Move, MoveId, Phase, PhaseKind,
    Requirements, SpawnDescriptor,
};

pub(crate) fn get_handmedownken() -> Move {
    Move {
        input: Some("236e"),
        move_type: MoveType::Special,
        requirements: Requirements {
            items: Some(set! {ItemId::HandMeDownKen}),
            grounded: Some(true),
            ..default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 30,
                ..default()
            }
            .into(),
            Phase {
                kind: PhaseKind::Attack(SpawnDescriptor {
                    hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                    speed: 3.0 * Vec3::X,
                    lifetime: Lifetime::Forever,
                    ..default()
                }),
                duration: 4,
                ..default()
            }
            .into(),
            Phase {
                kind: PhaseKind::Animation,
                duration: 10,
                cancellable: true,
                ..default()
            }
            .into(),
        ],
        ..default()
    }
}

pub(crate) fn get_gunshot() -> Move {
    // Single shot, the repeating bit
    Move {
        input: None,
        move_type: MoveType::Normal,
        requirements: Requirements {
            grounded: Some(true),
            ..default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 10,
                ..default()
            }
            .into(),
            Branch {
                default: Phase {
                    kind: PhaseKind::Animation,
                    duration: 30,
                    ..default()
                }
                .into(),
                branches: vec![(
                    Requirements {
                        cost: Some(Cost {
                            bullet: true,
                            ..default()
                        }),
                        ..default()
                    },
                    Phase {
                        duration: 20,
                        kind: PhaseKind::Attack(SpawnDescriptor {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.1, 0.1)),
                            speed: 8.0 * Vec3::X,
                            lifetime: Lifetime::Forever,
                            ..default()
                        }),
                        ..default()
                    }
                    .into(),
                )],
            },
            Branch {
                default: Phase {
                    kind: PhaseKind::Animation,
                    duration: 30,
                    ..default()
                }
                .into(),
                branches: vec![(
                    Requirements {
                        buttons_held: Some(set! {GameButton::Equipment }),
                        ..default()
                    },
                    MoveId::Gunshot.into(),
                )],
            },
        ],
        ..default()
    }
}

pub(crate) fn get_shot() -> Move {
    Move {
        input: Some("e"),
        move_type: MoveType::Normal,
        requirements: Requirements {
            items: Some(set! {ItemId::HandMeDownKen}),
            grounded: Some(true),
            ..default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 30,
                ..default()
            }
            .into(),
            MoveId::Gunshot.into(),
        ],
        ..default()
    }
}
