use std::iter::empty;

use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionCategory, ActionId, Animation, AnimationType, Area, DummyActionId, DummyAnimation,
    ItemId, Model, Stats, StatusCondition, StatusFlag, CHARGE_BAR_FULL_SEGMENT_COLOR,
    CHARGE_BAR_PARTIAL_SEGMENT_COLOR, FPS,
};

use crate::{
    actions::ActionRequirement,
    dashes,
    resources::{RenderInstructions, ResourceType},
    Action,
    ActionEvent::{self, *},
    Attack, AttackBuilder, CharacterBoxes, CharacterStateBoxes, ChargeProperty, Hitbox,
    IntermediateStrike, Item,
    ItemCategory::*,
    Lifetime, Movement, ResourceBarVisual, Situation, SpecialProperty, ToHit, WAGResource,
};

use super::{
    equipment::{universal_item_actions, universal_items},
    helpers::jumps,
    Character,
};

// Honestly, this character shouldn't really see use, but keep it around for testing
// So it's meant to just be able to compile.
// Could go back on that if that proves to be too much of a hassle
pub fn dummy() -> Character {
    let (jumps, gravity) = jumps(2.0, 1.0, Animation::Dummy(DummyAnimation::Jump));

    Character::new(
        Model::Dummy,
        HashMap::new(),
        dummy_animations(),
        dummy_moves(jumps),
        dummy_items(),
        dummy_boxes(),
        Stats {
            gravity,
            ..default()
        },
        vec![(
            ResourceType::Charge,
            WAGResource {
                max: Some(FPS as i32), // Frames to full,
                special: Some(SpecialProperty::Charge(ChargeProperty::default())),
                render_instructions: RenderInstructions::Bar(ResourceBarVisual {
                    default_color: CHARGE_BAR_PARTIAL_SEGMENT_COLOR,
                    full_color: Some(CHARGE_BAR_FULL_SEGMENT_COLOR),
                    ..default()
                }),
                ..default()
            },
        )],
        HashMap::new(),
    )
}

fn dummy_animations() -> HashMap<AnimationType, Animation> {
    vec![
        (AnimationType::AirIdle, DummyAnimation::AirIdle),
        (AnimationType::AirStun, DummyAnimation::AirStun),
        (AnimationType::StandIdle, DummyAnimation::Idle),
        (AnimationType::StandBlock, DummyAnimation::StandBlock),
        (AnimationType::StandStun, DummyAnimation::StandStun),
        (AnimationType::WalkBack, DummyAnimation::WalkBack),
        (AnimationType::WalkForward, DummyAnimation::WalkForward),
        (AnimationType::CrouchIdle, DummyAnimation::Crouch),
        (AnimationType::CrouchBlock, DummyAnimation::CrouchBlock),
        (AnimationType::CrouchStun, DummyAnimation::CrouchStun),
        (AnimationType::Getup, DummyAnimation::Getup),
        (AnimationType::Default, DummyAnimation::TPose),
    ]
    .into_iter()
    .map(|(k, v)| (k, Animation::Dummy(v)))
    .collect()
}

// Dashing

fn dummy_moves(jumps: impl Iterator<Item = (ActionId, Action)>) -> HashMap<ActionId, Action> {
    empty()
        .chain(item_actions())
        .chain(dashes!(
            DummyAnimation::DashForward,
            DummyAnimation::DashBack
        ))
        .chain(jumps)
        .chain(
            normals()
                .chain(specials())
                .map(|(k, v)| (ActionId::Dummy(k), v)),
        )
        .collect()
}

