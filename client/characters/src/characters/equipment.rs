use bevy::prelude::*;
use wag_core::{
    ActionId, Animation, ItemId, Stats, StatusCondition, StatusFlag, GI_PARRY_FLASH_COLOR,
};

use crate::{
    actions::ActionRequirement, Action, ActionBlock, ActionEvent, CancelCategory, CancelPolicy,
    ConsumableType::*, ContinuationRequirement, Item, ItemCategory::*,
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
            exit_requirement: ContinuationRequirement::None,
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
        None,
        CancelCategory::Any,
        vec![ActionBlock {
            events: vec![
                parry_animation.into(),
                ActionEvent::Flash(GI_PARRY_FLASH_COLOR.into()),
            ],
            exit_requirement: ContinuationRequirement::Time(10),
            ..default()
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
        // Consumables
        (
            ItemId::PreWorkout,
            Item {
                cost: 75,
                explanation: "Start with 50 meter\n\nGotta get that pump".into(),
                effect: Stats {
                    starting_meter: 50,
                    ..Stats::identity()
                },
                category: Consumable(OneRound),
            },
        ),
        // Basics
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
            ItemId::RedPaint,
            Item {
                cost: 500,
                explanation: "Increased animation speed\n\nBecause red makes you go fastah".into(),
                effect: Stats {
                    action_speed_multiplier: 1.1,
                    ..Stats::identity()
                },
                ..default()
            },
        ),
        (
            ItemId::Stopwatch,
            Item {
                cost: 150,
                explanation: "Increased link timing meter gain\n\nNot my tempo!".into(),
                effect: Stats {
                    link_bonus_multiplier: 1.1,
                    ..Stats::identity()
                },
                ..default()
            },
        ),
        (
            ItemId::Dumbbell,
            Item {
                cost: 120,
                explanation: "Makes you ever so slightly heavier\n\nNot for training purposes"
                    .into(),
                effect: Stats {
                    gravity: 0.1,
                    ..Stats::identity()
                },
                ..default()
            },
        ),
        (
            ItemId::EagleFeather,
            Item {
                cost: 70,
                explanation: "Makes you jump higher\n\nFly like an eagle".into(),
                effect: Stats {
                    jump_force_multiplier: 1.02,
                    ..Stats::identity()
                },
                ..default()
            },
        ),
        (
            ItemId::Cigarettes,
            Item {
                cost: 300,
                explanation: "Makes you intangible for the first few frames of your backdash\n\nDissapear in a puff".into(),
                effect: Stats {
                    backdash_invuln: 3,
                    ..Stats::identity()
                },
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
            },
        ),
        // Upgrades
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
            ItemId::TrackSpikes,
            Item {
                category: Upgrade(vec![ItemId::Boots, ItemId::Stopwatch]),
                explanation: "Allows you to cancel normals into a dash\n\nNow with Fast Action Disruption Compatible soles".into(),
                cost: 100,
                ..default()
            },
        ),
        (
            ItemId::WingedBoots,
            Item {
                category: Upgrade(vec![ItemId::Boots, ItemId::EagleFeather]),
                explanation: "Allows you to double jump\n\nLike Hermes.".into(),
                cost: 100,
                ..default()
            },
        ),
        (
            ItemId::GoaleeGear,
            Item {
                category: Upgrade(vec![ItemId::HockeyPads, ItemId::HockeyPads]),
                explanation:
                    "Increases health and removes chip damage on block\n\nI'm fucking <title card>!"
                        .into(),
                cost: 100,
                effect: Stats {
                    chip_damage: false,
                    ..Stats::identity()
                },
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
                cost: 10*id,
                ..default()
            },
        )
    }))
}
