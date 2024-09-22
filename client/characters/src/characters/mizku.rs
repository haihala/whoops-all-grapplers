use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionCategory, ActionId, Animation, AnimationType, Area, CancelType, CancelWindow, GameButton,
    Icon, ItemId, Joint, MizkuActionId, MizkuAnimation, Model, SoundEffect, Stats, StatusCondition,
    StatusFlag, MIZUKI_ALT_HELMET_COLOR, MIZUKI_ALT_JEANS_COLOR, MIZUKI_ALT_SHIRT_COLOR,
};

use crate::{
    actions::{ActionRequirement, Projectile},
    air_action, dashes, ground_action, jumps,
    resources::{RenderInstructions, ResourceType},
    throw_hit, throw_target, universal_item_actions, Action, ActionEvent, Attack,
    AttackHeight::*,
    BlockType::*,
    CommonAttackProps, ConsumableType, CounterVisual, Hitbox, Item, ItemCategory, Lifetime,
    Movement, Situation,
    StunType::*,
    ToHit, WAGResource,
};

use super::{equipment::universal_items, Character};

pub fn mizku() -> Character {
    let (jumps, gravity) = jumps!(2.1, 1.1, Animation::Mizku(MizkuAnimation::Jump));

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
        .chain(dashes!(
            MizkuAnimation::DashForward,
            MizkuAnimation::DashBack
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
            ground_action!(
                "f",
                ActionCategory::Normal,
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
                        on_block: 15,
                        ..default()
                    },
                ),
                16
            ),
        ),
        (
            MizkuActionId::LowKick,
            ground_action!(
                "[123]+f",
                ActionCategory::Normal,
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
                        on_block: 11,
                        ..default()
                    },
                ),
                12
            ),
        ),
        (
            MizkuActionId::HeelKick,
            Action {
                input: Some("s"),
                category: ActionCategory::Normal,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![
                            MizkuAnimation::HeelKick.into(),
                            Movement {
                                amount: Vec2::X * 10.0,
                                duration: 20,
                            }
                            .into(),
                            SoundEffect::FemaleExhale.into(),
                        ];
                    }

                    if situation.elapsed() == 9 {
                        return vec![
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
                                    on_block: 20,
                                    ..default()
                                },
                            )
                            .into(),
                            ActionEvent::AllowCancel(CancelWindow {
                                require_hit: true,
                                cancel_type: CancelType::Special,
                                duration: 20,
                            }),
                            Movement {
                                amount: Vec2::X * 3.0,
                                duration: 10,
                            }
                            .into(),
                        ];
                    }

                    situation.end_at(37)
                },
                requirements: vec![ActionRequirement::Grounded],
            },
        ),
        (
            MizkuActionId::Uppercut,
            ground_action!(
                "[123]+s",
                ActionCategory::Normal,
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
                        on_hit: Launch(Vec2::new(1.0, 6.0)),
                        on_block: 10,
                        ..default()
                    },
                ),
                40
            ),
        ),
        (
            MizkuActionId::HighStab,
            Action {
                input: Some("g"),
                category: ActionCategory::Normal,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![MizkuAnimation::HighStab.into()];
                    }

                    if situation.elapsed() == 7 {
                        return vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.3, 0.0, 1.8, 0.2)),
                                joint: Some(Joint::Katana),
                                lifetime: Lifetime::frames(6),
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
                                on_block: 30,
                                chip_damage: 5,
                                ..default()
                            },
                        )
                        .into()];
                    }

                    situation.end_at(53)
                },
                requirements: vec![ActionRequirement::Grounded],
            },
        ),
        (
            MizkuActionId::SkySlash,
            Action {
                input: Some("[123]+g"),
                category: ActionCategory::Normal,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![MizkuAnimation::SkyStab.into()];
                    }

                    if situation.elapsed() == 8 {
                        return vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.3, 0.4, 1.0, 1.0)),
                                joint: Some(Joint::Katana),
                                lifetime: Lifetime::frames(5),
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
                                on_block: 25,
                                chip_damage: 3,
                                ..default()
                            },
                        )
                        .into()];
                    }

                    situation.end_at(40)
                },
                requirements: vec![ActionRequirement::Grounded],
            },
        ),
        (
            MizkuActionId::AirSlice,
            Action {
                input: Some("g"),
                category: ActionCategory::Normal,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![MizkuAnimation::AirStab.into()];
                    }

                    if situation.elapsed() == 7 {
                        return vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(-0.2, -0.3, 1.0, 0.4)),
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
                                on_block: 30,
                                chip_damage: 5,
                                ..default()
                            },
                        )
                        .into()];
                    }

                    situation.end_at(70)
                },
                requirements: vec![ActionRequirement::Airborne],
            },
        ),
        (
            MizkuActionId::FalconKnee,
            air_action!(
                "f",
                ActionCategory::Normal,
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
                23
            ),
        ),
        (
            MizkuActionId::FootDive,
            Action {
                input: Some("s"),
                category: ActionCategory::Normal,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![
                            MizkuAnimation::FootDiveHold.into(),
                            Movement {
                                amount: Vec2::Y * -1.0,
                                duration: 7,
                            }
                            .into(),
                        ];
                    }

                    // TODO: Add an item to speed this up for instant overheads
                    if situation.elapsed() >= 20
                        && !situation.held_buttons.contains(&GameButton::Strong)
                    {
                        return vec![
                            MizkuAnimation::FootDiveRelease.into(),
                            // TODO: There used to be a 3f delay after the animation, but new
                            // system makes that hard, maybe think of a way to reintroduce that.
                            Attack::strike(
                                ToHit {
                                    hitbox: Hitbox(Area::new(0.1, 0.0, 0.7, 0.3)),
                                    joint: Some(Joint::FootR),
                                    lifetime: Lifetime::frames(7),
                                    block_type: Strike(High),
                                    ..default()
                                },
                                CommonAttackProps {
                                    damage: 18,
                                    push_back: 1.0,
                                    knock_back: 0.8,
                                    on_hit: if situation.inventory.contains(&ItemId::SpaceSuitBoots)
                                    {
                                        Launch(Vec2::new(-1.0, 15.0))
                                    } else {
                                        Stun(40)
                                    },
                                    on_block: 25,
                                    ..default()
                                },
                            )
                            .into(),
                        ];
                    }

                    vec![]
                },
                requirements: vec![ActionRequirement::Airborne],
            },
        ),
        (
            MizkuActionId::ForwardThrow,
            ground_action!(
                "w",
                ActionCategory::Throw,
                MizkuAnimation::StandThrowStartup,
                3,
                Attack::forward_throw(
                    ToHit {
                        block_type: Grab,
                        hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                        joint: Some(Joint::HandR),
                        lifetime: Lifetime::frames(3),
                        ..default()
                    },
                    ActionId::Mizku(MizkuActionId::StandThrowHit),
                    ActionId::Mizku(MizkuActionId::StandThrowTarget),
                ),
                37
            ),
        ),
        (
            MizkuActionId::BackThrow,
            ground_action!(
                "4+w",
                ActionCategory::Throw,
                MizkuAnimation::StandThrowStartup,
                3,
                Attack::back_throw(
                    ToHit {
                        block_type: Grab,
                        hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                        joint: Some(Joint::HandR),
                        lifetime: Lifetime::frames(3),
                        ..default()
                    },
                    ActionId::Mizku(MizkuActionId::StandThrowHit),
                    ActionId::Mizku(MizkuActionId::StandThrowTarget),
                ),
                37
            ),
        ),
        (
            MizkuActionId::StandThrowHit,
            throw_hit!(MizkuAnimation::StandThrowHit, 80),
        ),
        (
            MizkuActionId::StandThrowTarget,
            throw_target!(
                MizkuAnimation::StandThrowTarget,
                30,
                10,
                Vec2::new(-2.0, 6.0)
            ),
        ),
        (
            MizkuActionId::CrouchThrow,
            ground_action!(
                "[123]+w",
                ActionCategory::Normal,
                MizkuAnimation::CrouchThrowStartup,
                5,
                Attack::forward_throw(
                    ToHit {
                        block_type: Grab,
                        hitbox: Hitbox(Area::of_size(0.5, 0.2)),
                        joint: Some(Joint::HandL),
                        lifetime: Lifetime::frames(3),
                        ..default()
                    },
                    ActionId::Mizku(MizkuActionId::CrouchThrowHit),
                    ActionId::Mizku(MizkuActionId::CrouchThrowTarget),
                ),
                55
            ),
        ),
        (
            MizkuActionId::CrouchThrowHit,
            throw_hit!(MizkuAnimation::CrouchThrowHit, 80),
        ),
        (
            MizkuActionId::CrouchThrowTarget,
            throw_target!(
                MizkuAnimation::CrouchThrowTarget,
                34,
                10,
                Vec2::new(-5.0, 2.0)
            ),
        ),
        (
            MizkuActionId::AirThrow,
            air_action!(
                "w",
                ActionCategory::Throw,
                MizkuAnimation::AirThrowStartup,
                4,
                Attack::forward_throw(
                    ToHit {
                        block_type: Grab,
                        hitbox: Hitbox(Area::new(-0.2, 0.0, 0.8, 0.8)),
                        joint: Some(Joint::HandL),
                        lifetime: Lifetime::frames(2),
                        ..default()
                    },
                    ActionId::Mizku(MizkuActionId::AirThrowHit),
                    ActionId::Mizku(MizkuActionId::AirThrowTarget),
                ),
                36
            ),
        ),
        (
            MizkuActionId::AirThrowHit,
            throw_hit!(MizkuAnimation::AirThrowHit, 50),
        ),
        (
            MizkuActionId::AirThrowTarget,
            throw_target!(
                MizkuAnimation::AirThrowTarget,
                30,
                50,
                10,
                Vec2::new(-2.0, 2.0)
            ),
        ),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (MizkuActionId, Action)> {
    sword_stance().chain(kunai_throw())
}

