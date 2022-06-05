use bevy::prelude::*;
use types::GameButton;

use crate::{
    Branch, CancelLevel, Cost, Hitbox, Lifetime, Move, MoveId, Phase, PhaseKind, Requirements,
    SpawnDescriptor,
};

pub fn get_equipment_move(id: MoveId) -> Move {
    match id {
        MoveId::HandMeDownKen => get_handmedownken(),
        MoveId::Gunshot => get_gunshot(),
        MoveId::Shoot => get_shot(),
        _ => panic!("Requesting an equipment move that is not defined"),
    }
}

fn get_handmedownken() -> Move {
    Move {
        input: Some("236e"),
        requirements: Requirements {
            grounded: Some(true),
            cancel_level: Some(CancelLevel::LightSpecial),
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
                    hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.3, 0.2)),
                    speed: Some(1.0 * Vec3::X),
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
    }
}

fn get_gunshot() -> Move {
    // Single shot, the repeating bit
    Move {
        input: None,
        requirements: Requirements {
            grounded: Some(true),
            cancel_level: Some(CancelLevel::LightNormal),
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
                            hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.3, 0.2)),
                            speed: Some(10.0 * Vec3::X),
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
                        buttons_held: Some(vec![GameButton::Equipment]),
                        ..default()
                    },
                    MoveId::Gunshot.into(),
                )],
            },
        ],
    }
}

fn get_shot() -> Move {
    Move {
        input: Some("e"),
        requirements: Requirements {
            grounded: Some(true),
            cancel_level: Some(CancelLevel::LightNormal),
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
    }
}
