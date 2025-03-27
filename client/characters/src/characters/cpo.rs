use bevy::{prelude::*, utils::HashMap};

use foundation::{
    ActionId, Animation, AnimationType, Area, CPOAction, CPOAnimation, GameButton, Icon, ItemId,
    Model, Sound, Stats, StatusCondition, VoiceLine, CPO_ALT_SHIRT_COLOR, CPO_ALT_SOCKS_COLOR,
};

use crate::{
    items::{universal_item_actions, universal_items},
    jumps, Action, ActionEvent, AttackBuilder, CharacterBoxes, CharacterStateBoxes,
    CharacterUniversals, ConsumableType, DashBuilder, HitBuilder, Item, ItemCategory, Movement,
    ThrowEffectBuilder,
};

use super::Character;

const CHARACTER_UNIVERSALS: CharacterUniversals = CharacterUniversals {
    normal_grunt: Sound::MaleGrunt,
};

pub fn cpo() -> Character {
    // TODO: I eyeballed some bigger numbers here
    let (jumps, gravity) = jumps(1.8, 1.2, Animation::CPO(CPOAnimation::Jump));

    Character::new(
        Model::CPO,
        Sound::Motivation, // TODO: Theme music
        vec![
            // Jacket has a texture which makes it hard
            ("Shirt", CPO_ALT_SHIRT_COLOR),
            ("Sleeves", CPO_ALT_SHIRT_COLOR),
            ("Socks", CPO_ALT_SOCKS_COLOR),
        ]
        .into_iter()
        .collect(),
        cpo_anims(),
        cpo_moves(jumps),
        cpo_items(),
        cpo_boxes(),
        Stats {
            // TODO: Check values
            walk_speed: 1.2,
            back_walk_speed_multiplier: 0.8,
            kunais: 2,
            gravity,
            ..Stats::character_default()
        },
        vec![],
        vec![
            (VoiceLine::Defeat, Sound::MaleNo),
            (VoiceLine::BigHit, Sound::MaleArgh),
            (VoiceLine::SmallHit, Sound::MalePain),
        ]
        .into_iter()
        .collect(),
    )
}

fn cpo_anims() -> HashMap<AnimationType, Animation> {
    vec![
        (AnimationType::AirIdle, CPOAnimation::IdleAir),
        (AnimationType::AirStun, CPOAnimation::HitAir),
        (AnimationType::StandIdle, CPOAnimation::IdleStand),
        (AnimationType::StandBlock, CPOAnimation::BlockStand),
        (AnimationType::StandStun, CPOAnimation::HitStand),
        (AnimationType::WalkBack, CPOAnimation::WalkBack),
        (AnimationType::WalkForward, CPOAnimation::WalkForward),
        (AnimationType::CrouchIdle, CPOAnimation::IdleCrouch),
        (AnimationType::CrouchBlock, CPOAnimation::BlockCrouch),
        (AnimationType::CrouchStun, CPOAnimation::HitCrouch),
        (AnimationType::Getup, CPOAnimation::Getup),
        (AnimationType::Default, CPOAnimation::NeutralStandPose),
    ]
    .into_iter()
    .map(|(k, v)| (k, Animation::from(v)))
    .collect()
}

fn cpo_moves(jumps: impl Iterator<Item = (ActionId, Action)>) -> HashMap<ActionId, Action> {
    jumps
        .chain(dashes())
        .chain(item_actions())
        .chain(
            normals()
                .chain(throws())
                .chain(specials())
                .map(|(k, v)| (ActionId::CPO(k), v)),
        )
        .collect()
}

fn dashes() -> impl Iterator<Item = (ActionId, Action)> {
    [
        // Grounded forward dash
        DashBuilder::forward()
            .with_animation(CPOAnimation::DashGroundForward)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(
                0,
                Movement {
                    amount: Vec2::X * 2.0,
                    duration: 4,
                },
            )
            .on_frame(5, Movement::impulse(Vec2::new(2.0, 5.0)))
            .end_at(20)
            .build(),
        // Grounded back dash
        DashBuilder::back()
            .with_animation(CPOAnimation::DashGroundBack)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(0, Movement::impulse(Vec2::X * 6.9))
            .end_at(20)
            .build(),
        // Air forward dash
        DashBuilder::forward()
            .air_only()
            .with_animation(CPOAnimation::DashAirForward)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(0, Movement::impulse(Vec2::X * 3.0))
            .end_at(20)
            .build(),
        // Air back dash
        DashBuilder::back()
            .air_only()
            .with_animation(CPOAnimation::DashAirBack)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(0, Movement::impulse(Vec2::X * 3.0))
            .end_at(20)
            .build(),
    ]
    .into_iter()
    .flatten()
}

