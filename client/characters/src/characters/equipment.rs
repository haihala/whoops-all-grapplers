use bevy::prelude::*;
use wag_core::{Area, GameButton, ItemId, MoveId};

use crate::{
    moves::{Action, Attack, CancelPolicy, FlowControl, MoveType, Situation},
    Cost, Hitbox, Lifetime, Move, ToHit,
};

pub(crate) fn get_handmedownken() -> Move {
    Move {
        input: Some("236e"),
        move_type: MoveType::Special,
        requirement: |situation: Situation| {
            situation.inventory.contains(&ItemId::HandMeDownKen) && situation.grounded
        },
        phases: vec![
            FlowControl::Wait(30, CancelPolicy::Never),
            Attack {
                to_hit: ToHit {
                    hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                    velocity: Some(3.0 * Vec2::X),
                    lifetime: Lifetime::eternal(),
                    ..default()
                },
                ..default()
            }
            .into(),
            FlowControl::Wait(10, CancelPolicy::IfHit),
        ],
    }
}

pub(crate) fn get_gunshot() -> Move {
    // Single shot, the repeating bit
    Move {
        input: None,
        phases: vec![
            FlowControl::Wait(10, CancelPolicy::Never),
            FlowControl::DynamicAction(|situation: Situation| {
                if situation.resources.can_afford(Cost::bullet()) {
                    Some(
                        Attack {
                            to_hit: ToHit {
                                hitbox: Hitbox(Area::new(0.5, 1.2, 0.1, 0.1)),
                                velocity: Some(8.0 * Vec2::X),
                                lifetime: Lifetime::eternal(),
                                ..default()
                            },
                            ..default()
                        }
                        .into(),
                    )
                } else {
                    // TODO: put a sound effect here or something later
                    None
                }
            }),
            FlowControl::Wait(10, CancelPolicy::Never),
            FlowControl::DynamicAction(|situation: Situation| {
                if situation
                    .parser
                    .get_pressed()
                    .contains(&GameButton::Equipment)
                {
                    Some(Action::Move(MoveId::Gunshot))
                } else {
                    None
                }
            }),
            FlowControl::Wait(30, CancelPolicy::Never),
        ],
        ..default()
    }
}

pub(crate) fn get_shot() -> Move {
    Move {
        input: Some("e"),
        requirement: |situation: Situation| {
            situation.inventory.contains(&ItemId::Gun) && situation.grounded
        },
        phases: vec![
            FlowControl::Wait(30, CancelPolicy::Never),
            Action::Move(MoveId::Gunshot).into(),
        ],
        ..default()
    }
}
