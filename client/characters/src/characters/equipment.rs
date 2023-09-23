use bevy::prelude::*;
use wag_core::{Area, ItemId, Stats, StatusCondition, StatusFlag};

use crate::{
    Action, ActionBlock, ActionEvent, Attack, CancelCategory, CancelPolicy, CommonAttackProps,
    Hitbox, Item, ItemCategory::*, Lifetime, Requirement, Situation, ToHit,
};

pub(crate) fn get_handmedownken() -> Action {
    Action::new(
        Some("236g"),
        CancelCategory::Special,
        vec![
            ActionBlock {
                events: vec![],
                exit_requirement: Requirement::Time(30),
                cancel_policy: CancelPolicy::never(),
                mutator: None,
            },
            ActionBlock {
                events: vec![Attack::new(
                    ToHit {
                        hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                        velocity: Some(3.0 * Vec2::X),
                        lifetime: Lifetime::eternal(),
                        ..default()
                    },
                    CommonAttackProps::default(),
                )
                .into()],
                exit_requirement: Requirement::Time(10),
                cancel_policy: CancelPolicy::never(),
                mutator: None,
            },
        ],
        |situation: Situation| {
            situation.inventory.contains(&ItemId::HandMeDownKen) && situation.grounded
        },
    )
}

pub(crate) fn get_high_gi_parry() -> Action {
    Action::new(
        Some("56"),
        CancelCategory::Any,
        vec![ActionBlock {
            events: vec![
                ActionEvent::ForceStand,
                ActionEvent::Condition(StatusCondition {
                    flag: StatusFlag::Parry,
                    effect: None,
                    expiration: Some(10),
                }),
            ],
            // 0f moves will end on the same system they are processed and their events will get cleared before those get handled
            // Could be fixed, but likely not severe enough to.
            // TODO: Previous comment is from previous implementation, may not be true anymore
            exit_requirement: Requirement::Time(1),
            cancel_policy: CancelPolicy::never(),
            mutator: None,
        }],
        |situation: Situation| situation.inventory.contains(&ItemId::Gi) && situation.grounded,
    )
}

pub fn universal_items() -> impl Iterator<Item = (ItemId, Item)> {
    vec![
        (
            ItemId::HandMeDownKen,
            Item {
                cost: 10,
                explanation: "Haduu ken".into(),
                ..default()
            },
        ),
        (
            ItemId::Gi,
            Item {
                cost: 100,
                explanation: "Lesgo justin".into(),
                ..default()
            },
        ),
        (
            ItemId::Boots,
            Item {
                cost: 80,
                explanation: "Bonus walk speed".into(),
                effect: Stats {
                    walk_speed: 0.2,
                    ..default()
                },
                ..default()
            },
        ),
        (
            ItemId::SafetyBoots,
            Item {
                category: Upgrade(vec![ItemId::Boots]),
                explanation: "Gives more health in addition to boots' speed bonus".into(),
                cost: 100,
                effect: Stats {
                    max_health: 20,
                    ..default()
                },
            },
        ),
    ]
    .into_iter()
}