fn normals() -> impl Iterator<Item = (CPOAction, Action)> {
    debug!("CPO normals");

    vec![].into_iter()
}

fn throws() -> impl Iterator<Item = (CPOAction, Action)> {
    debug!("CPO throws");

    let (forward_throw_recipient, forward_throw_hit) = ThrowEffectBuilder::new(
        CPOAnimation::ThrowGroundHit,
        80,
        CPOAnimation::ThrowGroundForwardRecipient,
        30,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(-2.0, 6.0))
    .build();

    let (back_throw_recipient, _) = ThrowEffectBuilder::new(
        CPOAnimation::ThrowGroundHit,
        80,
        CPOAnimation::ThrowGroundBackRecipient,
        30,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(5.0, 2.0))
    .with_extra_target_events(vec![ActionEvent::Teleport(Vec2::new(2.0, 1.0))])
    .build();

    let (air_throw_recipient, air_throw_hit) = ThrowEffectBuilder::new(
        CPOAnimation::ThrowAirHit,
        50,
        CPOAnimation::ThrowAirRecipient,
        50,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(-2.0, 2.0))
    .build();

    vec![
        (
            CPOAction::ForwardThrowStartup,
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_total_duration(37)
                .with_animation(CPOAnimation::ThrowGroundStartup)
                .with_extra_initial_events(vec![ActionEvent::Condition(StatusCondition::kara_to(
                    vec![ActionId::GiParry],
                ))])
                .with_hit_on_frame(
                    3,
                    HitBuilder::normal()
                        .forward_throw()
                        .with_active_frames(3)
                        .throw_hit_action(CPOAction::GroundThrowHit)
                        .throw_target_action(CPOAction::ForwardThrowRecipient)
                        .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5)),
                )
                .build(),
        ),
        (
            CPOAction::BackThrowStartup,
            AttackBuilder::normal()
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_input("{4}w")
                .with_animation(CPOAnimation::ThrowGroundStartup)
                .with_total_duration(37)
                .with_hit_on_frame(
                    3,
                    HitBuilder::normal()
                        .back_throw()
                        .with_active_frames(3)
                        .throw_hit_action(CPOAction::GroundThrowHit)
                        .throw_target_action(CPOAction::BackThrowRecipient)
                        .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5)),
                )
                .build(),
        ),
        (CPOAction::GroundThrowHit, forward_throw_hit),
        (CPOAction::ForwardThrowRecipient, forward_throw_recipient),
        (CPOAction::BackThrowRecipient, back_throw_recipient),
        (
            CPOAction::AirThrowStartup,
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .air_only()
                .with_animation(CPOAnimation::ThrowAirStartup)
                .with_total_duration(40)
                .with_hit_on_frame(
                    4,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .forward_throw()
                        .throw_hit_action(CPOAction::AirThrowHit)
                        .throw_target_action(CPOAction::AirThrowRecipient)
                        .with_hitbox(Area::new(0.4, 0.8, 0.4, 0.4)),
                )
                .build(),
        ),
        (CPOAction::AirThrowHit, air_throw_hit),
        (CPOAction::AirThrowRecipient, air_throw_recipient),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (CPOAction, Action)> {
    debug!("CPO specials");
    vec![].into_iter()
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    universal_item_actions(
        Animation::CPO(CPOAnimation::GiParry),
        Animation::CPO(CPOAnimation::RC),
    )
}

