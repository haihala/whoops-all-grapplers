use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionId, Animation, AnimationType, Area, GameButton, Icon, ItemId, Joint, MizkuActionId,
    MizkuAnimation, Model, Stats, StatusCondition, StatusFlag, MIZUKI_ALT_HELMET_COLOR,
    MIZUKI_ALT_JEANS_COLOR, MIZUKI_ALT_SHIRT_COLOR,
};

use crate::{
    actions::{ActionCategory, ActionRequirement, Projectile},
    resources::{RenderInstructions, ResourceType},
    Action, ActionBlock,
    ActionEvent::*,
    Attack,
    AttackHeight::*,
    BlockType::*,
    CancelRule, CommonAttackProps, ConsumableType, ContinuationRequirement, CounterVisual,
    FlashRequest, Hitbox, Item, ItemCategory, Lifetime, Movement, Situation,
    StunType::*,
    ToHit, WAGResource,
};

use super::{
    equipment::{universal_item_actions, universal_items},
    helpers::{dashes, jumps},
    Character,
};

pub fn mizku() -> Character {
    let (jumps, gravity) = jumps(2.1, 1.1, Animation::Mizku(MizkuAnimation::Jump));

    Character::new(
        Model::Mizku,
        vec![
            ("T-shirt", MIZUKI_ALT_SHIRT_COLOR),
            ("Jeans", MIZUKI_ALT_JEANS_COLOR),
            ("Samurai Helmet.1", MIZUKI_ALT_HELMET_COLOR),
        ]
        .into_iter()
        .collect(),
        mizku_animations(),
        mizku_moves(jumps),
        mizku_items(),
        Stats {
            walk_speed: 1.5,
            gravity,
            ..default()
        },
        vec![
            (
                ResourceType::Sharpness,
                WAGResource {
                    max: Some(10),
                    render_instructions: RenderInstructions::Counter(CounterVisual {
                        label: "Sharpness",
                    }),
                    ..default()
                },
            ),
            (
                ResourceType::KunaiCounter,
                WAGResource {
                    render_instructions: RenderInstructions::Counter(CounterVisual {
                        label: "Kunais",
                    }),
                    max: Some(1),
                    ..default()
                },
            ),
        ],
    )
}

fn mizku_animations() -> HashMap<AnimationType, Animation> {
    vec![
        (AnimationType::AirIdle, MizkuAnimation::Air),
        (AnimationType::AirStun, MizkuAnimation::AirStagger),
        (AnimationType::StandIdle, MizkuAnimation::Idle),
        (AnimationType::StandBlock, MizkuAnimation::Block),
        (AnimationType::StandStun, MizkuAnimation::Stagger),
        (AnimationType::WalkBack, MizkuAnimation::WalkBack),
        (AnimationType::WalkForward, MizkuAnimation::WalkForward),
        (AnimationType::CrouchIdle, MizkuAnimation::Crouch),
        (AnimationType::CrouchBlock, MizkuAnimation::CrouchBlock),
        (AnimationType::CrouchStun, MizkuAnimation::CrouchStagger),
        (AnimationType::Getup, MizkuAnimation::Getup),
        (AnimationType::Default, MizkuAnimation::StandPose),
    ]
    .into_iter()
    .map(|(k, v)| (k, Animation::from(v)))
    .collect()
}

fn mizku_moves(jumps: impl Iterator<Item = (ActionId, Action)>) -> HashMap<ActionId, Action> {
    jumps
        .chain(dashes(
            MizkuAnimation::DashForward,
            MizkuAnimation::DashBack,
        ))
        .chain(item_actions())
        .chain(
            normals()
                .chain(specials())
                .map(|(k, v)| (ActionId::Mizku(k), v)),
        )
        .collect()
}

