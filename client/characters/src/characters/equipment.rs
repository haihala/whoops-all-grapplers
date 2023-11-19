use bevy::prelude::*;
use wag_core::{ActionId, ItemId, Stats, StatusCondition, StatusFlag};

use crate::{
    actions::ActionRequirement, Action, ActionBlock, ActionEvent, CancelCategory, CancelPolicy,
    ContinuationRequirement, Item, ItemCategory::*,
};

fn get_high_gi_parry() -> Action {
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
            exit_requirement: ContinuationRequirement::Time(1),
            cancel_policy: CancelPolicy::never(),
            mutator: None,
        }],
        vec![
            ActionRequirement::Grounded,
            ActionRequirement::ItemsOwned(vec![ItemId::Gi]),
        ],
    )
}

pub fn universal_item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    vec![(ActionId::HighGiParry, get_high_gi_parry())].into_iter()
}

pub fn universal_items() -> impl Iterator<Item = (ItemId, Item)> {
    vec![
        (
            ItemId::Gi,
            Item {
                cost: 100,
                explanation: "Tap forward to parry\n\nLesgo justin".into(),
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
            ItemId::HockeyPads,
            Item {
                cost: 50,
                explanation: "Bonus max health\n\nI am wearing hockey pads".into(),
                effect: Stats {
                    max_health: 20,
                    ..default()
                },
                ..default()
            },
        ),
        (
            ItemId::SafetyBoots,
            Item {
                category: Upgrade(vec![ItemId::Boots, ItemId::HockeyPads]),
                explanation: "Speed and health!\n\nSafe and fashionable".into(),
                cost: 100,
                ..default()
            },
        ),
    ]
    .into_iter()
}