fn cpo_items() -> HashMap<ItemId, Item> {
    vec![
        (
            ItemId::IceCube,
            Item {
                cost: 400,
                explanation: "First hit of 2h against airborne opponent freezes their momentum.\n\nLand this for a good day".into(),
                category: ItemCategory::Basic,
                icon: Icon::IceCube,
                ..default()
            },
        ),
        (
            ItemId::SpareKunai,
            Item {
                cost: 250,
                explanation: "Three is better than two".into(),
                category: ItemCategory::Basic,
                icon: Icon::Kunai,
                effect: Stats {
                    kunais: 1,
                    ..default()
                },
                suggested: true,
                ..default()
            },
        ),
        (
            ItemId::KunaiPouch,
            Item {
                cost: 400,
                explanation: "5 uses for Kunai.\n\nThe more the merrier".into(),
                category: ItemCategory::Upgrade(vec![ItemId::SpareKunai]),
                icon: Icon::KunaiPouch,
                effect: Stats {
                    kunais: 2,
                    ..default()
                },
                suggested: true,
                ..default()
            },
        ),
        (
            ItemId::KunaiBelt,
            Item {
                cost: 1000,
                explanation: "8 uses for Kunai.\n\n8 is perfection.".into(),
                category: ItemCategory::Upgrade(vec![ItemId::KunaiPouch]),
                icon: Icon::KunaiBelt,
                effect: Stats {
                    kunais: 3,
                    ..default()
                },
                suggested: true,
                ..default()
            },
        ),
        (
            ItemId::MiniTasers,
            Item {
                cost: 400,
                explanation: "Adds a shock effect to kunais (more stun)".into(),
                category: ItemCategory::Basic,
                icon: Icon::Taser,
                ..default()
            },
        ),
        (
            ItemId::Protractor,
            Item {
                cost: 250,
                explanation: "Stick position influences Kunai velocity\n\n. It's about angles."
                    .into(),
                category: ItemCategory::Basic,
                icon: Icon::Protractor,
                ..default()
            },
        ),
        (
            ItemId::BladeOil,
            Item {
                category: ItemCategory::Consumable(ConsumableType::OneRound),
                explanation: "Retain sharpness from the previous round.".into(),
                cost: 100,
                icon: Icon::BladeOil,
                effect: Stats {
                    retain_sharpness: true,
                    ..default()
                },
                ..default()
            },
        ),
        (
            ItemId::SmithyCoupon,
            Item {
                category: ItemCategory::Consumable(ConsumableType::OneRound),
                explanation: "Pre-sharpen the sword by two levels".into(),
                cost: 100,
                icon: Icon::SmithyCoupon,
                effect: Stats {
                    auto_sharpen: 2,
                    ..default()
                },
                ..default()
            },
        ),
        (
            ItemId::Fireaxe,
            Item {
                category: ItemCategory::Basic,
                explanation: "Release stance while holding forward to do an overhead".into(),
                cost: 400,
                icon: Icon::Fireaxe,
                ..default()
            },
        ),
        (
            ItemId::SmokeBomb,
            Item {
                category: ItemCategory::Basic,
                explanation: "Dash in sword stance".into(),
                cost: 1000,
                icon: Icon::SmokeBomb,
                suggested: true,
                ..default()
            },
        ),
    ]
    .into_iter()
    .chain(universal_items())
    .collect()
}

fn cpo_boxes() -> CharacterBoxes {
    CharacterBoxes {
        standing: CharacterStateBoxes {
            head: Area::new(-0.05, 1.8, 0.4, 0.3),
            chest: Area::new(0.0, 1.3, 0.6, 0.8),
            legs: Area::new(0.05, 0.6, 0.65, 1.2),
            pushbox: Area::from_center_size(Vec2::Y * 0.7, Vec2::new(0.4, 1.4)),
        },
        crouching: CharacterStateBoxes {
            head: Area::new(0.2, 0.6, 0.4, 0.3),
            chest: Area::new(0.1, 0.45, 0.6, 0.3),
            legs: Area::new(0.0, 0.2, 1.0, 0.4),
            pushbox: Area::from_center_size(Vec2::new(0.1, 0.35), Vec2::new(0.6, 0.7)),
        },
        airborne: CharacterStateBoxes {
            head: Area::new(0.15, 1.25, 0.4, 0.3),
            chest: Area::new(0.1, 0.9, 1.1, 0.6),
            legs: Area::new(-0.2, 0.4, 0.9, 0.8),
            pushbox: Area::from_center_size(Vec2::new(0.0, 0.55), Vec2::new(0.4, 0.6)),
        },
    }
}