fn normals() -> impl Iterator<Item = (MizkuActionId, Action)> {
    vec![
        (
            MizkuActionId::KneeThrust,
            Action::ground_normal(
                "f",
                MizkuAnimation::KneeThrust,
                5,
                Attack::strike(
                    ToHit {
                        hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.35)),
                        joint: Some(Joint::ShinL),
                        lifetime: Lifetime::frames(5),
                        ..default()
                    },
                    CommonAttackProps {
                        damage: 5,
                        on_hit: Stun(20),
                        on_block: Stun(15),
                        ..default()
                    },
                ),
                17,
            ),
        ),
        (
            MizkuActionId::LowKick,
            Action::ground_normal(
                "[123]+f",
                MizkuAnimation::LowKick,
                3,
                Attack::strike(
                    ToHit {
                        hitbox: Hitbox(Area::new(-0.4, 0.0, 0.9, 0.2)),
                        joint: Some(Joint::FootL),
                        lifetime: Lifetime::frames(3),
                        block_type: Strike(Low),
                        ..default()
                    },
                    CommonAttackProps {
                        damage: 8,
                        on_hit: Stun(18),
                        on_block: Stun(11),
                        ..default()
                    },
                ),
                12,
            ),
        ),
        (
            MizkuActionId::HeelKick,
            Action::grounded(
                Some("s"),
                ActionCategory::NeutralNormal,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::HeelKick.into(),
                            Movement {
                                amount: Vec2::X * 10.0,
                                duration: 20,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(9),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![
                            Attack::strike(
                                ToHit {
                                    hitbox: Hitbox(Area::new(-0.2, 0.0, 1.2, 0.2)),
                                    joint: Some(Joint::FootL),
                                    lifetime: Lifetime::frames(5),
                                    ..default()
                                },
                                CommonAttackProps {
                                    damage: 15,
                                    on_hit: Stun(31),
                                    on_block: Stun(20),
                                    ..default()
                                },
                            )
                            .into(),
                            Movement {
                                amount: Vec2::X * 3.0,
                                duration: 10,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(28),
                        cancel_policy: CancelRule::neutral_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::Uppercut,
            Action::ground_normal(
                "[123]+s",
                MizkuAnimation::Uppercut,
                8,
                Attack::strike(
                    ToHit {
                        hitbox: Hitbox(Area::of_size(0.3, 0.5)),
                        joint: Some(Joint::HandR),
                        lifetime: Lifetime::frames(8),
                        ..default()
                    },
                    CommonAttackProps {
                        damage: 16,
                        knock_back: 0.5,
                        push_back: 0.7,
                        on_hit: Launcher(6.0),
                        on_block: Stun(10),
                        ..default()
                    },
                ),
                42,
            ),
        ),
        (
            MizkuActionId::HighStab,
            Action::grounded(
                Some("g"),
                ActionCategory::NeutralNormal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::HighStab.into()],
                        exit_requirement: ContinuationRequirement::Time(17),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![],
                        exit_requirement: ContinuationRequirement::Time(43),
                        cancel_policy: CancelRule::neutral_normal_recovery(),
                        mutator: Some(|mut original: ActionBlock, situation: &Situation| {
                            original.events.push(
                                Attack::strike(
                                    ToHit {
                                        hitbox: Hitbox(Area::new(-0.2, 0.0, 2.0, 0.2)),
                                        joint: Some(Joint::Katana),
                                        lifetime: Lifetime::frames(4),
                                        ..default()
                                    },
                                    CommonAttackProps {
                                        damage: 10
                                            + situation
                                                .get_resource(ResourceType::Sharpness)
                                                .unwrap()
                                                .current
                                                * 10,
                                        on_hit: Stun(40),
                                        on_block: Stun(30),
                                        chip_damage: 5,
                                        ..default()
                                    },
                                )
                                .into(),
                            );

                            original
                        }),
                    },
                ],
            ),
        ),
        (
            MizkuActionId::SkySlash,
            Action::grounded(
                Some("[123]+g"),
                ActionCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::SkyStab.into()],
                        exit_requirement: ContinuationRequirement::Time(15),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![],
                        exit_requirement: ContinuationRequirement::Time(60),
                        cancel_policy: CancelRule::command_normal_recovery(),
                        mutator: Some(|mut original: ActionBlock, situation: &Situation| {
                            original.events.push(
                                Attack::strike(
                                    ToHit {
                                        hitbox: Hitbox(Area::of_size(2.5, 0.3)),
                                        joint: Some(Joint::Katana),
                                        lifetime: Lifetime::frames(3),
                                        block_type: Strike(Low),
                                        ..default()
                                    },
                                    CommonAttackProps {
                                        damage: 8 + situation
                                            .get_resource(ResourceType::Sharpness)
                                            .unwrap()
                                            .current
                                            * 10,
                                        on_hit: Stun(55),
                                        on_block: Stun(25),
                                        chip_damage: 3,
                                        ..default()
                                    },
                                )
                                .into(),
                            );

                            original
                        }),
                    },
                ],
            ),
        ),
        (
            MizkuActionId::AirSlice,
            Action::airborne(
                Some("g"),
                ActionCategory::NeutralNormal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::AirStab.into()],
                        exit_requirement: ContinuationRequirement::Time(7),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![],
                        exit_requirement: ContinuationRequirement::Time(63),
                        cancel_policy: CancelRule::neutral_normal_recovery(),
                        mutator: Some(|mut original: ActionBlock, situation: &Situation| {
                            original.events.push(
                                Attack::strike(
                                    ToHit {
                                        hitbox: Hitbox(Area::new(-0.2, 0.0, 2.0, 0.2)),
                                        joint: Some(Joint::Katana),
                                        lifetime: Lifetime::frames(12),
                                        ..default()
                                    },
                                    CommonAttackProps {
                                        damage: 10
                                            + situation
                                                .get_resource(ResourceType::Sharpness)
                                                .unwrap()
                                                .current
                                                * 10,
                                        on_hit: Stun(40),
                                        on_block: Stun(30),
                                        chip_damage: 5,
                                        ..default()
                                    },
                                )
                                .into(),
                            );

                            original
                        }),
                    },
                ],
            ),
        ),
        (
            MizkuActionId::FalconKnee,
            Action::air_normal(
                "f",
                MizkuAnimation::FalconKnee,
                2,
                Attack::strike(
                    ToHit {
                        hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.25)),
                        joint: Some(Joint::ShinR),
                        lifetime: Lifetime::frames(5),
                        block_type: Strike(High),
                        ..default()
                    },
                    CommonAttackProps {
                        damage: 5,
                        push_back: 0.5,
                        knock_back: 0.2,
                        ..default()
                    },
                ),
                23,
            ),
        ),
        (
            MizkuActionId::FootDive,
            Action::airborne(
                Some("s"),
                ActionCategory::NeutralNormal,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::FootDiveHold.into(),
                            Movement {
                                amount: Vec2::Y * -1.0,
                                duration: 7,
                            }
                            .into(),
                        ],
                        // TODO:Add an item that allows for instant overheads
                        exit_requirement: ContinuationRequirement::Time(20),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![],
                        exit_requirement: ContinuationRequirement::Conditions(vec![
                            ActionRequirement::ButtonNotPressed(GameButton::Strong),
                        ]),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![MizkuAnimation::FootDiveRelease.into()],
                        exit_requirement: ContinuationRequirement::Time(3),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(-0.35, 0.0, 0.7, 0.25)),
                                joint: Some(Joint::FootR),
                                lifetime: Lifetime::frames(7),
                                block_type: Strike(High),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 18,
                                push_back: 1.0,
                                knock_back: 0.8,
                                on_hit: Stun(40),
                                on_block: Stun(25),
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(17),
                        cancel_policy: CancelRule::never(),
                        mutator: Some(|mut original, situation| {
                            if situation.inventory.contains(&ItemId::SpaceSuitBoots) {
                                for ev in original.events.iter_mut() {
                                    if let Attack(attack) = ev {
                                        for ap in attack.target_on_hit.iter_mut() {
                                            if let HitStun(_) = ap {
                                                *ap = Launch {
                                                    impulse: Vec2::new(-1.0, 15.0),
                                                };
                                            }
                                        }
                                    }
                                }
                            }
                            original
                        }),
                    },
                ],
            ),
        ),
        (
            MizkuActionId::ForwardThrow,
            Action::grounded(
                Some("w"),
                ActionCategory::Throw,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::StandThrowStartup.into()],
                        exit_requirement: ContinuationRequirement::Time(3),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::forward_throw(
                            ToHit {
                                block_type: Grab,
                                hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            ActionId::Mizku(MizkuActionId::StandThrowHit),
                            ActionId::Mizku(MizkuActionId::StandThrowTarget),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(37),
                        ..default()
                    },
                ],
            ),
        ),
        (
            MizkuActionId::BackThrow,
            Action::grounded(
                Some("4+w"),
                ActionCategory::Throw,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::StandThrowStartup.into()],
                        exit_requirement: ContinuationRequirement::Time(3),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::back_throw(
                            ToHit {
                                block_type: Grab,
                                hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            ActionId::Mizku(MizkuActionId::StandThrowHit),
                            ActionId::Mizku(MizkuActionId::StandThrowTarget),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(37),
                        ..default()
                    },
                ],
            ),
        ),
        (
            MizkuActionId::StandThrowHit,
            Action::throw_hit(MizkuAnimation::StandThrowHit, 85),
        ),
        (
            MizkuActionId::StandThrowTarget,
            Action::throw_target(
                MizkuAnimation::StandThrowTarget,
                30,
                false,
                10,
                Vec2::new(-2.0, 6.0),
            ),
        ),
        (
            MizkuActionId::CrouchThrow,
            Action::grounded(
                Some("[123]+w"),
                ActionCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::CrouchThrowStartup.into()],
                        exit_requirement: ContinuationRequirement::Time(5),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::forward_throw(
                            ToHit {
                                block_type: Grab,
                                hitbox: Hitbox(Area::of_size(0.5, 0.2)),
                                joint: Some(Joint::HandL),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            ActionId::Mizku(MizkuActionId::CrouchThrowHit),
                            ActionId::Mizku(MizkuActionId::CrouchThrowTarget),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(55),
                        ..default()
                    },
                ],
            ),
        ),
        (
            MizkuActionId::CrouchThrowHit,
            Action::throw_hit(MizkuAnimation::CrouchThrowHit, 85),
        ),
        (
            MizkuActionId::CrouchThrowTarget,
            Action::throw_target(
                MizkuAnimation::CrouchThrowTarget,
                50,
                false,
                10,
                Vec2::new(-5.0, 2.0),
            ),
        ),
        (
            MizkuActionId::AirThrow,
            Action::airborne(
                Some("w"),
                ActionCategory::Throw,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::AirThrowStartup.into()],
                        exit_requirement: ContinuationRequirement::Time(4),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::forward_throw(
                            ToHit {
                                block_type: Grab,
                                hitbox: Hitbox(Area::new(-0.2, 0.0, 0.8, 0.8)),
                                joint: Some(Joint::HandL),
                                lifetime: Lifetime::frames(2),
                                ..default()
                            },
                            ActionId::Mizku(MizkuActionId::AirThrowHit),
                            ActionId::Mizku(MizkuActionId::AirThrowTarget),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(36),
                        ..default()
                    },
                ],
            ),
        ),
        (
            MizkuActionId::AirThrowHit,
            Action::throw_hit(MizkuAnimation::AirThrowHit, 50),
        ),
        (
            MizkuActionId::AirThrowTarget,
            Action::throw_target_with_split_duration(
                MizkuAnimation::AirThrowTarget,
                30,
                false,
                50,
                10,
                Vec2::new(-2.0, 2.0),
            ),
        ),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (MizkuActionId, Action)> {
    sword_stance().chain(kunai_throw())
}