macro_rules! sword_stance {
    ($strong:expr) => {{
        use wag_core::{ActionId, CancelType, CancelWindow};
        use $crate::FlashRequest;

        Action {
            input: Some(if $strong { "214s" } else { "214f" }),
            category: ActionCategory::Special,
            script: |situation: &Situation| {
                let mut events = vec![
                    MizkuAnimation::SwordStance.into(),
                    ActionEvent::AllowCancel(CancelWindow {
                        cancel_type: CancelType::Specific(
                            vec![
                                MizkuActionId::Sharpen,
                                MizkuActionId::ViperStrike,
                                MizkuActionId::RisingSun,
                            ]
                            .into_iter()
                            .map(|ma| ActionId::Mizku(ma))
                            .collect(),
                        ),
                        duration: 30,
                        require_hit: false,
                    }),
                ];

                if $strong {
                    events.extend(vec![
                        ActionEvent::ModifyResource(ResourceType::Meter, -20),
                        ActionEvent::Condition(StatusCondition {
                            flag: StatusFlag::Intangible,
                            // 10f of sword stance + 11f of rising sun
                            expiration: Some(22),
                            ..default()
                        }),
                        ActionEvent::Flash(FlashRequest::default()),
                    ]);
                }

                if situation.elapsed() == 0 {
                    return events;
                }

                if situation.elapsed() == 3 {
                    // TODO: Open cancel window
                }

                situation.end_at(38)
            },
            requirements: {
                let mut r = vec![ActionRequirement::Grounded];
                if $strong {
                    r.push(ActionRequirement::ResourceValue(ResourceType::Meter, 20));
                }
                r
            },
        }
    }};
}

