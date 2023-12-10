use bevy::prelude::*;
use wag_core::{ActionId, Animation, ItemId, Stats, StatusCondition, StatusFlag};

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
                    expiration: Some(15),
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

fn parry_flash(parry_animation: Animation) -> Action {
    Action::new(
        Some("56"),
        CancelCategory::Any,
        vec![ActionBlock {
            events: vec![parry_animation.into()],
            exit_requirement: ContinuationRequirement::Time(10),
            cancel_policy: CancelPolicy::never(),
            mutator: None,
        }],
        vec![
            ActionRequirement::Grounded,
            ActionRequirement::ItemsOwned(vec![ItemId::Gi]),
        ],
    )
}

pub fn universal_item_actions(
    parry_animation: Animation,
) -> impl Iterator<Item = (ActionId, Action)> {
    vec![
        (ActionId::HighGiParry, get_high_gi_parry()),
        (ActionId::ParryFlash, parry_flash(parry_animation)),
    ]
    .into_iter()
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
                    walk_speed: 0.1,
                    ..Stats::identity()
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
                    ..Stats::identity()
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
        (
            ItemId::ThumbTacks(1),
            Item {
                category: Basic,
                explanation: "+1 damage to all hits\n\nOuch".into(),
                cost: 50,
                effect: Stats {
                    flat_damage: 1,
                    ..Stats::identity()
                },
                ..default()
            },
        ),
    ]
    .into_iter()
    .chain((2..9).map(|id| {
        (
            ItemId::ThumbTacks(id),
            Item {
                category: Upgrade(vec![ItemId::ThumbTacks(id - 1), ItemId::ThumbTacks(id - 1)]),
                explanation: format!(
                    "+{} damage to all hits\n\nExponential growth is fun!",
                    usize::pow(2, (id - 1) as u32)
                ),
                cost: 10,
                ..default()
            },
        )
    }))
}
