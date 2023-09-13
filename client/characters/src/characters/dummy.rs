use std::{collections::HashMap, iter::empty};

use bevy::prelude::*;

use wag_core::{
    Animation, AnimationType, Area, DummyAnimation, ItemId, Joint, Model, MoveId, Stats,
    StatusCondition, StatusFlag, FPS,
};

use crate::{
    moves::{
        grounded, ActionEvent::*, Attack, CancelCategory, CancelPolicy, CommonAttackProps,
        FlowControl::*, Movement, Projectile, Situation, StunType::*,
    },
    properties::PropertyType,
    AttackHeight::*,
    BarRenderInstructions,
    BlockType::*,
    ChargeProperty, Hitbox, Item,
    ItemCategory::*,
    Lifetime, Move, Property, SpecialProperty, ToHit,
};

use super::{
    dash,
    equipment::{get_handmedownken, get_high_gi_parry},
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
        Stats {
            walk_speed: 3.0,
            max_health: 250,
            opener_damage_multiplier: 1.5,
            opener_meter_gain: 50,
            opener_stun_frames: 5,
        },
        vec![(
            PropertyType::Charge,
            Property {
                max: FPS as i32, // Frames to full,
                special: Some(SpecialProperty::Charge(ChargeProperty::default())),
                render_instructions: BarRenderInstructions {
                    default_color: Color::rgb(0.05, 0.4, 0.55),
                    full_color: Some(Color::rgb(0.9, 0.1, 0.3)),
                    ..default()
                },
                ..default()
            },
        )],
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
        (MoveId::HighGiParry, get_high_gi_parry()),
    ]
    .into_iter()
}

