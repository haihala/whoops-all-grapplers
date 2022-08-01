use bevy::prelude::*;
use types::{Area, GameButton, ItemId, MoveId};

use crate::{
    moves::{Action, FlowControl, MoveType, Situation},
    Cost, Hitbox, Lifetime, Move, SpawnDescriptor,
};

pub(crate) fn get_handmedownken() -> Move {
    Move {
        input: Some("236e"),
        move_type: MoveType::Special,
        requirement: |situation: Situation| {
            situation.inventory.contains(&ItemId::HandMeDownKen) && situation.grounded
        },
        phases: vec![
            FlowControl::Wait(30, false),
            Action::Hitbox(SpawnDescriptor {
                hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                speed: 3.0 * Vec3::X,
                lifetime: Lifetime::Forever,
                ..default()
            })
            .into(),
            FlowControl::Wait(10, true),
        ],
    }
}

pub(crate) fn get_gunshot() -> Move {
    // Single shot, the repeating bit
    Move {
        input: None,
        move_type: MoveType::Normal,
        requirement: |situation: Situation| situation.grounded,
        phases: vec![
            FlowControl::Wait(10, false),
            FlowControl::Dynamic(|situation: Situation| {
                if situation.resources.can_afford(Cost::bullet()) {
                    Action::Hitbox(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 1.2, 0.1, 0.1)),
                        speed: 8.0 * Vec3::X,
                        lifetime: Lifetime::Forever,
                        ..default()
                    })
                    .into()
                } else {
                    FlowControl::Wait(30, false)
                }
            }),
            FlowControl::Dynamic(|situation: Situation| {
                if situation
                    .parser
                    .get_pressed()
                    .contains(&GameButton::Equipment)
                {
                    Action::Move(MoveId::Gunshot).into()
                } else {
                    FlowControl::Wait(30, false)
                }
            }),
        ],
    }
}

pub(crate) fn get_shot() -> Move {
    Move {
        input: Some("e"),
        move_type: MoveType::Normal,
        requirement: |situation: Situation| {
            situation.inventory.contains(&ItemId::Gun) && situation.grounded
        },
        phases: vec![
            FlowControl::Wait(30, false),
            Action::Move(MoveId::Gunshot).into(),
        ],
    }
}
