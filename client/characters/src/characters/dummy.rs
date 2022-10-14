use std::{collections::HashMap, iter::empty};

use bevy::prelude::*;

use types::{Animation, Area, DummyAnimation, ItemId, Model, MoveId, Status, StatusCondition};

use crate::{
    moves::{
        grounded, not_grounded, Action, FlowControl, MoveType, Movement, Projectile, Situation,
    },
    AttackHeight, BlockType, Cost, Hitbox, Item, Lifetime, Move, OnHitEffect, ToHit,
};

use super::{
    dash,
    equipment::{get_gunshot, get_handmedownken, get_shot},
    jump, Character,
};

pub fn dummy() -> Character {
    Character::new(dummy_moves(), dummy_items())
}

// Dashing
const DASH_DURATION: usize = (0.5 * constants::FPS) as usize;
const DASH_IMPULSE: f32 = 10.0;

fn dummy_moves() -> HashMap<MoveId, Move> {
    empty()
        .chain(items())
        .chain(jumps())
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

fn jumps() -> impl Iterator<Item = (MoveId, Move)> {
    vec![
        (
            MoveId::BackJump,
            jump(
                "7",
                Vec2::new(-constants::DIAGONAL_JUMP_X, constants::DIAGONAL_JUMP_Y),
            ),
        ),
        (
            MoveId::NeutralJump,
            jump("8", Vec2::Y * constants::NEUTRAL_JUMP_Y),
        ),
        (
            MoveId::ForwardJump,
            jump(
                "9",
                Vec2::new(constants::DIAGONAL_JUMP_X, constants::DIAGONAL_JUMP_Y),
            ),
        ),
        (
            MoveId::BackSuperJump,
            jump(
                "[123]7",
                Vec2::new(
                    -constants::DIAGONAL_SUPERJUMP_X,
                    constants::DIAGONAL_SUPERJUMP_Y,
                ),
            ),
        ),
        (
            MoveId::NeutralSuperJump,
            jump("[123]8", Vec2::Y * constants::NEUTRAL_SUPERJUMP_Y),
        ),
        (
            MoveId::ForwardSuperJump,
            jump(
                "[123]9",
                Vec2::new(
                    constants::DIAGONAL_SUPERJUMP_X,
                    constants::DIAGONAL_SUPERJUMP_Y,
                ),
            ),
        ),
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
                Animation::Dummy(DummyAnimation::DashForward),
            ),
        ),
        (
            MoveId::DashBack,
            dash(
                "454",
                DASH_DURATION,
                -DASH_IMPULSE,
                Animation::Dummy(DummyAnimation::DashBack),
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
                phases: vec![
                    Action::Animation(Animation::Dummy(DummyAnimation::Slap)).into(),
                    FlowControl::Wait(9, false),
                    Action::Attack(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.7, 1.35, 0.35, 0.25)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        OnHitEffect { ..default() },
                    )
                    .into(),
                    FlowControl::Wait(10, true),
                ],
                ..default()
            },
        ),
        (
            MoveId::LowChop,
            Move {
                input: Some("[123]f"),
                phases: vec![
                    Action::Animation(Animation::Dummy(DummyAnimation::CrouchChop)).into(),
                    FlowControl::Wait(8, false),
                    Action::Attack(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.75, 0.2, 0.3, 0.2)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        OnHitEffect::default(),
                    )
                    .into(),
                    FlowControl::Wait(7, true),
                ],
                ..default()
            },
        ),
        (
            MoveId::BurnStraight,
            Move {
                input: Some("s"),
                phases: vec![
                    Action::Animation(Animation::Dummy(DummyAnimation::BurnStraight)).into(),
                    FlowControl::Wait(10, false),
                    Action::Attack(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.6, 1.35, 1.0, 0.2)),
                            lifetime: Lifetime::frames(8),
                            ..default()
                        },
                        OnHitEffect::default(),
                    )
                    .into(),
                    Action::Movement(Movement {
                        amount: Vec2::X * 2.0,
                        duration: 1,
                    })
                    .into(),
                    FlowControl::Wait(10, false),
                ],
                ..default()
            },
        ),
        (
            MoveId::AntiAir,
            Move {
                input: Some("[123]s"),
                phases: vec![
                    Action::Animation(Animation::Dummy(DummyAnimation::AntiAir)).into(),
                    FlowControl::Wait(13, false),
                    Action::Attack(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.75, 1.9, 0.3, 0.5)),
                            lifetime: Lifetime::frames(4),
                            ..default()
                        },
                        OnHitEffect {
                            knockback: (Vec2::splat(4.0), Vec2::ZERO).into(),
                            ..default()
                        },
                    )
                    .into(),
                    FlowControl::Wait(13, false),
                ],
                ..default()
            },
        ),
        (
            MoveId::AirSlap,
            Move {
                input: Some("f"),
                requirement: not_grounded,
                phases: vec![
                    Action::Animation(Animation::Dummy(DummyAnimation::AirSlap)).into(),
                    FlowControl::Wait(8, false),
                    Action::Attack(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.7, 1.3, 0.35, 0.25)),
                            lifetime: Lifetime::frames(5),
                            block_type: BlockType::Constant(AttackHeight::High),
                            ..default()
                        },
                        OnHitEffect::default(),
                    )
                    .into(),
                    FlowControl::Wait(10, true),
                ],
                ..default()
            },
        ),
        (
            MoveId::Divekick,
            Move {
                input: Some("s"),
                requirement: not_grounded,
                phases: vec![
                    Action::Animation(Animation::Dummy(DummyAnimation::Divekick)).into(),
                    FlowControl::Wait(5, false),
                    Action::Attack(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.6, 0.1, 0.35, 0.25)),
                            lifetime: Lifetime::frames(10),
                            block_type: BlockType::Constant(AttackHeight::High),
                            ..default()
                        },
                        OnHitEffect::default(),
                    )
                    .into(),
                    FlowControl::Wait(10, true),
                ],
                ..default()
            },
        ),
        (
            MoveId::Grab,
            Move {
                input: Some("g"),
                phases: vec![
                    FlowControl::Wait(5, false),
                    Action::Attack(
                        ToHit {
                            block_type: BlockType::Grab,
                            hitbox: Hitbox(Area::new(0.75, 1.9, 0.3, 0.5)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        OnHitEffect {
                            damage: (25, 0).into(),
                            stun: (60, 0).into(),
                            knockback: (Vec2::Y * 1.0, Vec2::ZERO).into(),
                            ..default()
                        },
                    )
                    .into(),
                    FlowControl::Wait(40, true),
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
                move_type: MoveType::Normal,
                phases: vec![
                    Action::Animation(Animation::Dummy(DummyAnimation::Dodge)).into(),
                    Action::Condition(StatusCondition {
                        name: Status::Dodge,
                        effect: None,
                        expiration: Some(20),
                    })
                    .into(),
                    FlowControl::Wait(45, false),
                ],
                ..default()
            },
        ),
        (
            MoveId::BudgetBoom,
            Move {
                input: Some("[41]6f"),
                move_type: MoveType::Special,
                phases: vec![
                    FlowControl::Wait(10, false),
                    Action::Attack(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.3, 0.2)),
                            velocity: Some(5.0 * Vec2::X),
                            lifetime: Lifetime::frames((constants::FPS * 0.25) as usize),
                            projectile: Some(Projectile {
                                model: Model::Fireball,
                            }),
                            ..default()
                        },
                        OnHitEffect::default(),
                    )
                    .into(),
                    FlowControl::Wait(5, true),
                ],
                ..default()
            },
        ),
        (
            MoveId::SonicBoom,
            Move {
                input: Some("[41]6f"),
                move_type: MoveType::Special,
                requirement: |situation: Situation| {
                    situation.resources.can_afford(Cost::charge()) && grounded(situation)
                },
                phases: vec![
                    Action::Pay(Cost::charge()).into(),
                    FlowControl::Wait(10, false),
                    Action::Attack(
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
                        OnHitEffect {
                            damage: (10, 3).into(),
                            ..default()
                        },
                    )
                    .into(),
                    FlowControl::Wait(5, true),
                ],
            },
        ),
        (
            MoveId::Hadouken,
            Move {
                input: Some("236f"),
                move_type: MoveType::Special,
                phases: vec![
                    FlowControl::Wait(30, false),
                    Action::Attack(
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
                        OnHitEffect::default(),
                    )
                    .into(),
                    FlowControl::Wait(30, true),
                ],
                ..default()
            },
        ),
        (
            MoveId::HeavyHadouken,
            Move {
                input: Some("236s"),
                move_type: MoveType::Special,
                requirement: |situation: Situation| situation.resources.can_afford(Cost::meter(30)),
                phases: vec![
                    Action::Pay(Cost::meter(30)).into(),
                    FlowControl::Wait(30, false),
                    Action::Attack(
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
                        OnHitEffect {
                            stun: (30, 30).into(),
                            ..default()
                        },
                    )
                    .into(),
                    FlowControl::Wait(20, false),
                ],
            },
        ),
    ]
    .into_iter()
}

fn dummy_items() -> HashMap<ItemId, Item> {
    vec![
        (
            ItemId::Drugs,
            Item {
                cost: 100,
                tier: 1,
                is_starter: false,
            },
        ),
        (
            ItemId::HandMeDownKen,
            Item {
                cost: 0,
                tier: 0,
                is_starter: true,
            },
        ),
        (
            ItemId::Gi,
            Item {
                cost: 100,
                tier: 2,
                is_starter: true,
            },
        ),
        (
            ItemId::Gun,
            Item {
                cost: 100,
                tier: 2,
                is_starter: true,
            },
        ),
    ]
    .into_iter()
    .collect()
}
