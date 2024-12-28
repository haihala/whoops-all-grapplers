use bevy::prelude::*;
use foundation::{
    ActionCategory, ActionId, Animation, CancelType, Icon, ItemId, Stats, StatusCondition,
    StatusFlag, VfxRequest, VisualEffect, GI_PARRY_FLASH_COLOR, RC_PULSE_BASE_COLOR,
    RC_PULSE_EDGE_COLOR,
};

use crate::{
    actions::ActionRequirement, Action, ActionBuilder, ActionEvent, ConsumableType::*, Item,
    ItemCategory::*, Movement,
};

fn gi_parry(animation: Animation) -> Action {
    ActionBuilder::for_category(ActionCategory::Other)
        .with_input("{6}(gw)")
        .static_immediate_events(vec![
            animation.into(),
            ActionEvent::ForceStand,
            ActionEvent::Condition(StatusCondition {
                flag: StatusFlag::Parry,
                effect: None,
                expiration: Some(10),
            }),
            ActionEvent::Flash(GI_PARRY_FLASH_COLOR.into()),
            ActionEvent::Condition(StatusCondition {
                flag: StatusFlag::Cancel(CancelType::Anything),
                expiration: Some(25),
                ..default()
            }),
        ])
        .end_at(30)
        .with_requirement(ActionRequirement::ItemOwned(ItemId::Gi))
        .build()
}

fn fast_fall() -> Action {
    ActionBuilder::for_category(ActionCategory::Other)
        .with_input("{5}[123]")
        .air_only()
        .static_immediate_events(vec![Movement::impulse(Vec2::Y * -1.5).into()])
        .end_at(10)
        .with_requirement(ActionRequirement::ItemOwned(ItemId::DivingHelmet))
        .build()
}

fn romaine_cancel(animation: Animation) -> Action {
    ActionBuilder::for_category(ActionCategory::MegaInterrupt)
        .air_or_ground()
        .with_input("(gw)")
        .with_requirement(ActionRequirement::ItemOwned(ItemId::RomaineLettuce))
        .with_requirement(ActionRequirement::AnyActionOngoing)
        .end_at(10)
        .with_meter_cost()
        .static_immediate_events(vec![
            animation.into(),
            ActionEvent::RelativeVisualEffect(VfxRequest {
                effect: VisualEffect::RingPulse(RC_PULSE_BASE_COLOR, RC_PULSE_EDGE_COLOR),
                tf: Transform::from_translation(Vec3::Y),
                ..default()
            }),
            ActionEvent::RelativeVisualEffect(VfxRequest {
                effect: VisualEffect::Icon(Icon::Lettuce),
                tf: Transform::from_translation(Vec3::Y),
                ..default()
            }),
        ])
        .build()
}

pub fn universal_item_actions(
    parry_animation: Animation,
    rc_animation: Animation,
) -> impl Iterator<Item = (ActionId, Action)> {
    vec![
        (ActionId::GiParry, gi_parry(parry_animation)),
        (ActionId::RomaineCancel, romaine_cancel(rc_animation)),
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
                    ..default()
                },
                category: Consumable(OneRound),
                icon: Icon::PreWorkout,
                ..default()
            },
        ),
        // Basics
        (
            ItemId::Gi,
            Item {
                cost: 400,
                explanation: "Forward+g+w to parry, cancels to anything on success.\n\nLesgo justin".into(),
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
                    ..default()
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
                    ..default()
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
                    ..default()
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
                    ..default()
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
                    ..default()
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
                    ..default()
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
                    ..default()
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
                    ..default()
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
                    ..default()
                },
                icon: Icon::Cigarettes,
                ..default()
            },
        ),
        (
            ItemId::ThumbTacks,
            Item {
                category: Basic,
                explanation: "+1% damage to all hits\n\nPrickly!".into(),
                cost: 125,
                effect: Stats {
                    damage_multiplier: 1.01,
                    ..default()
                },
                icon: Icon::ThumbTack,
                max_stack: 10,
            },
        ),
        (
            ItemId::ComicBook,
            Item {
                category: Basic,
                explanation: "Gives you one normal to normal cancel per sequence.".into(),
                cost: 1000,
                icon: Icon::ComicBook,
                ..default()
            },
        ),
        (
            ItemId::RomaineLettuce,
            Item {
                category: Basic,
                explanation: "Press g+w mid-move to cancel ongoing action. Costs bar.".into(),
                cost: 1000,
                icon: Icon::Lettuce,
                ..default()
            },
        ),
        // Upgrades
        (
            ItemId::TrackSpikes,
            Item {
                category: Upgrade(vec![ItemId::Boots, ItemId::Stopwatch, ItemId::ThumbTacks]),
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
                icon: Icon::FeatheredBoots,
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
            ItemId::Wing,
            Item {
                category: Upgrade(vec![ItemId::DivingHelmet, ItemId::Feather]),
                explanation: "Allows you to dash mid air".into(),
                cost: 400,
                icon: Icon::PigeonWing,
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
                    ..default()
                },
                icon: Icon::GoalieGear,
                ..default()
            },
        ),
    ]
    .into_iter()
}
