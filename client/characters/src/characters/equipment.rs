use bevy::prelude::*;
use wag_core::{Area, ItemId, StatusCondition, StatusFlag};

use crate::{
    moves::{
        Action, Attack, CancelPolicy::*, CommonAttackProps, FlowControl::*, MoveType::*, Situation,
    },
    Hitbox, Lifetime, Move, ToHit,
};

pub(crate) fn get_handmedownken() -> Move {
    Move {
        input: Some("236g"),
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

pub(crate) fn get_high_gi_parry() -> Move {
    Move {
        input: Some("56"),
        requirement: |situation: Situation| {
            situation.inventory.contains(&ItemId::Gi) && situation.grounded
        },
        phases: vec![
            vec![
                Action::ForceStand,
                Action::Condition(StatusCondition {
                    flag: StatusFlag::Parry,
                    effect: None,
                    expiration: Some(10),
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