fn sword_stance() -> impl Iterator<Item = (MizkuActionId, Action)> {
    vec![
        (MizkuActionId::FSwordStance, take_sword_stance(false)),
        (MizkuActionId::SSwordStance, take_sword_stance(true)),
        (MizkuActionId::Sharpen, sharpen()),
        (MizkuActionId::ViperStrike, viper_strike()),
        (MizkuActionId::RisingSun, rising_sun()),
    ]
    .into_iter()
}

fn take_sword_stance(strong: bool) -> Action {
    let mut events = vec![MizkuAnimation::SwordStance.into()];
    let mut requirements = vec![ActionRequirement::Grounded];

    if strong {
        events.extend(vec![
            ModifyResource(ResourceType::Meter, 20),
            Condition(StatusCondition {
                flag: StatusFlag::Intangible,
                // 10f of sword stance + 11f of rising sun
                expiration: Some(22),
                ..default()
            }),
            Flash(FlashRequest::default()),
        ]);

        requirements.push(ActionRequirement::ResourceValue(ResourceType::Meter, 20))
    }

    Action::new(
        Some(if strong { "214s" } else { "214f" }),
        ActionCategory::Special,
        vec![
            ActionBlock {
                events,
                exit_requirement: ContinuationRequirement::Time(3),
                ..default()
            },
            ActionBlock {
                exit_requirement: ContinuationRequirement::Time(47),
                cancel_policy: CancelRule::specific(vec![
                    ActionId::Mizku(MizkuActionId::Sharpen),
                    ActionId::Mizku(MizkuActionId::ViperStrike),
                    ActionId::Mizku(MizkuActionId::RisingSun),
                ]),
                ..default()
            },
        ],
        requirements,
    )
}