fn dashes() -> impl Iterator<Item = (MoveId, Move)> {
    vec![
        (
            MoveId::DashForward,
            dash(
                "5656",
                DASH_DURATION,
                DASH_IMPULSE,
                DummyAnimation::DashForward.into(),
            ),
        ),
        (
            MoveId::DashBack,
            dash(
                "5454",
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
            Move::grounded(
                Some("f"),
                CancelCategory::Normal,
                vec![
                    DummyAnimation::Slap.into(),
                    Wait(9, CancelPolicy::never()),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.35)),
                            joint: Some(Joint::HandR),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(10, CancelPolicy::neutral_normal_recovery()),
                ],
            ),
        ),
        (
            MoveId::LowChop,
            Move::grounded(
                Some("[123]f"),
                CancelCategory::CommandNormal,
                vec![
                    DummyAnimation::CrouchChop.into(),
                    Wait(8, CancelPolicy::never()),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.1, -0.2, 0.3, 0.2)),
                            joint: Some(Joint::HandL),
                            lifetime: Lifetime::frames(5),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(7, CancelPolicy::command_normal_recovery()),
                ],
            ),
        ),
        (
            MoveId::BurnStraight,
            Move::grounded(
                Some("s"),
                CancelCategory::Normal,
                vec![
                    DummyAnimation::BurnStraight.into(),
                    Wait(10, CancelPolicy::never()),
                    DynamicActions(|situation: Situation| {
                        vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::new(-0.3, 0.0, 1.0, 0.2)),
                                joint: Some(Joint::HandR),
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
                    Wait(10, CancelPolicy::neutral_normal_recovery()),
                ],
            ),
        ),
        (
            MoveId::AntiAir,
            Move::grounded(
                Some("[123]s"),
                CancelCategory::CommandNormal,
                vec![
                    DummyAnimation::AntiAir.into(),
                    Wait(13, CancelPolicy::never()),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::of_size(0.3, 0.5)),
                            joint: Some(Joint::HandR),
                            lifetime: Lifetime::frames(4),
                            ..default()
                        },
                        CommonAttackProps {
                            knock_back: Vec2::new(-4.0, 3.0),
                            ..default()
                        },
                    )
                    .into(),
                    Wait(13, CancelPolicy::command_normal_recovery()),
                ],
            ),
        ),
        (
            MoveId::AirSlap,
            Move::airborne(
                Some("f"),
                CancelCategory::Normal,
                vec![
                    DummyAnimation::AirSlap.into(),
                    Wait(8, CancelPolicy::never()),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.25)),
                            joint: Some(Joint::HandR),
                            lifetime: Lifetime::frames(5),
                            block_type: Constant(High),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(10, CancelPolicy::neutral_normal_recovery()),
                ],
            ),
        ),
        (
            MoveId::Divekick,
            Move::airborne(
                Some("s"),
                CancelCategory::Normal,
                vec![
                    DummyAnimation::Divekick.into(),
                    Wait(5, CancelPolicy::never()),
                    Attack::new(
                        ToHit {
                            hitbox: Hitbox(Area::of_size(0.35, 0.25)),
                            joint: Some(Joint::FootR),
                            lifetime: Lifetime::frames(10),
                            block_type: Constant(High),
                            ..default()
                        },
                        CommonAttackProps::default(),
                    )
                    .into(),
                    Wait(10, CancelPolicy::neutral_normal_recovery()),
                ],
            ),
        ),
        (
            MoveId::ForwardThrow,
            Move::grounded(
                Some("w"),
                CancelCategory::Normal,
                vec![
                    DummyAnimation::NormalThrow.into(),
                    Wait(5, CancelPolicy::never()),
                    Attack::new(
                        ToHit {
                            block_type: Grab,
                            hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                            joint: Some(Joint::HandL),
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
                    Wait(40, CancelPolicy::never()),
                ],
            ),
        ),
        (
            MoveId::BackThrow,
            Move::grounded(
                Some("4w"),
                CancelCategory::CommandNormal,
                vec![
                    DummyAnimation::NormalThrow.into(),
                    Wait(5, CancelPolicy::never()),
                    Attack::new(
                        ToHit {
                            block_type: Grab,
                            hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                            joint: Some(Joint::HandL),
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
                    Wait(40, CancelPolicy::never()),
                ],
            ),
        ),
        (
            MoveId::Sweep,
            Move::grounded(
                Some("[123]w"),
                CancelCategory::CommandNormal,
                vec![
                    DummyAnimation::Sweep.into(),
                    Wait(10, CancelPolicy::never()),
                    Attack::new(
                        ToHit {
                            block_type: Grab,
                            hitbox: Hitbox(Area::new(-0.3, 0.0, 1.0, 0.2)),
                            joint: Some(Joint::HandR),
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
                    Wait(15, CancelPolicy::command_normal_recovery()),
                ],
            ),
        ),
        (
            MoveId::AirThrow,
            Move::airborne(
                Some("w"),
                CancelCategory::Normal,
                vec![
                    DummyAnimation::AirThrow.into(),
                    Wait(9, CancelPolicy::never()),
                    Attack::new(
                        ToHit {
                            block_type: Grab,
                            hitbox: Hitbox(Area::of_size(0.8, 0.8)),
                            joint: Some(Joint::HandR),
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
                    Wait(30, CancelPolicy::never()),
                ],
            ),
        ),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (MoveId, Move)> {
    vec![
        (
            MoveId::Dodge,
            Move::grounded(
                Some("252"),
                CancelCategory::Special,
                vec![
                    vec![
                        ForceStand,
                        DummyAnimation::Dodge.into(),
                        Condition(StatusCondition {
                            flag: StatusFlag::Intangible,
                            effect: None,
                            expiration: Some(20),
                        }),
                    ]
                    .into(),
                    Wait(45, CancelPolicy::never()),
                ],
            ),
        ),
        (
            MoveId::GroundSlam,
            Move::grounded(
                Some("[789]6s"),
                CancelCategory::Special,
                vec![
                    DummyAnimation::GroundSlam.into(),
                    Wait(14, CancelPolicy::never()),
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
                    Wait(20, CancelPolicy::never()),
                ],
            ),
        ),
        (
            MoveId::AirSlam,
            Move::airborne(
                Some("[789]6s"),
                CancelCategory::Special,
                vec![
                    DummyAnimation::AirSlam.into(),
                    Wait(14, CancelPolicy::never()),
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
                    Wait(35, CancelPolicy::never()),
                ],
            ),
        ),
        (
            MoveId::BudgetBoom,
            Move::grounded(
                Some("[41]6f"),
                CancelCategory::Special,
                vec![
                    ForceStand.into(),
                    Wait(10, CancelPolicy::never()),
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
                    Wait(5, CancelPolicy::never()),
                ],
            ),
        ),
        (
            MoveId::SonicBoom,
            Move::new(
                Some("[41]6f"),
                CancelCategory::Special,
                vec![
                    vec![ForceStand, ClearProperty(PropertyType::Charge)].into(),
                    Wait(10, CancelPolicy::never()),
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
                    Wait(5, CancelPolicy::never()),
                ],
                |situation: Situation| {
                    // Charge check
                    situation
                        .properties
                        .get(&PropertyType::Charge)
                        .unwrap()
                        .is_full()
                        && grounded(situation)
                },
            ),
        ),
        (
            MoveId::Hadouken,
            Move::grounded(
                Some("236f"),
                CancelCategory::Special,
                vec![
                    ForceStand.into(),
                    Wait(30, CancelPolicy::never()),
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
                    Wait(30, CancelPolicy::never()),
                ],
            ),
        ),
        (
            MoveId::HeavyHadouken,
            Move::new(
                Some("236s"),
                CancelCategory::Special,
                vec![
                    vec![ForceStand, ModifyProperty(PropertyType::Meter, -30)].into(),
                    Wait(30, CancelPolicy::never()),
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
                    Wait(20, CancelPolicy::never()),
                ],
                |situation: Situation| {
                    situation
                        .properties
                        .get(&PropertyType::Meter)
                        .unwrap()
                        .current
                        >= 30
                },
            ),
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
                explanation: "Get yoked".into(),
                ..default()
            },
        ),
        (
            ItemId::HandMeDownKen,
            Item {
                cost: 10,
                explanation: "Haduu ken".into(),
                ..default()
            },
        ),
        (
            ItemId::Gi,
            Item {
                cost: 100,
                explanation: "Lesgo justin".into(),
                ..default()
            },
        ),
        (
            ItemId::Boots,
            Item {
                cost: 80,
                explanation: "Bonus walk speed".into(),
                effect: Stats {
                    walk_speed: 0.2,
                    ..default()
                },
                ..default()
            },
        ),
        (
            ItemId::SafetyBoots,
            Item {
                category: Upgrade(vec![ItemId::Boots]),
                explanation: "Gives more health in addition to boots' speed bonus".into(),
                cost: 100,
                effect: Stats {
                    max_health: 20,
                    ..default()
                },
            },
        ),
    ]
    .into_iter()
    .collect()
}