fn normals() -> impl Iterator<Item = (DummyActionId, Action)> {
    vec![
        (
            DummyActionId::Slap,
            AttackBuilder::normal("f")
                .with_frame_data(9, 5, 10)
                .with_hitbox(Area::new(0.5, 1.0, 0.35, 0.35))
                .with_animation(DummyAnimation::Slap)
                .build(),
        ),
        (
            DummyActionId::LowChop,
            AttackBuilder::normal("[123]f")
                .with_animation(DummyAnimation::CrouchChop)
                .with_frame_data(8, 5, 7)
                .with_hitbox(Area::new(0.4, 0.1, 0.3, 0.2))
                .build(),
        ),
        (
            DummyActionId::BurnStraight,
            Action {
                input: Some("s"),
                category: ActionCategory::Normal,
                script: Box::new(|situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![DummyAnimation::BurnStraight.into()];
                    }

                    if situation.elapsed() == 10 {
                        let has_roids = situation.inventory.contains(&ItemId::Roids);

                        return vec![
                            Attack {
                                to_hit: ToHit {
                                    hitbox: Hitbox(Area::new(0.6, 1.1, 1.0, 0.2)),
                                    lifetime: Lifetime::frames(8),
                                    ..default()
                                },
                                ..IntermediateStrike {
                                    base_damage: 20,
                                    hit_stun_event: ActionEvent::HitStun(20),
                                    attacker_push_on_hit: if has_roids { 1.0 } else { 2.0 },
                                    attacker_push_on_block: if has_roids { 0.0 } else { 3.0 },
                                    ..default()
                                }
                                .build_attack(situation)
                            }
                            .into(),
                            Movement {
                                amount: Vec2::X * 2.0,
                                duration: 1,
                            }
                            .into(),
                        ];
                    }

                    situation.end_at(20)
                }),
                requirements: vec![],
            },
        ),
        (
            DummyActionId::AntiAir,
            AttackBuilder::normal("[123]s")
                .with_animation(DummyAnimation::AntiAir)
                .with_frame_data(13, 4, 13)
                .with_hitbox(Area::new(1.0, 1.3, 0.3, 0.5))
                .build(),
        ),
        (
            DummyActionId::AirSlap,
            AttackBuilder::normal("f")
                .air_only()
                .with_animation(DummyAnimation::AirSlap)
                .with_frame_data(8, 5, 10)
                .with_hitbox(Area::new(0.3, 0.6, 0.35, 0.25))
                .build(),
        ),
        (
            DummyActionId::Divekick,
            AttackBuilder::normal("s")
                .air_only()
                .with_animation(DummyAnimation::Divekick)
                .with_frame_data(5, 10, 15)
                .with_hitbox(Area::new(0.6, -0.2, 0.35, 0.25))
                .with_extra_requirements(vec![ActionRequirement::ItemsOwned(vec![ItemId::Boots])])
                .build(),
        ),
        (
            DummyActionId::Sweep,
            AttackBuilder::normal("[123]w")
                .with_animation(DummyAnimation::Sweep)
                .with_frame_data(10, 5, 15)
                .launches(Vec2::new(1.0, 8.0))
                .with_hitbox(Area::new(0.6, 0.1, 1.0, 0.2))
                .build(),
        ),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (DummyActionId, Action)> {
    vec![
        (
            DummyActionId::Dodge,
            Action {
                input: Some("252"),
                category: ActionCategory::Special,
                script: Box::new(|situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![
                            ForceStand,
                            DummyAnimation::Dodge.into(),
                            Condition(StatusCondition {
                                flag: StatusFlag::Intangible,
                                effect: None,
                                expiration: Some(20),
                            }),
                        ];
                    }

                    situation.end_at(45)
                }),
                requirements: vec![],
            },
        ),
        (
            DummyActionId::GroundSlam,
            AttackBuilder::special("[789]6s")
                .with_animation(DummyAnimation::GroundSlam)
                .with_frame_data(14, 8, 20)
                .with_hitbox(Area::new(0.7, 1.25, 0.8, 0.8))
                .with_damage(20)
                .with_extra_activation_events(vec![Movement {
                    amount: Vec2::X * 2.0,
                    duration: 1,
                }
                .into()])
                .build(),
        ),
        (
            DummyActionId::AirSlam,
            AttackBuilder::special("[789]6s")
                .air_only()
                .with_animation(DummyAnimation::AirSlam)
                .with_frame_data(14, 8, 35)
                .with_hitbox(Area::new(0.9, 1.25, 0.8, 0.8))
                .with_damage(20)
                .with_extra_activation_events(vec![Movement {
                    amount: Vec2::X * 1.0,
                    duration: 3,
                }
                .into()])
                .build(),
        ),
        (
            DummyActionId::BudgetBoom,
            AttackBuilder::special("[41]6f")
                .with_projectile(Model::Fireball, 5.0 * Vec2::X)
                .with_frame_data(10, 15, 4)
                .with_hitbox(Area::new(0.5, 1.2, 0.3, 0.2))
                .with_damage(20)
                .with_extra_activation_events(vec![Movement {
                    amount: Vec2::X * 1.0,
                    duration: 3,
                }
                .into()])
                .build(),
        ),
        (
            DummyActionId::SonicBoom,
            AttackBuilder::special("[41]6f")
                .if_charged()
                .with_projectile(Model::Fireball, 6.0 * Vec2::X)
                .with_timings(10, 5)
                .with_hitbox(Area::new(0.5, 1.2, 0.4, 0.3))
                .with_damage(10)
                .with_multiple_hits(3)
                .with_extra_activation_events(vec![Movement {
                    amount: Vec2::X * 1.0,
                    duration: 3,
                }
                .into()])
                .build(),
        ),
        (
            DummyActionId::Hadouken,
            AttackBuilder::special("236f")
                .with_frame_data(30, 3600, 30)
                .with_projectile(Model::Fireball, 4.0 * Vec2::X)
                .with_hitbox(Area::new(0.5, 1.0, 0.3, 0.3))
                .with_multiple_hits(3)
                .build(),
        ),
        (
            DummyActionId::HeavyHadouken,
            AttackBuilder::special("236s")
                .with_meter_cost(30)
                .with_timings(30, 30)
                .with_projectile(Model::Fireball, 5.0 * Vec2::X)
                .with_hitbox(Area::new(0.5, 1.0, 0.4, 0.5))
                .with_multiple_hits(2)
                .build(),
        ),
    ]
    .into_iter()
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    empty().chain(universal_item_actions(Animation::Dummy(
        DummyAnimation::TPose,
    )))
}

fn dummy_items() -> HashMap<ItemId, Item> {
    vec![(
        ItemId::Roids,
        Item {
            cost: 100,
            category: Consumable(crate::items::ConsumableType::OneRound),
            explanation: "Get yoked".into(),
            effect: Stats {
                action_speed_multiplier: 1.1,
                ..Stats::identity()
            },
            ..default()
        },
    )]
    .into_iter()
    .chain(universal_items())
    .collect()
}

fn dummy_boxes() -> CharacterBoxes {
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