fn sharpen() -> Action {
    Action::new(
        Some("g"),
        ActionCategory::Special,
        vec![
            ActionBlock {
                events: vec![MizkuAnimation::Sharpen.into()],
                exit_requirement: ContinuationRequirement::Time(45),
                ..default()
            },
            ActionBlock {
                events: vec![ModifyResource(ResourceType::Sharpness, 1)],
                ..default()
            },
        ],
        vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![
                ActionId::Mizku(MizkuActionId::FSwordStance),
                ActionId::Mizku(MizkuActionId::SSwordStance),
            ]),
        ],
    )
}

fn viper_strike() -> Action {
    Action::new(
        Some("s"),
        ActionCategory::Special,
        vec![
            ActionBlock {
                events: vec![MizkuAnimation::ViperStrike.into()],
                exit_requirement: ContinuationRequirement::Time(14),
                ..default()
            },
            ActionBlock {
                events: vec![],
                exit_requirement: ContinuationRequirement::Time(66),
                mutator: Some(|mut original: ActionBlock, situation: &Situation| {
                    original.events.push(
                        Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(-0.2, 0.0, 2.0, 0.2)),
                                joint: Some(Joint::Katana),
                                lifetime: Lifetime::frames(4),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 20
                                    + situation
                                        .get_resource(ResourceType::Sharpness)
                                        .unwrap()
                                        .current
                                        * 10,
                                on_hit: Stun(40),
                                on_block: Stun(30),
                                chip_damage: 5,
                                ..default()
                            },
                        )
                        .into(),
                    );

                    original
                }),
                ..default()
            },
        ],
        vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![
                ActionId::Mizku(MizkuActionId::FSwordStance),
                ActionId::Mizku(MizkuActionId::SSwordStance),
            ]),
        ],
    )
}

