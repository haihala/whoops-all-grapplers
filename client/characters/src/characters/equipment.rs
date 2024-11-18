use bevy::prelude::*;
use wag_core::{
    ActionCategory, ActionId, Animation, CancelType, CancelWindow, Icon, ItemId, Stats,
    StatusCondition, StatusFlag, GI_PARRY_FLASH_COLOR,
};

use crate::{
    actions::ActionRequirement, Action, ActionBuilder, ActionEvent, ConsumableType::*, Item,
    ItemCategory::*, Movement,
};

pub fn gi_parry(animation: Animation) -> Action {
    ActionBuilder::for_category(ActionCategory::Other)
        .with_input("{6}(fs)")
        .immediate_events(vec![
            animation.into(),
            ActionEvent::ForceStand,
            ActionEvent::Condition(StatusCondition {
                flag: StatusFlag::Parry,
                effect: None,
                expiration: Some(10),
            }),
            ActionEvent::Flash(GI_PARRY_FLASH_COLOR.into()),
            ActionEvent::AllowCancel(CancelWindow {
                cancel_type: CancelType::Anything,
                require_hit: true,
                duration: 25,
            }),
        ])
        .end_at(30)
        .with_requirement(ActionRequirement::ItemOwned(ItemId::Gi))
        .build()
}

pub fn fast_fall() -> Action {
    ActionBuilder::for_category(ActionCategory::Other)
        .with_input("{5}[123]")
        .air_only()
        .immediate_events(vec![Movement::impulse(Vec2::Y * -1.5).into()])
        .end_at(10)
        .with_requirement(ActionRequirement::ItemOwned(ItemId::DivingHelmet))
        .build()
}

