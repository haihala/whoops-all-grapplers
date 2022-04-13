use bevy::prelude::*;

use crate::{
    CancelLevel, ConditionResolver, Hitbox, Lifetime, Move, MoveCost, MoveFlags, MoveId,
    MoveStartCondition, Phase, PhaseKind, SpawnDescriptor,
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
        cancel_level: CancelLevel::LightSpecial,
        conditions: MoveStartCondition::GROUND,
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
        ..Default::default()
    }
}

fn get_gunshot() -> Move {
    // Single shot, the repeating bit
    Move {
        cancel_level: CancelLevel::LightNormal,
        conditions: MoveStartCondition::GROUND,
        cost: MoveCost {
            // TODO bullets go here
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
            ConditionResolver {
                default: Phase {
                    kind: PhaseKind::Animation,
                    duration: 30,
                    ..Default::default()
                }
                .into(),
                branches: vec![(MoveFlags::EQUIPMENT_PRESSED, MoveId::Gunshot.into())],
            },
        ],
        ..Default::default()
    }
}

fn get_shot() -> Move {
    Move {
        input: Some("e"),
        cancel_level: CancelLevel::LightNormal,
        conditions: MoveStartCondition::GROUND,
        phases: vec![
            Phase {
                kind: PhaseKind::Animation,
                duration: 30,
                ..Default::default()
            }
            .into(),
            MoveId::Gunshot.into(),
        ],
        ..Default::default()
    }
}