fn sword_stance() -> impl Iterator<Item = (MizkuActionId, Action)> {
    vec![
        (MizkuActionId::FSwordStance, sword_stance!(false)),
        (MizkuActionId::SSwordStance, sword_stance!(true)),
        (MizkuActionId::Sharpen, sharpen()),
        (MizkuActionId::ViperStrike, viper_strike()),
        (MizkuActionId::RisingSun, rising_sun()),
    ]
    .into_iter()
}
fn sharpen() -> Action {
    Action {
        input: Some("g"),
        category: ActionCategory::FollowUp,
        script: |situation: &Situation| {
            if situation.elapsed() == 0 {
                return vec![MizkuAnimation::Sharpen.into()];
            }

            if situation.elapsed() == 43 {
                vec![
                    ActionEvent::ModifyResource(ResourceType::Sharpness, 1),
                    ActionEvent::End,
                ];
            }

            vec![]
        },
        requirements: vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![
                ActionId::Mizku(MizkuActionId::FSwordStance),
                ActionId::Mizku(MizkuActionId::SSwordStance),
            ]),
        ],
    }
}

fn viper_strike() -> Action {
    Action {
        input: Some("s"),
        category: ActionCategory::FollowUp,
        script: |situation: &Situation| {
            if situation.elapsed() == 0 {
                return vec![
                    MizkuAnimation::ViperStrike.into(),
                    Movement {
                        amount: Vec2::X * 8.0,
                        duration: 7,
                    }
                    .into(),
                ];
            }

            if situation.elapsed() == 8 {
                return vec![
                    Attack::strike(
                        ToHit {
                            hitbox: Hitbox(Area::new(0.4, 0.0, 1.6, 0.45)),
                            block_type: Strike(Low),
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
                            on_hit: Stun(40),
                            on_block: 30,
                            chip_damage: 5,
                            ..default()
                        },
                    )
                    .into(),
                    ActionEvent::AllowCancel(CancelWindow {
                        cancel_type: CancelType::Super,
                        require_hit: true,
                        duration: 30,
                    }),
                ];
            }

            situation.end_at(72)
        },
        requirements: vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![
                ActionId::Mizku(MizkuActionId::FSwordStance),
                ActionId::Mizku(MizkuActionId::SSwordStance),
            ]),
        ],
    }
}

