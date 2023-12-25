use std::iter::empty;

use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionId, Animation, AnimationType, Area, GameButton, ItemId, Joint, MizkuActionId,
    MizkuAnimation, Model, Stats, StatusCondition, StatusFlag, StickPosition,
    CHARGE_BAR_FULL_SEGMENT_COLOR, CHARGE_BAR_PARTIAL_SEGMENT_COLOR, FPS, MIZUKI_ALT_HELMET_COLOR,
    MIZUKI_ALT_JEANS_COLOR, MIZUKI_ALT_SHIRT_COLOR,
};

use crate::{
    actions::{ActionRequirement, AnimationRequest, Projectile},
    resources::{RenderInstructions, ResourceType},
    Action, ActionBlock,
    ActionEvent::*,
    Attack,
    AttackHeight::*,
    BlockType::*,
    CancelCategory, CancelRule, ChargeProperty, CommonAttackProps, ContinuationRequirement,
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
    let (jumps, gravity) = super::jumps(1.8, 1.2, Animation::Mizku(MizkuAnimation::Jump));

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
            Action::grounded(
                Some("f"),
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::KneeThrust.into()],
                        exit_requirement: ContinuationRequirement::Time(5),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::strike(
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
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(17),
                        cancel_policy: CancelRule::neutral_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::LowKick,
            Action::grounded(
                Some("[123]+f"),
                CancelCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::LowKick.into()],
                        exit_requirement: ContinuationRequirement::Time(8),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(-0.4, 0.0, 0.9, 0.2)),
                                joint: Some(Joint::FootL),
                                lifetime: Lifetime::frames(3),
                                block_type: Strike(Low),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 8,
                                on_hit: Stun(21),
                                on_block: Stun(11),
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(12),
                        cancel_policy: CancelRule::command_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::HeelKick,
            Action::grounded(
                Some("s"),
                CancelCategory::Normal,
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
            Action::grounded(
                Some("[123]+s"),
                CancelCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::Uppercut.into()],
                        exit_requirement: ContinuationRequirement::Time(8),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::of_size(0.3, 0.5)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(8),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 16,
                                knock_back: 2.0,
                                on_hit: Launcher(6.0),
                                on_block: Stun(10),
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(20),
                        cancel_policy: CancelRule::command_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::FalconKnee,
            Action::airborne(
                Some("f"),
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::FalconKnee.into()],
                        exit_requirement: ContinuationRequirement::Time(2),
                        ..default()
                    },
                    ActionBlock {
                        // TODO: Add sweet and sour spots
                        events: vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.25)),
                                joint: Some(Joint::ShinR),
                                lifetime: Lifetime::frames(5),
                                block_type: Strike(High),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 5,
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(23),
                        cancel_policy: CancelRule::neutral_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::FootDive,
            Action::airborne(
                Some("s"),
                CancelCategory::Normal,
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
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::GroundThrowStartup.into()],
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
                            ActionId::Mizku(MizkuActionId::GroundThrowHit),
                            ActionId::Mizku(MizkuActionId::GroundThrowTarget),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(13),
                        ..default()
                    },
                ],
            ),
        ),
        (
            MizkuActionId::BackThrow,
            Action::grounded(
                Some("4+w"),
                CancelCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::GroundThrowStartup.into()],
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
                            ActionId::Mizku(MizkuActionId::GroundThrowHit),
                            ActionId::Mizku(MizkuActionId::GroundThrowTarget),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(13),
                        ..default()
                    },
                ],
            ),
        ),
        (
            MizkuActionId::GroundThrowHit,
            Action::grounded(
                None,
                CancelCategory::Special,
                vec![ActionBlock {
                    events: vec![MizkuAnimation::GroundThrowHit.into()],
                    exit_requirement: ContinuationRequirement::Time(20),
                    cancel_policy: CancelRule::never(),
                    mutator: None,
                }],
            ),
        ),
        (
            MizkuActionId::GroundThrowTarget,
            Action::grounded(
                None,
                CancelCategory::Uncancellable,
                vec![
                    ActionBlock {
                        events: vec![AnimationRequest {
                            animation: MizkuAnimation::GroundThrowTarget.into(),
                            invert: true,
                            ..default()
                        }
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(20),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![
                            ModifyResource(ResourceType::Health, -10),
                            Launch {
                                // This may be broken, but it may have been fixed while
                                // fixing flipping launch velocities
                                impulse: Vec2::new(2.0, 3.0),
                            },
                        ],
                        ..default()
                    },
                ],
            ),
        ),
        (
            MizkuActionId::Sweep,
            Action::grounded(
                Some("[123]+w"),
                CancelCategory::CommandNormal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::Sweep.into()],
                        exit_requirement: ContinuationRequirement::Time(7),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::strike(
                            ToHit {
                                block_type: Strike(Low),
                                hitbox: Hitbox(Area::new(-0.4, 0.2, 1.0, 0.3)),
                                joint: Some(Joint::FootR),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 9,
                                on_hit: Launcher(1.0),
                                on_block: Stun(10),
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(13),
                        cancel_policy: CancelRule::command_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::AirThrow,
            Action::airborne(
                Some("w"),
                CancelCategory::Normal,
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
                        exit_requirement: ContinuationRequirement::Time(9),
                        ..default()
                    },
                ],
            ),
        ),
        (
            MizkuActionId::AirThrowHit,
            Action::airborne(
                None,
                CancelCategory::Special,
                vec![ActionBlock {
                    events: vec![MizkuAnimation::AirThrowHit.into()],
                    exit_requirement: ContinuationRequirement::Time(30),
                    cancel_policy: CancelRule::never(),
                    mutator: None,
                }],
            ),
        ),
        (
            MizkuActionId::AirThrowTarget,
            Action::airborne(
                None,
                CancelCategory::Uncancellable,
                vec![ActionBlock {
                    events: vec![
                        AnimationRequest {
                            animation: MizkuAnimation::AirThrowTarget.into(),
                            invert: true,
                            ..default()
                        }
                        .into(),
                        ModifyResource(ResourceType::Health, -10),
                        Launch {
                            impulse: Vec2::new(-2.0, 2.0),
                        },
                    ],
                    exit_requirement: ContinuationRequirement::Time(60),
                    cancel_policy: CancelRule::never(),
                    mutator: None,
                }],
            ),
        ),
        (
            MizkuActionId::Sharpen,
            Action::grounded(
                Some("g"),
                CancelCategory::Normal,
                vec![
                    ActionBlock {
                        events: vec![MizkuAnimation::Sharpen.into()],
                        exit_requirement: ContinuationRequirement::Time(48),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![
                            ModifyResource(ResourceType::Sharpness, 1),
                            ModifyResource(ResourceType::Meter, 25),
                        ],
                        exit_requirement: ContinuationRequirement::Time(32),
                        // Since there is no hitbox, you can't cancel this under normal circumstances
                        // as it can never hit, which is requried for neutral normal cancellation.
                        cancel_policy: CancelRule::neutral_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (MizkuActionId, Action)> {
    rising_suns().chain(sway())
}

macro_rules! rising_sun {
    ( $air:expr, $button:literal, $charged:expr ) => {
        Action::new(
            Some(concat!("[123][789]", $button)),
            CancelCategory::Special,
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
                                    expiration: Some(12),
                                }));
                            }

                            events.extend(vec![
                                Flash(FlashRequest {
                                    duration: 0.5,
                                    ..default()
                                }),
                                ModifyResource(ResourceType::Meter, -50),
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
                                            .resources
                                            .iter()
                                            .find_map(|(rt, r)| {
                                                if rt == &ResourceType::Sharpness {
                                                    Some(r)
                                                } else {
                                                    None
                                                }
                                            })
                                            .unwrap()
                                            .current
                                            * if $air { 5 } else { 10 },
                                    on_hit: Launcher(if $air { 8.0 } else { 12.0 }),
                                    on_block: Stun(if $air { 30 } else { 40 }),
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
            {
                let mut requirements = vec![if $air {
                    ActionRequirement::Airborne
                } else {
                    ActionRequirement::Grounded
                }];

                if $button == "s" {
                    requirements.push(ActionRequirement::ResourceValue(ResourceType::Meter, 50));
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
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::BackSway.into(),
                            Movement {
                                amount: -Vec2::X * 2.0,
                                duration: 3,
                            }
                            .into(),
                        ],
                        cancel_policy: CancelRule::specific(vec![
                            ActionId::Mizku(MizkuActionId::ShortSwayDash),
                            ActionId::Mizku(MizkuActionId::SwayCancel),
                        ]),
                        exit_requirement: ContinuationRequirement::Time(3),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Movement {
                            amount: -Vec2::X * 8.0,
                            duration: 10,
                        }
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(37),
                        cancel_policy: CancelRule::specific(vec![
                            ActionId::Mizku(MizkuActionId::ShortSwayDash),
                            ActionId::Mizku(MizkuActionId::SwayCancel),
                        ]),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::LongBackSway,
            Action::new(
                Some("214s"),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::BackSway.into(),
                            Movement {
                                amount: -Vec2::X * 5.0,
                                duration: 3,
                            }
                            .into(),
                            ModifyResource(ResourceType::Meter, -25),
                            Flash(FlashRequest {
                                duration: 0.3,
                                ..default()
                            }),
                        ],
                        cancel_policy: CancelRule::specific(vec![
                            ActionId::Mizku(MizkuActionId::LongSwayDash),
                            ActionId::Mizku(MizkuActionId::SwayCancel),
                        ]),
                        exit_requirement: ContinuationRequirement::Time(3),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Movement {
                            amount: -Vec2::X * 8.0,
                            duration: 10,
                        }
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(37),
                        cancel_policy: CancelRule::specific(vec![
                            ActionId::Mizku(MizkuActionId::LongSwayDash),
                            ActionId::Mizku(MizkuActionId::SwayCancel),
                        ]),
                        mutator: None,
                    },
                ],
                vec![
                    ActionRequirement::Grounded,
                    ActionRequirement::ResourceValue(ResourceType::Meter, 25),
                ],
            ),
        ),
        (
            MizkuActionId::ShortSwayDash,
            Action::new(
                Some("s"),
                CancelCategory::Specific(vec![ActionId::Mizku(MizkuActionId::ShortBackSway)]),
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
                        cancel_policy: CancelRule::specific(vec![
                            ActionId::Mizku(MizkuActionId::SwayOverhead),
                            ActionId::Mizku(MizkuActionId::SwayLow),
                            ActionId::Mizku(MizkuActionId::Pilebunker),
                        ]),
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
                        cancel_policy: CancelRule::specific(vec![
                            ActionId::Mizku(MizkuActionId::SwayOverhead),
                            ActionId::Mizku(MizkuActionId::SwayLow),
                            ActionId::Mizku(MizkuActionId::Pilebunker),
                        ]),
                        ..default()
                    },
                ],
                vec![ActionRequirement::OngoingAction(vec![ActionId::Mizku(
                    MizkuActionId::ShortBackSway,
                )])],
            ),
        ),
        (
            MizkuActionId::LongSwayDash,
            Action::new(
                Some("f"),
                CancelCategory::Specific(vec![ActionId::Mizku(MizkuActionId::LongBackSway)]),
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
                        cancel_policy: CancelRule::specific(vec![
                            ActionId::Mizku(MizkuActionId::SwayCancel),
                            ActionId::Mizku(MizkuActionId::SwayOverhead),
                            ActionId::Mizku(MizkuActionId::SwayLow),
                            ActionId::Mizku(MizkuActionId::Pilebunker),
                        ]),
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
                        cancel_policy: CancelRule::specific(vec![
                            ActionId::Mizku(MizkuActionId::SwayCancel),
                            ActionId::Mizku(MizkuActionId::SwayOverhead),
                            ActionId::Mizku(MizkuActionId::SwayLow),
                            ActionId::Mizku(MizkuActionId::Pilebunker),
                        ]),
                        ..default()
                    },
                ],
                vec![ActionRequirement::OngoingAction(vec![ActionId::Mizku(
                    MizkuActionId::LongBackSway,
                )])],
            ),
        ),
        (
            MizkuActionId::SwayCancel,
            Action::new(
                Some("g"),
                CancelCategory::Specific(vec![
                    ActionId::Mizku(MizkuActionId::LongBackSway),
                    ActionId::Mizku(MizkuActionId::LongSwayDash), // Long backdash can be cancelled, short cannot
                    ActionId::Mizku(MizkuActionId::ShortBackSway),
                ]),
                vec![ActionBlock {
                    events: vec![MizkuAnimation::SwayCancel.into()],
                    exit_requirement: ContinuationRequirement::Time(10),
                    ..default()
                }],
                vec![ActionRequirement::OngoingAction(vec![
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
                CancelCategory::Specific(vec![
                    ActionId::Mizku(MizkuActionId::LongSwayDash),
                    ActionId::Mizku(MizkuActionId::ShortSwayDash),
                ]),
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
                                on_hit: Stun(24),
                                on_block: Stun(16),
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(20),
                        ..default()
                    },
                ],
                vec![ActionRequirement::OngoingAction(vec![
                    ActionId::Mizku(MizkuActionId::LongSwayDash),
                    ActionId::Mizku(MizkuActionId::ShortSwayDash),
                ])],
            ),
        ),
        (
            MizkuActionId::SwayLow,
            Action::new(
                Some("[123]+w"),
                CancelCategory::Specific(vec![
                    ActionId::Mizku(MizkuActionId::LongSwayDash),
                    ActionId::Mizku(MizkuActionId::ShortSwayDash),
                ]),
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
                                    on_block: Stun(16),
                                    on_hit: Launcher(0.0),
                                    ..default()
                                },
                            )
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(61),
                        ..default()
                    },
                ],
                vec![ActionRequirement::OngoingAction(vec![
                    ActionId::Mizku(MizkuActionId::LongSwayDash),
                    ActionId::Mizku(MizkuActionId::ShortSwayDash),
                ])],
            ),
        ),
        (
            MizkuActionId::Pilebunker,
            Action::new(
                Some("w"),
                CancelCategory::Specific(vec![
                    ActionId::Mizku(MizkuActionId::LongSwayDash),
                    ActionId::Mizku(MizkuActionId::ShortSwayDash),
                ]),
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
                                on_hit: Roller(Vec2::new(10.0, 2.0)),
                                on_block: Stun(20),
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(37),
                        ..default()
                    },
                ],
                vec![ActionRequirement::OngoingAction(vec![
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
                CancelCategory::Special,
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
                CancelCategory::Normal,
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
                category: ItemCategory::Consumable(crate::items::ConsumableType::UntilUsed),
                ..default()
            },
        ),
        (
            ItemId::SteelHeelBoots,
            Item {
                cost: 100,
                explanation: "6s for an overhead".into(),
                category: ItemCategory::Upgrade(vec![ItemId::SafetyBoots, ItemId::HockeyPads]),
                ..default()
            },
        ),
        (
            ItemId::SpaceSuitBoots,
            Item {
                category: ItemCategory::Upgrade(vec![ItemId::Boots, ItemId::Dumbbell]),
                explanation: "Makes j.h launch on hit\n\nAnd we have liftoff".into(),
                cost: 100,
                ..default()
            },
        ),
    ]
    .into_iter()
    .chain(universal_items())
    .collect()
}
