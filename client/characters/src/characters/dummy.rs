use std::{collections::HashMap, iter::empty};

use bevy::prelude::*;

use wag_core::{
    Animation, AnimationType, Area, DummyAnimation, ItemId, Model, MoveId, Status, StatusCondition,
};

use crate::{
    moves::{
        airborne, crouching, grounded, standing, Action, Attack, CancelPolicy, FlowControl,
        MoveType, Movement, Projectile, Situation,
    },
    AttackHeight, BlockType, Cost, Hitbox, Item, Lifetime, Move, OnHitEffect, ToHit,
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
                requirement: standing,
                phases: vec![
                    Animation::Dummy(DummyAnimation::Slap).into(),
                    FlowControl::Wait(9, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.7, 1.35, 0.35, 0.25)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(10, CancelPolicy::IfHit),
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
                    Animation::Dummy(DummyAnimation::CrouchChop).into(),
                    FlowControl::Wait(8, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.75, 0.2, 0.3, 0.2)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(7, CancelPolicy::IfHit),
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
                    Animation::Dummy(DummyAnimation::BurnStraight).into(),
                    FlowControl::Wait(10, CancelPolicy::Never),
                    FlowControl::DynamicAction(|situation: Situation| {
                        Some(
                            Attack {
                                to_hit: ToHit {
                                    hitbox: Hitbox(Area::new(0.6, 1.35, 1.0, 0.2)),
                                    lifetime: Lifetime::frames(8),
                                    ..default()
                                },
                                on_hit: OnHitEffect {
                                    damage: 20,
                                    knockback: Vec2::splat(3.0),
                                    pushback: if situation.inventory.contains(&ItemId::Drugs) {
                                        3.0
                                    } else {
                                        -10.0
                                    } * Vec2::X,
                                    ..default()
                                },
                                ..default()
                            }
                            .into(),
                        )
                    }),
                    Movement {
                        amount: Vec2::X * 2.0,
                        duration: 1,
                    }
                    .into(),
                    FlowControl::Wait(10, CancelPolicy::IfHit),
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
                    Animation::Dummy(DummyAnimation::AntiAir).into(),
                    FlowControl::Wait(13, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.75, 1.9, 0.3, 0.5)),
                            lifetime: Lifetime::frames(4),
                            ..default()
                        },
                        on_hit: OnHitEffect {
                            knockback: Vec2::splat(4.0),
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(13, CancelPolicy::IfHit),
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
                    Animation::Dummy(DummyAnimation::AirSlap).into(),
                    FlowControl::Wait(8, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.7, 1.3, 0.35, 0.25)),
                            lifetime: Lifetime::frames(5),
                            block_type: BlockType::Constant(AttackHeight::High),
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(10, CancelPolicy::IfHit),
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
                    Animation::Dummy(DummyAnimation::Divekick).into(),
                    FlowControl::Wait(5, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.6, 0.1, 0.35, 0.25)),
                            lifetime: Lifetime::frames(10),
                            block_type: BlockType::Constant(AttackHeight::High),
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(10, CancelPolicy::IfHit),
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
                    Animation::Dummy(DummyAnimation::NormalThrow).into(),
                    FlowControl::Wait(5, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            block_type: BlockType::Grab,
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.5)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        on_hit: OnHitEffect {
                            damage: 25,
                            stun: 60,
                            knockback: Vec2::Y * 1.0,
                            forced_animation: Some(Animation::Dummy(
                                DummyAnimation::NormalThrowRecipient,
                            )),
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(40, CancelPolicy::Never),
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
                    Animation::Dummy(DummyAnimation::NormalThrow).into(),
                    FlowControl::Wait(5, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            block_type: BlockType::Grab,
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.5)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        on_hit: OnHitEffect {
                            damage: 25,
                            stun: 60,
                            knockback: Vec2::Y * 1.0,
                            forced_animation: Some(Animation::Dummy(
                                DummyAnimation::NormalThrowRecipient,
                            )),
                            side_switch: true,
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(40, CancelPolicy::Never),
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
                    Animation::Dummy(DummyAnimation::Sweep).into(),
                    FlowControl::Wait(10, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            block_type: BlockType::Grab,
                            hitbox: Hitbox(Area::new(0.7, 0.2, 1.0, 0.2)),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        on_hit: OnHitEffect {
                            damage: 25,
                            knockback: Vec2::Y * 8.0,
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(15, CancelPolicy::IfHit),
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
                    Action::ForceStand.into(),
                    Animation::Dummy(DummyAnimation::Dodge).into(),
                    Action::Condition(StatusCondition {
                        name: Status::Dodge,
                        effect: None,
                        expiration: Some(20),
                    })
                    .into(),
                    FlowControl::Wait(45, CancelPolicy::Never),
                ],
                ..default()
            },
        ),
        (
            MoveId::GroundSlam,
            Move {
                input: Some("[789]6s"),
                move_type: MoveType::Special,
                requirement: grounded,
                phases: vec![
                    Animation::Dummy(DummyAnimation::GroundSlam).into(),
                    FlowControl::Wait(14, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.7, 1.25, 0.8, 0.8)),
                            lifetime: Lifetime::frames(8),
                            ..default()
                        },
                        on_hit: OnHitEffect {
                            damage: 20,
                            knockback: Vec2::Y,
                            pushback: -3.0 * Vec2::X,
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    Movement {
                        amount: Vec2::X * 2.0,
                        duration: 1,
                    }
                    .into(),
                    FlowControl::Wait(20, CancelPolicy::IfHit),
                ],
            },
        ),
        (
            MoveId::AirSlam,
            Move {
                input: Some("[789]6s"),
                move_type: MoveType::Special,
                requirement: airborne,
                phases: vec![
                    Animation::Dummy(DummyAnimation::AirSlam).into(),
                    FlowControl::Wait(14, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.9, 1.25, 0.8, 0.8)),
                            lifetime: Lifetime::frames(8),
                            ..default()
                        },
                        on_hit: OnHitEffect {
                            damage: 20,
                            knockback: Vec2::Y,
                            pushback: -3.0 * Vec2::X,
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    Movement {
                        amount: Vec2::X * 1.0,
                        duration: 2,
                    }
                    .into(),
                    FlowControl::Wait(35, CancelPolicy::IfHit),
                ],
            },
        ),
        (
            MoveId::BudgetBoom,
            Move {
                input: Some("[41]6f"),
                requirement: standing,
                move_type: MoveType::Special,
                phases: vec![
                    Action::ForceStand.into(),
                    FlowControl::Wait(10, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.3, 0.2)),
                            velocity: Some(5.0 * Vec2::X),
                            lifetime: Lifetime::frames((wag_core::FPS * 0.25) as usize),
                            projectile: Some(Projectile {
                                model: Model::Fireball,
                            }),
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(5, CancelPolicy::IfHit),
                ],
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
                    Action::ForceStand.into(),
                    Action::Pay(Cost::charge()).into(),
                    FlowControl::Wait(10, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.2, 0.4, 0.3)),
                            velocity: Some(6.0 * Vec2::X),
                            lifetime: Lifetime::until_owner_hit(),
                            projectile: Some(Projectile {
                                model: Model::Fireball,
                            }),
                            hits: 3,
                            ..default()
                        },
                        on_hit: OnHitEffect {
                            damage: 10,
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(5, CancelPolicy::IfHit),
                ],
            },
        ),
        (
            MoveId::Hadouken,
            Move {
                input: Some("236f"),
                move_type: MoveType::Special,
                phases: vec![
                    Action::ForceStand.into(),
                    FlowControl::Wait(30, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                            velocity: Some(4.0 * Vec2::X),
                            lifetime: Lifetime::until_owner_hit(),
                            projectile: Some(Projectile {
                                model: Model::Fireball,
                            }),
                            hits: 3,
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(30, CancelPolicy::IfHit),
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
                    Action::ForceStand.into(),
                    Action::Pay(Cost::meter(30)).into(),
                    FlowControl::Wait(30, CancelPolicy::Never),
                    Attack {
                        to_hit: ToHit {
                            hitbox: Hitbox(Area::new(0.5, 1.0, 0.4, 0.5)),
                            velocity: Some(5.0 * Vec2::X),
                            lifetime: Lifetime::until_owner_hit(),
                            projectile: Some(Projectile {
                                model: Model::Fireball,
                            }),
                            hits: 2,
                            ..default()
                        },
                        on_hit: OnHitEffect {
                            stun: 30,
                            ..default()
                        },
                        ..default()
                    }
                    .into(),
                    FlowControl::Wait(20, CancelPolicy::IfHit),
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
