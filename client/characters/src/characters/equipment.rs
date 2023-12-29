use bevy::prelude::*;
use wag_core::{
    ActionId, Animation, ItemId, Stats, StatusCondition, StatusFlag, GI_PARRY_FLASH_COLOR,
};

use crate::{
    actions::ActionRequirement, Action, ActionBlock, ActionEvent, CancelCategory, CancelRule,
    ConsumableType::*, ContinuationRequirement, Item, ItemCategory::*, Movement,
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
            cancel_policy: CancelRule::never(),
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
        CancelCategory::Uncancellable,
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

fn fast_fall() -> Action {
    Action::new(
        Some("[456789][123]"),
        CancelCategory::Uncancellable,
        vec![ActionBlock {
            events: vec![Movement::impulse(Vec2::Y * -1.5).into()],
            exit_requirement: ContinuationRequirement::Time(10),
            cancel_policy: CancelRule::any(),
            ..default()
        }],
        vec![
            ActionRequirement::Airborne,
            ActionRequirement::ItemsOwned(vec![ItemId::Flyweight]),
        ],
    )
}

pub fn universal_item_actions(
    parry_animation: Animation,
) -> impl Iterator<Item = (ActionId, Action)> {
    vec![
        (ActionId::HighGiParry, get_high_gi_parry()),
        (ActionId::ParryFlash, parry_flash(parry_animation)),
        (ActionId::FastFall, fast_fall()),
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
                cost: 300,
                explanation: "Tap forward to parry\n\nLesgo justin".into(),
                ..default()
            },
        ),
        (
            ItemId::Boots,
            Item {
                cost: 260,
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
                cost: 150,
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
                cost: 1200,
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
                cost: 200,
                explanation: "Gain meter for timing links well\n\nNot my tempo!".into(),
                effect: Stats {
                    link_bonus: 5,
                    ..Stats::identity()
                },
                ..default()
            },
        ),
        (
            ItemId::Crowbar,
            Item {
                cost: 300,
                explanation: "Gain bonus stun frames, meter gain and damage on the opening hit of a combo\n\nBlock this whack ass mixup".into(),
                effect: Stats {
                    opener_damage_multiplier: 1.5,
                    opener_meter_gain: 10,
                    opener_stun_frames: 5,
                    ..Stats::identity()
                },
                ..default()
            },
        ),
        (
            ItemId::Dumbbell,
            Item {
                cost: 350,
                explanation: "Makes you ever so slightly heavier\n\nNot for training purposes"
                    .into(),
                effect: Stats {
                    gravity: 0.02,
                    ..Stats::identity()
                },
                ..default()
            },
        ),
        (
            ItemId::Feather,
            Item {
                cost: 320,
                explanation: "Makes you jump slightly higher\n\nBoing".into(),
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
                cost: 700,
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
                explanation: "+1% damage to all hits\n\nOuch".into(),
                cost: 100,
                effect: Stats {
                    damage_multiplier: 1.01,
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
                cost: 200,
                ..default()
            },
        ),
        (
            ItemId::TrackSpikes,
            Item {
                category: Upgrade(vec![ItemId::Boots, ItemId::Stopwatch]),
                explanation: "Allows you to cancel normals into a dash\n\nNow with Fast Action Disruption Compatible soles".into(),
                cost: 500,
                ..default()
            },
        ),
        (
            ItemId::FeatheredBoots,
            Item {
                category: Upgrade(vec![ItemId::Boots, ItemId::Feather]),
                explanation: "Allows you to jump from crouch to super jump\n\nLike Hermes.".into(),
                cost: 400,
                ..default()
            },
        ),
        (
            ItemId::PidgeonWing,
            Item {
                category: Upgrade(vec![ItemId::Feather, ItemId::Feather]),
                explanation: "Allows you to double jump\n\nPidgeon flap!".into(),
                cost: 700,
                ..default()
            },
        ),        (
            ItemId::Flyweight,
            Item {
                category: Upgrade(vec![ItemId::Feather, ItemId::Dumbbell]),
                explanation: "Allows you to tap down to fast fall\n\nHiyaa!".into(),
                cost: 600,
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
                cost: 300,
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
                    "+{}% damage to all hits\n\nExponential growth is fun!",
                    usize::pow(2, (id - 1) as u32)
                ),
                cost: 10*(id-1),
                ..default()
            },
        )
    }))
}
