use bevy::prelude::*;
use wag_core::{Area, GameButton, ItemId, MoveId, Status, StatusCondition};

use crate::{
    moves::{
        Action, Attack, CancelPolicy::*, CommonAttackProps, FlowControl::*, MoveType::*, Situation,
    },
    Cost, Hitbox, Lifetime, Move, ToHit,
};

pub(crate) fn get_handmedownken() -> Move {
    Move {
        input: Some("236e"),
        move_type: Special,
        requirement: |situation: Situation| {
            situation.inventory.contains(&ItemId::HandMeDownKen) && situation.grounded
        },
        phases: vec![
            Wait(30, Never),
            Attack::new(
                ToHit {
                    hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                    velocity: Some(3.0 * Vec2::X),
                    lifetime: Lifetime::eternal(),
                    ..default()
                },
                CommonAttackProps::default(),
            )
            .into(),
            Wait(10, IfHit),
        ],
    }
}

pub(crate) fn get_gunshot() -> Move {
    // Single shot, the repeating bit
    Move {
        input: None,
        phases: vec![
            Wait(10, Never),
            DynamicActions(|situation: Situation| {
                if situation.resources.can_afford(Cost::bullet()) {
                    vec![Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.1, 0.1)),
                            velocity: Some(8.0 * Vec2::X),
                            lifetime: Lifetime::eternal(),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into()]
                } else {
                    // TODO: put a sound effect here or something later
                    vec![]
                }
            }),
            Wait(10, Never),
            DynamicActions(|situation: Situation| {
                if situation
                    .parser
                    .get_pressed()
                    .contains(&GameButton::Equipment)
                {
                    vec![Action::Move(MoveId::Gunshot)]
                } else {
                    vec![]
                }
            }),
            Wait(30, Never),
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
        phases: vec![Wait(30, Never), Action::Move(MoveId::Gunshot).into()],
        ..default()
    }
}

pub(crate) fn get_high_gi_parry() -> Move {
    Move {
        input: Some("56"),
        move_type: Normal,
        requirement: |situation: Situation| {
            situation.inventory.contains(&ItemId::Gi) && situation.grounded
        },
        phases: vec![
            vec![
                Action::ForceStand,
                Action::Condition(StatusCondition {
                    name: Status::Parry,
                    effect: None,
                    expiration: Some(20), // TODO tone down, this is for testing
                }),
            ]
            .into(),
            // 0f moves will end on the same system they are processed and their events will get cleared before those get handled
            // Could be fixed, but likely not severe enough to.
            Wait(1, Never),
        ],
        ..default()
    }
}
