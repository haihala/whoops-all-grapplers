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
            ..Default::default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 30,
                ..Default::default()
            }
            .into(),
            Phase {
                kind: PhaseKind::Attack(SpawnDescriptor {
                    hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.3, 0.2)),
                    speed: Some(1.0 * Vec3::X),
                    lifetime: Lifetime::Forever,
                    ..Default::default()
                }),
                duration: 4,
                ..Default::default()
            }
            .into(),
            Phase {
                kind: PhaseKind::Animation,
                duration: 10,
                cancellable: true,
                ..Default::default()
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
            cost: Some(Cost {
                // TODO bullets go here
                ..Default::default()
            }),
            ..Default::default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 10,
                ..Default::default()
            }
            .into(),
            Phase {
                duration: 20,
                kind: PhaseKind::Attack(SpawnDescriptor {
                    hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.3, 0.2)),
                    speed: Some(10.0 * Vec3::X),
                    lifetime: Lifetime::Forever,
                    ..Default::default()
                }),
                ..Default::default()
            }
            .into(),
            Branch {
                default: Phase {
                    kind: PhaseKind::Animation,
                    duration: 30,
                    ..Default::default()
                }
                .into(),
                branches: vec![(
                    Requirements {
                        buttons_held: Some(vec![GameButton::Equipment]),
                        ..Default::default()
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
            ..Default::default()
        },
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 30,
                ..Default::default()
            }
            .into(),
            MoveId::Gunshot.into(),
        ],
    }
}