fn rising_sun() -> Action {
    Action {
        input: Some("f"),
        category: ActionCategory::FollowUp,
        script: |situation: &Situation| {
            if situation.elapsed() == 0 {
                return vec![MizkuAnimation::GrisingSun.into()];
            }

            if situation.elapsed() == 3 {
                return vec![
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
                            on_hit: Launch(Vec2::new(1.0, 3.0)),
                            on_block: 40,
                            chip_damage: 10,
                            ..default()
                        },
                    )
                    .into(),
                    ActionEvent::AllowCancel(CancelWindow {
                        cancel_type: CancelType::Super,
                        require_hit: true,
                        duration: 30,
                    }),
                ];
            }
            situation.end_at(77)
        },
        requirements: vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![
                ActionId::Mizku(MizkuActionId::FSwordStance),
                ActionId::Mizku(MizkuActionId::SSwordStance),
            ]),
        ],
    }
}

fn kunai_throw() -> impl Iterator<Item = (MizkuActionId, Action)> {
    vec![(
        MizkuActionId::KunaiThrow,
        Action {
            input: Some("236f"),
            category: ActionCategory::Special,
            script: |situation: &Situation| {
                if situation.elapsed() == 0 {
                    return vec![
                        MizkuAnimation::KunaiThrow.into(),
                        ActionEvent::ForceStand,
                        ActionEvent::ModifyResource(ResourceType::KunaiCounter, -1),
                    ];
                }

                if situation.elapsed() == 13 {
                    return vec![
                        Attack::strike(
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
                                on_block: 10,
                                ..default()
                            },
                        )
                        .into(),
                        ActionEvent::AllowCancel(CancelWindow {
                            cancel_type: CancelType::Super,
                            require_hit: true,
                            duration: 10,
                        }),
                    ];
                }

                situation.end_at(23)
            },
            requirements: vec![
                ActionRequirement::ResourceValue(ResourceType::KunaiCounter, 1),
                ActionRequirement::Grounded,
            ],
        },
    )]
    .into_iter()
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    universal_item_actions!(Animation::Mizku(MizkuAnimation::GiParry))
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
                cost: 200,
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
            ItemId::KunaiBelt,
            Item {
                cost: 500,
                explanation: "8 uses for Kunai.\n\n8 is perfection.".into(),
                category: ItemCategory::Upgrade(vec![ItemId::KunaiPouch]),
                icon: Icon::KunaiBelt,
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