fn rising_sun() -> Action {
    Action::new(
        Some("f"),
        ActionCategory::Special,
        vec![
            ActionBlock {
                events: vec![MizkuAnimation::GrisingSun.into()],
                exit_requirement: ContinuationRequirement::Time(11),
                ..default()
            },
            ActionBlock {
                exit_requirement: ContinuationRequirement::Time(64),
                mutator: Some(|mut original: ActionBlock, situation: &Situation| {
                    original.events.push(
                        Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.0, 0.0, 2.0, 1.0)),
                                joint: Some(Joint::Katana),
                                lifetime: Lifetime::frames(6),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 20
                                    + situation
                                        .get_resource(ResourceType::Sharpness)
                                        .unwrap()
                                        .current
                                        * 10,
                                on_hit: Launcher(3.0),
                                on_block: Stun(40),
                                chip_damage: 10,
                                ..default()
                            },
                        )
                        .into(),
                    );

                    original
                }),
                cancel_policy: CancelRule::special_recovery(),
                ..default()
            },
        ],
        vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![
                ActionId::Mizku(MizkuActionId::FSwordStance),
                ActionId::Mizku(MizkuActionId::SSwordStance),
            ]),
        ],
    )
}

fn kunai_throw() -> impl Iterator<Item = (MizkuActionId, Action)> {
    vec![(
        MizkuActionId::KunaiThrow,
        Action::new(
            Some("236f"),
            ActionCategory::Special,
            vec![
                ActionBlock {
                    events: vec![
                        MizkuAnimation::KunaiThrow.into(),
                        ForceStand,
                        ModifyResource(ResourceType::KunaiCounter, -1),
                    ],
                    exit_requirement: ContinuationRequirement::Time(13),
                    ..default()
                },
                ActionBlock {
                    events: vec![Attack::strike(
                        ToHit {
                            hitbox: Hitbox(Area::new(1.0, 1.2, 0.3, 0.3)),
                            velocity: Some(Vec2::new(6.0, -0.4)),
                            lifetime: Lifetime::until_owner_hit(),
                            projectile: Some(Projectile {
                                model: Model::Kunai,
                            }),
                            ..default()
                        },
                        CommonAttackProps {
                            damage: 12,
                            on_hit: Stun(15),
                            on_block: Stun(10),
                            ..default()
                        },
                    )
                    .into()],
                    exit_requirement: ContinuationRequirement::Time(10),
                    cancel_policy: CancelRule::special_recovery(),
                    ..default()
                },
            ],
            vec![
                ActionRequirement::ResourceValue(ResourceType::KunaiCounter, 1),
                ActionRequirement::Grounded,
            ],
        ),
    )]
    .into_iter()
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    universal_item_actions(Animation::Mizku(MizkuAnimation::GiParry))
}

