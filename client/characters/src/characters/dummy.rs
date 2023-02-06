use std::{collections::HashMap, iter::empty};

use bevy::prelude::*;

use wag_core::{
    Animation, AnimationType, Area, DummyAnimation, ItemId, Model, MoveId, Status, StatusCondition,
};

use crate::{
    moves::{
        airborne, crouching, grounded, standing, Action::*, Attack, CancelPolicy::*,
        CommonAttackProps, FlowControl::*, MoveType::*, Movement, Projectile, Situation,
        StunType::*,
    },
    AttackHeight::*,
    BlockType::*,
    Cost, Hitbox, Item,
    ItemCategory::*,
    Lifetime, Move, ToHit,
};

use super::{
    dash,
    equipment::{get_gunshot, get_handmedownken, get_shot},
    Character,
};

pub fn dummy() -> Character {
    Character::new(
        Model::Dummy,
        dummy_animations(),
        dummy_moves(),
        dummy_items(),
        2.0,
        1.0,
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
    ]
    .into_iter()
    .map(|(k, v)| (k, Animation::Dummy(v)))
    .collect()
}

// Dashing
const DASH_DURATION: usize = (0.5 * wag_core::FPS) as usize;
const DASH_IMPULSE: f32 = 10.0;

fn dummy_moves() -> HashMap<MoveId, Move> {
    empty()
        .chain(items())
        .chain(dashes())
        .chain(normals())
        .chain(specials())
        .collect()
}

fn items() -> impl Iterator<Item = (MoveId, Move)> {
    vec![
        (MoveId::HandMeDownKen, get_handmedownken()),
        (MoveId::Gunshot, get_gunshot()),
        (MoveId::Shoot, get_shot()),
    ]
    .into_iter()
}

fn dashes() -> impl Iterator<Item = (MoveId, Move)> {
    vec![
        (
            MoveId::DashForward,
            dash(
                "656",
                DASH_DURATION,
                DASH_IMPULSE,
                DummyAnimation::DashForward.into(),
            ),
        ),
        (
            MoveId::DashBack,
            dash(
                "454",
                DASH_DURATION,
                -DASH_IMPULSE,
                DummyAnimation::DashBack.into(),
            ),
        ),
    ]
    .into_iter()
}