pub fn universal_item_actions(
    parry_animation: Animation,
) -> impl Iterator<Item = (ActionId, Action)> {
    vec![
        (ActionId::GiParry, gi_parry(parry_animation)),
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
                cost: 100,
                explanation: "Start with 50 meter\n\nGotta get that pump".into(),
                effect: Stats {
                    starting_meter: 50,
                    ..Stats::identity()
                },
                category: Consumable(OneRound),
                icon: Icon::PreWorkout,
            },
        ),
        // Basics
        (
            ItemId::Gi,
            Item {
                cost: 400,
                explanation: "Forward+f+s to parry, cancels to anything on success.\n\nLesgo justin".into(),
                icon: Icon::Gi,
                ..default()
            },
        ),
        (
            ItemId::Boots,
            Item {
                cost: 400,
                explanation: "Bonus walk speed".into(),
                effect: Stats {
                    walk_speed: 0.3,
                    ..Stats::identity()
                },
                icon: Icon::Boots,
                ..default()
            },
        ),
        (
            ItemId::HockeyPads,
            Item {
                cost: 250,
                explanation: "Bonus max health\n\nI am wearing hockey pads".into(),
                effect: Stats {
                    max_health: 30,
                    ..Stats::identity()
                },
                icon: Icon::HockeyPads,
                ..default()
            },
        ),
        (
            ItemId::RedPaint,
            Item {
                cost: 1000,
                explanation: "Increased animation speed\n\nBecause red makes you go fastah".into(),
                effect: Stats {
                    action_speed_multiplier: 1.3,
                    ..Stats::identity()
                },
                icon: Icon::RedPaint,
                ..default()
            },
        ),
        (
            ItemId::Stopwatch,
            Item {
                cost: 400,
                explanation: "Gain meter over time\n\nTick tock.".into(),
                effect: Stats {
                    meter_per_second: 2.0,
                    ..Stats::identity()
                },
                icon: Icon::Stopwatch,
                ..default()
            },
        ),
        (
            ItemId::Crowbar,
            Item {
                cost: 250,
                explanation: "Gain bonus stun frames, meter gain and damage on the opening hit of a combo\n\nBlock this whack ass mixup".into(),
                effect: Stats {
                    opener_damage_multiplier: 1.5,
                    opener_meter_gain: 10,
                    opener_stun_frames: 5,
                    ..Stats::identity()
                },
                icon: Icon::Crowbar,
                ..default()
            },
        ),
        (
            ItemId::OliveOil,
            Item {
                cost: 400,
                explanation: "Slightly control knockback direction while getting comboed\n\nSlippery".into(),
                effect: Stats {
                    direct_influence: 1.0,
                    ..Stats::identity()
                },
                icon: Icon::OliveOil,
                ..default()
            },
        ),
        (
            ItemId::Dumbbell,
            Item {
                cost: 250,
                explanation: "Makes you fall faster when comboed."
                    .into(),
                effect: Stats {
                    gravity_scaling: 0.04,
                    ..Stats::identity()
                },
                icon: Icon::Dumbbell,
                ..default()
            },
        ),
        (
            ItemId::Feather,
            Item {
                cost: 250,
                explanation: "Makes you jump slightly higher\n\nBoing".into(),
                effect: Stats {
                    jump_force_multiplier: 1.02,
                    ..Stats::identity()
                },
                icon: Icon::Feather,
                ..default()
            },
        ),
        (
            ItemId::Cigarettes,
            Item {
                cost: 400,
                explanation: "Makes you intangible for the first few frames of your backdash\n\nDissapear in a puff".into(),
                effect: Stats {
                    backdash_invuln: 5,
                    ..Stats::identity()
                },
                icon: Icon::Cigarettes,
                ..default()
            },
        ),
        (
            ItemId::ThumbTacks(1),
            Item {
                category: Basic,
                explanation: "+1% damage to all hits\n\nPrickly!".into(),
                cost: 125,
                effect: Stats {
                    damage_multiplier: 1.01,
                    ..Stats::identity()
                },
                icon: Icon::ThumbTacks(1),
            },
        ),
        // Upgrades
        (
            ItemId::TrackSpikes,
            Item {
                category: Upgrade(vec![ItemId::Boots, ItemId::Stopwatch, ItemId::ThumbTacks(2)]),
                explanation: "Allows you to cancel normals into a dash\n\nNow with Fast Action Disruption Compatible soles".into(),
                cost: 1000,
                icon: Icon::TrackSpikes,
                ..default()
            },
        ),
        (
            ItemId::MoonBoots,
            Item {
                category: Upgrade(vec![ItemId::Boots]),
                explanation: "Allows you to crouch before jumping to super jump\n\nBoing.".into(),
                cost: 400,
                icon: Icon::SpaceSuitBoots,
                ..default()
            },
        ),
        (
            ItemId::FeatheredBoots,
            Item {
                category: Upgrade(vec![ItemId::Feather, ItemId::Boots]),
                explanation: "Allows you to double jump".into(),
                cost: 1000,
                icon: Icon::PigeonWing,
                ..default()
            },
        ),
        (
            ItemId::DivingHelmet,
            Item {
                category: Upgrade(vec![ItemId::Dumbbell]),
                explanation: "Allows you to tap down to fast fall\n\nHiyaa!".into(),
                cost: 400,
                icon: Icon::DivingHelmet,
                ..default()
            },
        ),
        (
            ItemId::GoalieGear,
            Item {
                category: Upgrade(vec![ItemId::HockeyPads]),
                explanation:
                    "Increases health, removes chip damage on block and rewards blocking with meter\n\nJust try to break me"
                        .into(),
                cost: 400,
                effect: Stats {
                    max_health: 10,
                    chip_damage: false,
                    defense_meter: 1,
                    ..Stats::identity()
                },
                icon: Icon::GoalieGear,
            },
        ),
    ]
    .into_iter()
    .chain((2..9).map(|id| {
        let exponential = usize::pow(2, (id - 1) as u32);
        (
            ItemId::ThumbTacks(id),
            Item {
                category: Upgrade(vec![ItemId::ThumbTacks(id - 1)]),
                explanation: format!(
                    "+{}% damage to all hits. Stacks multiplicatively with previous upgrades.",
                    exponential
                ),
                cost: 125*exponential,
                icon: Icon::ThumbTacks(id),
                effect: Stats {
                    damage_multiplier: 1.0 + (exponential as f32 * 0.01),
                    ..default()
                },
            },
        )
    }))
}
