use bevy::prelude::*;
use wag_core::{Area, ItemId, StatusCondition, StatusFlag};

use crate::{
    moves::{
        Action, Attack, CancelCategory, CancelPolicy, CommonAttackProps, FlowControl::*, Situation,
    },
    Hitbox, Lifetime, Move, ToHit,
};

pub(crate) fn get_handmedownken() -> Move {
    Move::new(
        Some("236g"),
        CancelCategory::Special,
        vec![
            Wait(30, CancelPolicy::never()),
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
            Wait(10, CancelPolicy::never()),
        ],
        |situation: Situation| {
            situation.inventory.contains(&ItemId::HandMeDownKen) && situation.grounded
        },
    )
}

pub(crate) fn get_high_gi_parry() -> Move {
    Move::new(
        Some("56"),
        CancelCategory::Any,
        vec![
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
            Wait(1, CancelPolicy::never()),
        ],
        |situation: Situation| situation.inventory.contains(&ItemId::Gi) && situation.grounded,
    )
}