fn normals() -> impl Iterator<Item = (MoveId, Move)> {
    vec![
        (
            MoveId::Slap,
            Move {
                input: Some("f"),
                requirement: standing,
                phases: vec![
                    DummyAnimation::Slap.into(),
                    Wait(9, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.7, 1.35, 0.35, 0.25)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(10, IfHit),
                ],
                ..default()
            },
        ),
        (
            MoveId::LowChop,
            Move {
                input: Some("f"),
                requirement: crouching,
                phases: vec![
                    DummyAnimation::CrouchChop.into(),
                    Wait(8, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.75, 0.2, 0.3, 0.2)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(7, IfHit),
                ],
                ..default()
            },
        ),
        (
            MoveId::BurnStraight,
            Move {
                input: Some("s"),
                requirement: standing,
                phases: vec![
                    DummyAnimation::BurnStraight.into(),
                    Wait(10, Never),
                    DynamicActions(|situation: Situation| {
                        vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.6, 1.35, 1.0, 0.2)),
                                lifetime: Lifetime::frames(8),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 20,
                                on_hit: Stun(20),
                                knock_back: if situation.inventory.contains(&ItemId::Roids) {
                                    1.0
                                } else {
                                    -3.0
                                } * Vec2::X,
                                push_back: if situation.inventory.contains(&ItemId::Roids) {
                                    0.0
                                } else {
                                    -2.0
                                } * Vec2::X,
                                ..default()
                            },
                        )
                        .into()]
                    }),
                    Movement {
                        amount: Vec2::X * 2.0,
                        duration: 1,
                    }
                    .into(),
                    Wait(10, IfHit),
                ],
                ..default()
            },
        ),
        (
            MoveId::AntiAir,
            Move {
                input: Some("s"),
                requirement: crouching,
                phases: vec![
                    DummyAnimation::AntiAir.into(),
                    Wait(13, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.75, 1.9, 0.3, 0.5)),
                            lifetime: Lifetime::frames(4),
                            ..default()
                        },
                        CommonAttackProps {
                            knock_back: Vec2::new(-4.0, 3.0),
                            ..default()
                        },
                    )
                    .into(),
                    Wait(13, IfHit),
                ],
                ..default()
            },
        ),
        (
            MoveId::AirSlap,
            Move {
                input: Some("f"),
                requirement: airborne,
                phases: vec![
                    DummyAnimation::AirSlap.into(),
                    Wait(8, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.7, 1.3, 0.35, 0.25)),
                            lifetime: Lifetime::frames(5),
                            block_type: Constant(High),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(10, IfHit),
                ],
                ..default()
            },
        ),
        (
            MoveId::Divekick,
            Move {
                input: Some("s"),
                requirement: airborne,
                phases: vec![
                    DummyAnimation::Divekick.into(),
                    Wait(5, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.6, 0.1, 0.35, 0.25)),
                            lifetime: Lifetime::frames(10),
                            block_type: Constant(High),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(10, IfHit),
                ],
                ..default()
            },
        ),
        (
            MoveId::ForwardThrow,
            Move {
                input: Some("g"),
                requirement: standing,
                phases: vec![
                    DummyAnimation::NormalThrow.into(),
                    Wait(5, Never),
                    Attack::new(
                        ToHit {
                            block_type: Grab,
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.5)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        CommonAttackProps {
                            damage: 25,
                            on_hit: Launcher,
                            ..default()
                        },
                    )
                    .with_to_target_on_hit(vec![
                        SnapToOpponent,
                        RecipientAnimation(DummyAnimation::NormalThrowRecipient.into()),
                    ])
                    .into(),
                    Wait(40, Never),
                ],
                ..default()
            },
        ),
        (
            MoveId::BackThrow,
            Move {
                input: Some("4g"),
                requirement: standing,
                phases: vec![
                    DummyAnimation::NormalThrow.into(),
                    Wait(5, Never),
                    Attack::new(
                        ToHit {
                            block_type: Grab,
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.5)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        CommonAttackProps {
                            damage: 25,
                            on_hit: Launcher,
                            ..default()
                        },
                    )
                    .with_to_target_on_hit(vec![
                        SnapToOpponent,
                        SideSwitch,
                        RecipientAnimation(DummyAnimation::NormalThrowRecipient.into()),
                    ])
                    .into(),
                    Wait(40, Never),
                ],
                ..default()
            },
        ),
        (
            MoveId::Sweep,
            Move {
                input: Some("g"),
                requirement: crouching,
                phases: vec![
                    DummyAnimation::Sweep.into(),
                    Wait(10, Never),
                    Attack::new(
                        ToHit {
                            block_type: Grab,
                            hitbox: Hitbox(Area::new(0.7, 0.2, 1.0, 0.2)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        CommonAttackProps {
                            knock_back: Vec2::Y * 8.0,
                            on_hit: Launcher,
                            ..default()
                        },
                    )
                    .into(),
                    Wait(15, IfHit),
                ],
                ..default()
            },
        ),
        (
            MoveId::AirThrow,
            Move {
                input: Some("g"),
                requirement: airborne,
                phases: vec![
                    DummyAnimation::AirThrow.into(),
                    Wait(9, Never),
                    Attack::new(
                        ToHit {
                            block_type: Grab,
                            hitbox: Hitbox(Area::new(0.75, 1.0, 0.8, 0.8)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        CommonAttackProps {
                            damage: 25,
                            on_hit: Launcher,
                            knock_back: Vec2::new(1.0, -4.0),
                            ..default()
                        },
                    )
                    .with_to_target_on_hit(vec![
                        SnapToOpponent,
                        RecipientAnimation(DummyAnimation::AirThrowRecipient.into()),
                    ])
                    .into(),
                    Wait(30, Never),
                ],
                ..default()
            },
        ),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (MoveId, Move)> {
    vec![
        (
            MoveId::Dodge,
            Move {
                input: Some("252"),
                move_type: Normal,
                phases: vec![
                    vec![
                        ForceStand,
                        DummyAnimation::Dodge.into(),
                        Condition(StatusCondition {
                            name: Status::Dodge,
                            effect: None,
                            expiration: Some(20),
                        }),
                    ]
                    .into(),
                    Wait(45, Never),
                ],
                ..default()
            },
        ),
        (
            MoveId::GroundSlam,
            Move {
                input: Some("[789]6s"),
                move_type: Special,
                requirement: grounded,
                phases: vec![
                    DummyAnimation::GroundSlam.into(),
                    Wait(14, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.7, 1.25, 0.8, 0.8)),
                            lifetime: Lifetime::frames(8),
                            ..default()
                        },
                        CommonAttackProps {
                            damage: 20,
                            knock_back: Vec2::Y,
                            push_back: -3.0 * Vec2::X,
                            ..default()
                        },
                    )
                    .into(),
                    Movement {
                        amount: Vec2::X * 2.0,
                        duration: 1,
                    }
                    .into(),
                    Wait(20, IfHit),
                ],
            },
        ),
        (
            MoveId::AirSlam,
            Move {
                input: Some("[789]6s"),
                move_type: Special,
                requirement: airborne,
                phases: vec![
                    DummyAnimation::AirSlam.into(),
                    Wait(14, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.9, 1.25, 0.8, 0.8)),
                            lifetime: Lifetime::frames(8),
                            ..default()
                        },
                        CommonAttackProps {
                            damage: 20,
                            knock_back: -3.0 * Vec2::X,
                            push_back: Vec2::Y,
                            ..default()
                        },
                    )
                    .into(),
                    Movement {
                        amount: Vec2::X * 1.0,
                        duration: 2,
                    }
                    .into(),
                    Wait(35, IfHit),
                ],
            },
        ),
        (
            MoveId::BudgetBoom,
            Move {
                input: Some("[41]6f"),
                requirement: standing,
                move_type: Special,
                phases: vec![
                    ForceStand.into(),
                    Wait(10, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.3, 0.2)),
                            velocity: Some(5.0 * Vec2::X),
                            lifetime: Lifetime::frames((wag_core::FPS * 0.25) as usize),
                            projectile: Some(Projectile {
                                model: Model::Fireball,
                            }),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(5, IfHit),
                ],
            },
        ),
        (
            MoveId::SonicBoom,
            Move {
                input: Some("[41]6f"),
                move_type: Special,
                requirement: |situation: Situation| {
                    situation.resources.can_afford(Cost::charge()) && grounded(situation)
                },
                phases: vec![
                    vec![ForceStand, Pay(Cost::charge())].into(),
                    Wait(10, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.4, 0.3)),
                            velocity: Some(6.0 * Vec2::X),
                            lifetime: Lifetime::until_owner_hit(),
                            projectile: Some(Projectile {
                                model: Model::Fireball,
                            }),
                            hits: 3,
                            ..default()
                        },
                        CommonAttackProps {
                            damage: 10,
                            ..default()
                        },
                    )
                    .into(),
                    Wait(5, IfHit),
                ],
            },
        ),
        (
            MoveId::Hadouken,
            Move {
                input: Some("236f"),
                move_type: Special,
                phases: vec![
                    ForceStand.into(),
                    Wait(30, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                            velocity: Some(4.0 * Vec2::X),
                            lifetime: Lifetime::until_owner_hit(),
                            projectile: Some(Projectile {
                                model: Model::Fireball,
                            }),
                            hits: 3,
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(30, IfHit),
                ],
                ..default()
            },
        ),
        (
            MoveId::HeavyHadouken,
            Move {
                input: Some("236s"),
                move_type: Special,
                requirement: |situation: Situation| situation.resources.can_afford(Cost::meter(30)),
                phases: vec![
                    vec![ForceStand, Pay(Cost::meter(30))].into(),
                    Wait(30, Never),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.4, 0.5)),
                            velocity: Some(5.0 * Vec2::X),
                            lifetime: Lifetime::until_owner_hit(),
                            projectile: Some(Projectile {
                                model: Model::Fireball,
                            }),
                            hits: 2,
                            ..default()
                        },
                        CommonAttackProps {
                            on_hit: Stun(30),
                            ..default()
                        },
                    )
                    .into(),
                    Wait(20, IfHit),
                ],
            },
        ),
    ]
    .into_iter()
}

fn dummy_items() -> HashMap<ItemId, Item> {
    vec![
        (
            ItemId::Roids,
            Item {
                cost: 100,
                category: Consumable,
                ..default()
            },
        ),
        (
            ItemId::HandMeDownKen,
            Item {
                cost: 10,
                ..default()
            },
        ),
        (
            ItemId::Gi,
            Item {
                cost: 100,
                ..default()
            },
        ),
        (
            ItemId::Gun,
            Item {
                cost: 100,
                ..default()
            },
        ),
        (
            ItemId::Boots,
            Item {
                cost: 80,
                ..default()
            },
        ),
        (
            ItemId::SafetyBoots,
            Item {
                category: Upgrade(vec![ItemId::Boots]),
                cost: 100,
                ..default()
            },
        ),
    ]
    .into_iter()
    .collect()
}