fn mizku_items() -> HashMap<ItemId, Item> {
    vec![
        (
            ItemId::SpareKunai,
            Item {
                cost: 75,
                explanation: "Two is better than one".into(),
                category: ItemCategory::Basic,
                icon: Icon::Kunai,
                effect: Stats {
                    extra_kunais: 1,
                    ..Stats::identity()
                },
            },
        ),
        (
            ItemId::KunaiPouch,
            Item {
                cost: 75,
                explanation: "5 uses for Kunai.\n\nThe more the merrier".into(),
                category: ItemCategory::Upgrade(vec![ItemId::SpareKunai]),
                icon: Icon::KunaiPouch,
                effect: Stats {
                    extra_kunais: 3,
                    ..Stats::identity()
                },
            },
        ),
        (
            ItemId::SpaceSuitBoots,
            Item {
                category: ItemCategory::Upgrade(vec![ItemId::Boots, ItemId::Dumbbell]),
                explanation: "Makes jumping stomp launch on hit\n\nAnd we have liftoff".into(),
                cost: 800,
                icon: Icon::SpaceSuitBoots,
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
                    ..Stats::identity()
                },
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
                    ..Stats::identity()
                },
            },
        ),
    ]
    .into_iter()
    .chain(universal_items())
    .collect()
}
