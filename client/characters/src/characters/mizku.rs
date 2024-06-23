use std::iter::{empty, once};

use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionId, Animation, AnimationType, Area, GameButton, Icon, ItemId, Joint, MizkuActionId,
    MizkuAnimation, Model, Stats, StatusCondition, StatusFlag, StickPosition,
    CHARGE_BAR_FULL_SEGMENT_COLOR, CHARGE_BAR_PARTIAL_SEGMENT_COLOR, FPS, MIZUKI_ALT_HELMET_COLOR,
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
    CancelRule, ChargeProperty, CommonAttackProps, ConsumableType, ContinuationRequirement,
    CounterVisual, FlashRequest, Hitbox, Item, ItemCategory, Lifetime, Movement, ResourceBarVisual,
    Situation, SpecialProperty,
    StunType::*,
    ToHit, WAGResource,
};

use super::{
    equipment::{universal_item_actions, universal_items},
    Character,
};

pub fn mizku() -> Character {
    let (jumps, gravity) = super::jumps(1.7, 1.0, Animation::Mizku(MizkuAnimation::Jump));

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
                ResourceType::ItemCount(ItemId::Kunai),
                WAGResource {
                    render_instructions: RenderInstructions::Counter(CounterVisual {
                        label: "Kunais",
                    }),
                    ..default()
                },
            ),
            (
                ResourceType::Charge,
                WAGResource {
                    max: Some((FPS / 2.) as i32), // Frames to full,
                    special: Some(SpecialProperty::Charge(ChargeProperty {
                        directions: vec![StickPosition::SW, StickPosition::S, StickPosition::SE],
                        ..default()
                    })),
                    render_instructions: RenderInstructions::Bar(ResourceBarVisual {
                        default_color: CHARGE_BAR_PARTIAL_SEGMENT_COLOR,
                        full_color: Some(CHARGE_BAR_FULL_SEGMENT_COLOR),
                        ..default()
                    }),
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
    empty()
        .chain(jumps)
        .chain(super::dashes(
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
                8,
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
                        exit_requirement: ContinuationRequirement::Time(18),
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
                        exit_requirement: ContinuationRequirement::Time(26),
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
                20,
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
            MizkuActionId::LowStab,
            Action::grounded(
                Some("[123]+g"),
                ActionCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::LowStab.into()],
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
            Action::throw_hit(MizkuAnimation::StandThrowHit, 20),
        ),
        (
            MizkuActionId::StandThrowTarget,
            Action::throw_target(MizkuAnimation::StandThrowTarget, 30, false, 10, Vec2::ZERO),
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
            Action::throw_hit(MizkuAnimation::CrouchThrowHit, 65),
        ),
        (
            MizkuActionId::CrouchThrowTarget,
            Action::throw_target(
                MizkuAnimation::CrouchThrowTarget,
                21,
                true,
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
            Action::throw_hit(MizkuAnimation::AirThrowHit, 30),
        ),
        (
            MizkuActionId::AirThrowTarget,
            Action::throw_target_with_split_duration(
                MizkuAnimation::AirThrowTarget,
                3,
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
    rising_suns().chain(sway()).chain(once((
        MizkuActionId::Sharpen,
        Action::grounded(
            Some("214g"),
            ActionCategory::Special,
            vec![
                ActionBlock {
                    events: vec![MizkuAnimation::Sharpen.into()],
                    exit_requirement: ContinuationRequirement::Time(48),
                    ..default()
                },
                ActionBlock {
                    events: vec![
                        ModifyResource(ResourceType::Sharpness, 1),
                        ModifyResource(ResourceType::Meter, 20),
                    ],
                    exit_requirement: ContinuationRequirement::Time(32),
                    // Since there is no hitbox, you can't cancel this under normal circumstances
                    // as it can never hit, which is requried for neutral normal cancellation.
                    cancel_policy: CancelRule::neutral_normal_recovery(),
                    mutator: None,
                },
            ],
        ),
    )))
}

macro_rules! rising_sun {
    ( $air:expr, $button:literal, $charged:expr ) => {
        Action::new(
            Some(concat!("[123][789]", $button)),
            ActionCategory::Special,
            vec![
                ActionBlock {
                    events: {
                        let mut events = vec![if $air {
                            MizkuAnimation::ArisingSun
                        } else {
                            MizkuAnimation::GrisingSun
                        }
                        .into()];

                        if !$air {
                            events.push(ForceStand);
                        }

                        if !$charged {
                            events.push(if $button == "s" {
                                ModifyResource(ResourceType::Sharpness, -1)
                            } else {
                                ClearResource(ResourceType::Sharpness)
                            });
                        }

                        if $button == "s" {
                            if $charged {
                                events.push(Condition(StatusCondition {
                                    flag: StatusFlag::Intangible,
                                    effect: None,
                                    expiration: Some(20),
                                }));
                            }

                            events.extend(vec![
                                Flash(FlashRequest {
                                    duration: 0.5,
                                    ..default()
                                }),
                                ModifyResource(ResourceType::Meter, -40),
                            ]);
                        }

                        events
                    },
                    exit_requirement: ContinuationRequirement::Time(11),
                    ..default()
                },
                ActionBlock {
                    exit_requirement: ContinuationRequirement::Time(if $air { 79 } else { 64 }),
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
                                    damage: if $button == "s" { 20 } else { 10 }
                                        + situation
                                            .get_resource(ResourceType::Sharpness)
                                            .unwrap()
                                            .current
                                            * if $air { 5 } else { 10 },
                                    on_hit: Launcher(if $air { 4.0 } else { 6.0 }),
                                    on_block: Stun(if $air { 30 } else { 40 }),
                                    chip_damage: if $button == "s" { 10 } else { 5 },
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
            {
                let mut requirements = vec![if $air {
                    ActionRequirement::Airborne
                } else {
                    ActionRequirement::Grounded
                }];

                if $button == "s" {
                    requirements.push(ActionRequirement::ResourceValue(ResourceType::Meter, 40));
                }

                if $charged {
                    requirements.push(ActionRequirement::ResourceFull(ResourceType::Charge));
                }

                requirements
            },
        )
    };
}

fn rising_suns() -> impl Iterator<Item = (MizkuActionId, Action)> {
    vec![
        (
            MizkuActionId::GrisingSunChargedS,
            rising_sun!(false, "s", true),
        ),
        (
            MizkuActionId::ArisingSunChargedS,
            rising_sun!(true, "s", true),
        ),
        (
            MizkuActionId::GrisingSunUnchargedS,
            rising_sun!(false, "s", false),
        ),
        (
            MizkuActionId::ArisingSunUnchargedS,
            rising_sun!(true, "s", false),
        ),
        (
            MizkuActionId::GrisingSunChargedF,
            rising_sun!(false, "f", true),
        ),
        (
            MizkuActionId::ArisingSunChargedF,
            rising_sun!(true, "f", true),
        ),
        (
            MizkuActionId::GrisingSunUnchargedF,
            rising_sun!(false, "f", false),
        ),
        (
            MizkuActionId::ArisingSunUnchargedF,
            rising_sun!(true, "f", false),
        ),
    ]
    .into_iter()
}

fn sway() -> impl Iterator<Item = (MizkuActionId, Action)> {
    vec![
        (
            MizkuActionId::ShortBackSway,
            Action::grounded(
                Some("214f"),
                ActionCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::Sway.into(),
                            Movement {
                                amount: -Vec2::X * 2.0,
                                duration: 3,
                            }
                            .into(),
                        ],
                        cancel_policy: CancelRule::specific_or_category(
                            vec![
                                ActionId::Mizku(MizkuActionId::ShortSwayDash),
                                ActionId::Mizku(MizkuActionId::SwayCancel),
                            ],
                            ActionCategory::Super,
                        ),
                        exit_requirement: ContinuationRequirement::Time(3),
                        mutator: Some(|mut original: ActionBlock, situation: &Situation| {
                            if situation.inventory.contains(&ItemId::GentlemansPipe) {
                                original.events.push(Condition(StatusCondition {
                                    flag: StatusFlag::Intangible,
                                    effect: None,
                                    expiration: Some(20),
                                }));
                            }
                            original
                        }),
                    },
                    ActionBlock {
                        events: vec![Movement {
                            amount: -Vec2::X * 8.0,
                            duration: 10,
                        }
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(37),
                        cancel_policy: CancelRule::specific_or_category(
                            vec![
                                ActionId::Mizku(MizkuActionId::ShortSwayDash),
                                ActionId::Mizku(MizkuActionId::SwayCancel),
                            ],
                            ActionCategory::Super,
                        ),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::LongBackSway,
            Action::new(
                Some("214s"),
                ActionCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::Sway.into(),
                            Movement {
                                amount: -Vec2::X * 5.0,
                                duration: 3,
                            }
                            .into(),
                            ModifyResource(ResourceType::Meter, -20),
                            Flash(FlashRequest {
                                duration: 0.3,
                                ..default()
                            }),
                        ],
                        cancel_policy: CancelRule::specific_or_category(
                            vec![
                                ActionId::Mizku(MizkuActionId::LongSwayDash),
                                ActionId::Mizku(MizkuActionId::SwayCancel),
                            ],
                            ActionCategory::Super,
                        ),
                        exit_requirement: ContinuationRequirement::Time(3),
                        mutator: Some(|mut original: ActionBlock, situation: &Situation| {
                            if situation.inventory.contains(&ItemId::GentlemansPipe) {
                                original.events.push(Condition(StatusCondition {
                                    flag: StatusFlag::Intangible,
                                    effect: None,
                                    expiration: Some(30),
                                }));
                            }
                            original
                        }),
                    },
                    ActionBlock {
                        events: vec![Movement {
                            amount: -Vec2::X * 8.0,
                            duration: 10,
                        }
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(37),
                        cancel_policy: CancelRule::specific_or_category(
                            vec![
                                ActionId::Mizku(MizkuActionId::LongSwayDash),
                                ActionId::Mizku(MizkuActionId::SwayCancel),
                            ],
                            ActionCategory::Super,
                        ),
                        mutator: None,
                    },
                ],
                vec![
                    ActionRequirement::Grounded,
                    ActionRequirement::ResourceValue(ResourceType::Meter, 20),
                ],
            ),
        ),
        (
            MizkuActionId::ShortSwayDash,
            Action::new(
                Some("s"),
                ActionCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::SwayDash.into(), ClearMovement],
                        exit_requirement: ContinuationRequirement::Time(5),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![
                            // Overlap with each other to add more in the beginning
                            Movement {
                                amount: Vec2::X * 8.0,
                                duration: 8,
                            }
                            .into(),
                            Movement {
                                amount: Vec2::X * 3.0,
                                duration: 16,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(16),
                        cancel_policy: CancelRule::specific_or_category(
                            vec![
                                ActionId::Mizku(MizkuActionId::SwayOverhead),
                                ActionId::Mizku(MizkuActionId::SwayLow),
                                ActionId::Mizku(MizkuActionId::Pilebunker),
                            ],
                            ActionCategory::Super,
                        ),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![
                            ClearMovement,
                            Movement {
                                amount: Vec2::X * 1.5,
                                duration: 8,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(16),
                        cancel_policy: CancelRule::specific_or_category(
                            vec![
                                ActionId::Mizku(MizkuActionId::SwayOverhead),
                                ActionId::Mizku(MizkuActionId::SwayLow),
                                ActionId::Mizku(MizkuActionId::Pilebunker),
                            ],
                            ActionCategory::Super,
                        ),
                        ..default()
                    },
                ],
                vec![ActionRequirement::ActionOngoing(vec![ActionId::Mizku(
                    MizkuActionId::ShortBackSway,
                )])],
            ),
        ),
        (
            MizkuActionId::LongSwayDash,
            Action::new(
                Some("f"),
                ActionCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::SwayDash.into(), ClearMovement],
                        exit_requirement: ContinuationRequirement::Time(5),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![
                            // Overlap with each other to add more in the beginning
                            Movement {
                                amount: Vec2::X * 10.0,
                                duration: 8,
                            }
                            .into(),
                            Movement {
                                amount: Vec2::X * 5.0,
                                duration: 16,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(16),
                        cancel_policy: CancelRule::specific_or_category(
                            vec![
                                ActionId::Mizku(MizkuActionId::SwayCancel),
                                ActionId::Mizku(MizkuActionId::SwayOverhead),
                                ActionId::Mizku(MizkuActionId::SwayLow),
                                ActionId::Mizku(MizkuActionId::Pilebunker),
                            ],
                            ActionCategory::Super,
                        ),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![
                            ClearMovement,
                            Movement {
                                amount: Vec2::X * 3.0,
                                duration: 8,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(16),
                        cancel_policy: CancelRule::specific_or_category(
                            vec![
                                ActionId::Mizku(MizkuActionId::SwayCancel),
                                ActionId::Mizku(MizkuActionId::SwayOverhead),
                                ActionId::Mizku(MizkuActionId::SwayLow),
                                ActionId::Mizku(MizkuActionId::Pilebunker),
                            ],
                            ActionCategory::Super,
                        ),
                        ..default()
                    },
                ],
                vec![ActionRequirement::ActionOngoing(vec![ActionId::Mizku(
                    MizkuActionId::LongBackSway,
                )])],
            ),
        ),
        (
            MizkuActionId::SwayCancel,
            Action::new(
                Some("g"),
                ActionCategory::Special,
                vec![ActionBlock {
                    events: vec![MizkuAnimation::SwayCancel.into()],
                    exit_requirement: ContinuationRequirement::Time(10),
                    cancel_policy: CancelRule::special_recovery(),
                    ..default()
                }],
                vec![ActionRequirement::ActionOngoing(vec![
                    ActionId::Mizku(MizkuActionId::LongBackSway),
                    ActionId::Mizku(MizkuActionId::LongSwayDash), // Long backdash can be cancelled, short cannot
                    ActionId::Mizku(MizkuActionId::ShortBackSway),
                ])],
            ),
        ),
        (
            MizkuActionId::SwayOverhead,
            Action::new(
                Some("6+w"),
                ActionCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::SwayOverhead.into()],
                        exit_requirement: ContinuationRequirement::Time(20),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.0, 0.0, 1.0, 1.0)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(3),
                                block_type: Strike(High),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 11,
                                chip_damage: 1,
                                on_hit: Stun(24),
                                on_block: Stun(16),
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(20),
                        cancel_policy: CancelRule::special_recovery(),
                        ..default()
                    },
                ],
                vec![ActionRequirement::ActionOngoing(vec![
                    ActionId::Mizku(MizkuActionId::LongSwayDash),
                    ActionId::Mizku(MizkuActionId::ShortSwayDash),
                ])],
            ),
        ),
        (
            MizkuActionId::SwayLow,
            Action::new(
                Some("[123]+w"),
                ActionCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::SwayLow.into()],
                        exit_requirement: ContinuationRequirement::Time(14),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![
                            Movement {
                                amount: Vec2::X * 20.0,
                                duration: 16,
                            }
                            .into(),
                            Attack::strike(
                                ToHit {
                                    hitbox: Hitbox(Area::new(0.0, 0.0, 1.0, 1.0)),
                                    joint: Some(Joint::FootL),
                                    lifetime: Lifetime::frames(20),
                                    block_type: Strike(Low),
                                    ..default()
                                },
                                CommonAttackProps {
                                    damage: 10,
                                    chip_damage: 1,
                                    on_block: Stun(16),
                                    on_hit: Launcher(0.0),
                                    ..default()
                                },
                            )
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(61),
                        cancel_policy: CancelRule::special_recovery(),
                        ..default()
                    },
                ],
                vec![ActionRequirement::ActionOngoing(vec![
                    ActionId::Mizku(MizkuActionId::LongSwayDash),
                    ActionId::Mizku(MizkuActionId::ShortSwayDash),
                ])],
            ),
        ),
        (
            MizkuActionId::Pilebunker,
            Action::new(
                Some("w"),
                ActionCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::Pilebunker.into()],
                        exit_requirement: ContinuationRequirement::Time(23),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.0, 0.0, 1.0, 1.0)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 30, // It should hurt
                                chip_damage: 1,
                                on_hit: Roller(Vec2::new(10.0, 2.0)),
                                on_block: Stun(20),
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(37),
                        cancel_policy: CancelRule::special_recovery(),
                        ..default()
                    },
                ],
                vec![ActionRequirement::ActionOngoing(vec![
                    ActionId::Mizku(MizkuActionId::LongSwayDash),
                    ActionId::Mizku(MizkuActionId::ShortSwayDash),
                ])],
            ),
        ),
    ]
    .into_iter()
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    vec![
        (
            MizkuActionId::KunaiThrow,
            Action::new(
                Some("236f"),
                ActionCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::KunaiThrow.into(),
                            ForceStand,
                            Consume(ItemId::Kunai),
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
                    ActionRequirement::ItemsOwned(vec![ItemId::Kunai]),
                    ActionRequirement::Grounded,
                ],
            ),
        ),
        (
            MizkuActionId::Overhead,
            Action::new(
                Some("6+s"),
                ActionCategory::NeutralNormal,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::Overhead.into(),
                            Movement {
                                amount: Vec2::X * 10.0,
                                duration: 20,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(25),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![
                            Attack::strike(
                                ToHit {
                                    hitbox: Hitbox(Area::new(-0.2, 0.0, 1.2, 0.2)),
                                    joint: Some(Joint::FootR),
                                    lifetime: Lifetime::frames(5),
                                    block_type: Strike(High),
                                    ..default()
                                },
                                CommonAttackProps {
                                    damage: 10,
                                    on_hit: Stun(40),
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
                        exit_requirement: ContinuationRequirement::Time(35),
                        cancel_policy: CancelRule::command_normal_recovery(),
                        mutator: None,
                    },
                ],
                vec![
                    ActionRequirement::ItemsOwned(vec![ItemId::SteelHeelBoots]),
                    ActionRequirement::Grounded,
                ],
            ),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (ActionId::Mizku(k), v))
    .chain(universal_item_actions(Animation::Mizku(
        MizkuAnimation::GiParry,
    )))
}

fn mizku_items() -> HashMap<ItemId, Item> {
    vec![
        (
            ItemId::Kunai,
            Item {
                cost: 75,
                explanation: "qcf+f to throw, comes in handy\n\nThat's the power...of a president!"
                    .into(),
                category: ItemCategory::Consumable(ConsumableType::UntilUsed),
                icon: Icon::Kunai,
                ..default()
            },
        ),
        (
            ItemId::SteelHeelBoots,
            Item {
                cost: 300,
                explanation: "6s for an overhead".into(),
                category: ItemCategory::Upgrade(vec![ItemId::SafetyBoots, ItemId::HockeyPads]),
                ..default()
            },
        ),
        (
            ItemId::GentlemansPipe,
            Item {
                cost: 300,
                explanation: "Intangibility in sway".into(),
                category: ItemCategory::Upgrade(vec![ItemId::Cigarettes]),
                ..default()
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
    ]
    .into_iter()
    .chain(universal_items())
    .collect()
}
