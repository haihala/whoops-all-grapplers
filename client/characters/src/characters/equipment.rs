use bevy::prelude::*;
use core::{Area, GameButton, ItemId, MoveId};

use crate::{
    moves::{Action, FlowControl, MoveType, Situation},
    Cost, Hitbox, Lifetime, Move, OnHitEffect, ToHit,
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
            Action::Attack(
                ToHit {
                    hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                    velocity: Some(3.0 * Vec2::X),
                    lifetime: Lifetime::eternal(),
                    ..default()
                },
                OnHitEffect::default(),
            )
            .into(),
            FlowControl::Wait(10, true),
        ],
    }
}

pub(crate) fn get_gunshot() -> Move {
    // Single shot, the repeating bit
    Move {
        input: None,
        phases: vec![
            FlowControl::Wait(10, false),
            FlowControl::Dynamic(|situation: Situation| {
                if situation.resources.can_afford(Cost::bullet()) {
                    Action::Attack(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.1, 0.1)),
                            velocity: Some(8.0 * Vec2::X),
                            lifetime: Lifetime::eternal(),
                            ..default()
                        },
                        OnHitEffect::default(),
                    )
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
            FlowControl::Wait(30, false),
            Action::Move(MoveId::Gunshot).into(),
        ],
        ..default()
    }
}
