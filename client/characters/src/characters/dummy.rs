use std::{collections::HashMap, iter::empty};

use bevy::prelude::*;

use wag_core::{
    Animation, AnimationType, Area, DummyAnimation, ItemId, Joint, Model, MoveId, Stats,
    StatusCondition, StatusFlag, FPS,
};

use crate::{
    resources::ResourceType, Action, ActionBlock, ActionEvent::*, Attack, AttackHeight::*,
    BlockType::*, CancelCategory, CancelPolicy, ChargeProperty, CommonAttackProps, Hitbox, Item,
    ItemCategory::*, Lifetime, Movement, Projectile, Requirement, ResourceBarVisual, Situation,
    SpecialProperty, StunType::*, ToHit, WAGResource,
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
            ResourceType::Charge,
            WAGResource {
                max: FPS as i32, // Frames to full,
                special: Some(SpecialProperty::Charge(ChargeProperty::default())),
                render_instructions: ResourceBarVisual {
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

fn dummy_moves() -> HashMap<MoveId, Action> {
    empty()
        .chain(items())
        .chain(dashes())
        .chain(normals())
        .chain(specials())
        .collect()
}

fn items() -> impl Iterator<Item = (MoveId, Action)> {
    vec![
        (MoveId::HandMeDownKen, get_handmedownken()),
        (MoveId::HighGiParry, get_high_gi_parry()),
    ]
    .into_iter()
}

fn dashes() -> impl Iterator<Item = (MoveId, Action)> {
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

fn normals() -> impl Iterator<Item = (MoveId, Action)> {
    vec![
        (
            MoveId::Slap,
            Action::grounded(
                Some("f"),
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::Slap.into()],
                        exit_requirement: Requirement::Time(9),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.35)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(5),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()],
                        exit_requirement: Requirement::Time(10),
                        cancel_policy: CancelPolicy::neutral_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::LowChop,
            Action::grounded(
                Some("[123]f"),
                CancelCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::CrouchChop.into()],
                        exit_requirement: Requirement::Time(8),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.1, -0.2, 0.3, 0.2)),
                                joint: Some(Joint::HandL),
                                lifetime: Lifetime::frames(5),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()],
                        exit_requirement: Requirement::Time(7),
                        cancel_policy: CancelPolicy::command_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::BurnStraight,
            Action::grounded(
                Some("s"),
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::BurnStraight.into()],
                        exit_requirement: Requirement::Time(10),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![
                            Attack::new(
                                ToHit {
                                    hitbox: Hitbox(Area::new(-0.3, 0.0, 1.0, 0.2)),
                                    joint: Some(Joint::HandR),
                                    lifetime: Lifetime::frames(8),
                                    ..default()
                                },
                                CommonAttackProps {
                                    damage: 20,
                                    on_hit: Stun(20),
                                    knock_back: -3.0 * Vec2::X,
                                    push_back: -2.0 * Vec2::X,
                                    ..default()
                                },
                            )
                            .into(),
                            Movement {
                                amount: Vec2::X * 2.0,
                                duration: 1,
                            }
                            .into(),
                        ],
                        exit_requirement: Requirement::Time(10),
                        cancel_policy: CancelPolicy::neutral_normal_recovery(),
                        mutator: Some(|block, situation| {
                            if !situation.inventory.contains(&ItemId::Roids) {
                                return block.clone();
                            }

                            ActionBlock {
                                events: block
                                    .events
                                    .clone()
                                    .into_iter()
                                    .map(|event| match event {
                                        Attack(_) => Attack::new(
                                            ToHit {
                                                hitbox: Hitbox(Area::new(-0.3, 0.0, 1.0, 0.2)),
                                                joint: Some(Joint::HandR),
                                                lifetime: Lifetime::frames(8),
                                                ..default()
                                            },
                                            CommonAttackProps {
                                                damage: 20,
                                                on_hit: Stun(20),
                                                // These are the only changed properties, but it's easier to reconstruct than to edit
                                                knock_back: 1.0 * Vec2::X,
                                                push_back: Vec2::ZERO,
                                                ..default()
                                            },
                                        )
                                        .into(),
                                        _ => event,
                                    })
                                    .collect(),
                                ..block.clone()
                            }
                        }),
                    },
                ],
            ),
        ),
        (
            MoveId::AntiAir,
            Action::grounded(
                Some("[123]s"),
                CancelCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::AntiAir.into()],
                        exit_requirement: Requirement::Time(13),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
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
                        .into()],
                        exit_requirement: Requirement::Time(13),
                        cancel_policy: CancelPolicy::command_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::AirSlap,
            Action::airborne(
                Some("f"),
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::AirSlap.into()],
                        exit_requirement: Requirement::Time(8),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.25)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(5),
                                block_type: Constant(High),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()],
                        exit_requirement: Requirement::Time(10),
                        cancel_policy: CancelPolicy::neutral_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::Divekick,
            Action::airborne(
                Some("s"),
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::Divekick.into()],
                        exit_requirement: Requirement::Time(5),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::of_size(0.35, 0.25)),
                                joint: Some(Joint::FootR),
                                lifetime: Lifetime::frames(10),
                                block_type: Constant(High),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()],
                        exit_requirement: Requirement::Time(10),
                        cancel_policy: CancelPolicy::neutral_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::ForwardThrow,
            Action::grounded(
                Some("w"),
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::NormalThrow.into()],
                        exit_requirement: Requirement::Time(5),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
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
                        .into()],
                        exit_requirement: Requirement::Time(40),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::BackThrow,
            Action::grounded(
                Some("4w"),
                CancelCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::NormalThrow.into()],
                        exit_requirement: Requirement::Time(5),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
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
                        .into()],
                        exit_requirement: Requirement::Time(40),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::Sweep,
            Action::grounded(
                Some("[123]w"),
                CancelCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::Sweep.into()],
                        exit_requirement: Requirement::Time(10),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
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
                        .into()],
                        exit_requirement: Requirement::Time(15),
                        cancel_policy: CancelPolicy::command_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::AirThrow,
            Action::airborne(
                Some("w"),
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::AirThrow.into()],
                        exit_requirement: Requirement::Time(9),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
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
                        .into()],
                        exit_requirement: Requirement::Time(30),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                ],
            ),
        ),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (MoveId, Action)> {
    vec![
        (
            MoveId::Dodge,
            Action::grounded(
                Some("252"),
                CancelCategory::Special,
                vec![ActionBlock {
                    events: vec![
                        ForceStand,
                        DummyAnimation::Dodge.into(),
                        Condition(StatusCondition {
                            flag: StatusFlag::Intangible,
                            effect: None,
                            expiration: Some(20),
                        }),
                    ],
                    exit_requirement: Requirement::Time(45),
                    cancel_policy: CancelPolicy::never(),
                    mutator: None,
                }],
            ),
        ),
        (
            MoveId::GroundSlam,
            Action::grounded(
                Some("[789]6s"),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::GroundSlam.into()],
                        exit_requirement: Requirement::Time(14),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![
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
                        ],
                        exit_requirement: Requirement::Time(20),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::AirSlam,
            Action::airborne(
                Some("[789]6s"),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![DummyAnimation::AirSlam.into()],
                        exit_requirement: Requirement::Time(14),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![
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
                        ],
                        exit_requirement: Requirement::Time(35),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::BudgetBoom,
            Action::grounded(
                Some("[41]6f"),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![ForceStand],
                        exit_requirement: Requirement::Time(10),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
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
                        .into()],
                        exit_requirement: Requirement::Time(5),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::SonicBoom,
            Action::new(
                Some("[41]6f"),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![ForceStand, ClearProperty(ResourceType::Charge)],
                        exit_requirement: Requirement::Time(10),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
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
                        .into()],
                        exit_requirement: Requirement::Time(5),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                ],
                |situation: Situation| {
                    // Charge check
                    situation.resources[&ResourceType::Charge].is_full() && situation.grounded()
                },
            ),
        ),
        (
            MoveId::Hadouken,
            Action::grounded(
                Some("236f"),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![ForceStand],
                        exit_requirement: Requirement::Time(30),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
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
                        .into()],
                        exit_requirement: Requirement::Time(30),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MoveId::HeavyHadouken,
            Action::new(
                Some("236s"),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![ForceStand, ModifyProperty(ResourceType::Meter, -30)],
                        exit_requirement: Requirement::Time(30),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                    ActionBlock {
                        events: vec![Attack::new(
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
                        .into()],
                        exit_requirement: Requirement::Time(20),
                        cancel_policy: CancelPolicy::never(),
                        mutator: None,
                    },
                ],
                |situation: Situation| situation.resources[&ResourceType::Meter].current >= 30,
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
