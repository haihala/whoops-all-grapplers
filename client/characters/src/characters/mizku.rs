use std::{collections::HashMap, iter::empty};

use bevy::prelude::*;

use wag_core::{
    ActionId, Animation, AnimationType, Area, GameButton, ItemId, Joint, MizkuActionId,
    MizkuAnimation, Model, Stats, StatusCondition, StatusFlag, FPS,
};

use crate::{
    actions::{ActionRequirement, AnimationRequest, Projectile},
    resources::{RenderInstructions, ResourceType},
    Action, ActionBlock,
    ActionEvent::*,
    Attack,
    AttackHeight::*,
    BlockType::*,
    CancelCategory, CancelPolicy, ChargeProperty, CommonAttackProps, ContinuationRequirement,
    CounterVisual, FlashRequest, Hitbox, Item, ItemCategory, Lifetime, Movement, ResourceBarVisual,
    SpecialProperty,
    StunType::*,
    ToHit, WAGResource,
};

use super::{
    dash,
    equipment::{universal_item_actions, universal_items},
    Character,
};

pub fn mizku() -> Character {
    Character::new(
        Model::Mizku,
        mizku_animations(),
        mizku_moves(),
        mizku_items(),
        2.0,
        1.0,
        Stats {
            walk_speed: 1.5,
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
                    special: Some(SpecialProperty::Charge(ChargeProperty::default())),
                    render_instructions: RenderInstructions::Bar(ResourceBarVisual {
                        default_color: Color::rgb(0.05, 0.4, 0.55),
                        full_color: Some(Color::rgb(0.9, 0.1, 0.3)),
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
        (AnimationType::Jump, MizkuAnimation::Jump),
    ]
    .into_iter()
    .map(|(k, v)| (k, Animation::from(v)))
    .collect()
}

fn mizku_moves() -> HashMap<ActionId, Action> {
    empty()
        .chain(item_actions())
        .chain(dashes())
        .chain(
            normals()
                .chain(specials())
                .map(|(k, v)| (ActionId::Mizku(k), v)),
        )
        .collect()
}

const DASH_DURATION: usize = 17;
const DASH_IMPULSE: f32 = 10.0;
fn dashes() -> impl Iterator<Item = (ActionId, Action)> {
    vec![
        (
            ActionId::DashForward,
            dash(
                "5656",
                DASH_DURATION,
                DASH_IMPULSE,
                MizkuAnimation::DashForward,
            ),
        ),
        (
            ActionId::DashBack,
            dash(
                "5454",
                DASH_DURATION,
                -DASH_IMPULSE,
                MizkuAnimation::DashBack,
            ),
        ),
    ]
    .into_iter()
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
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.35)),
                                joint: Some(Joint::ShinL),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(15),
                        cancel_policy: CancelPolicy::neutral_normal_recovery(),
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
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.1, -0.2, 0.3, 0.2)),
                                joint: Some(Joint::FootL),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(12),
                        cancel_policy: CancelPolicy::command_normal_recovery(),
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
                            Attack::new(
                                ToHit {
                                    hitbox: Hitbox(Area::new(-0.2, 0.0, 1.2, 0.2)),
                                    joint: Some(Joint::FootL),
                                    lifetime: Lifetime::frames(5),
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
                                amount: Vec2::X * 3.0,
                                duration: 10,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(26),
                        cancel_policy: CancelPolicy::neutral_normal_recovery(),
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
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::of_size(0.3, 0.5)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            CommonAttackProps {
                                knock_back: Vec2::new(-4.0, 3.0),
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(20),
                        cancel_policy: CancelPolicy::command_normal_recovery(),
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
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.25)),
                                joint: Some(Joint::ShinR),
                                lifetime: Lifetime::frames(5),
                                block_type: Constant(High),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(23),
                        cancel_policy: CancelPolicy::neutral_normal_recovery(),
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
                        exit_requirement: ContinuationRequirement::Time(5),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Movement {
                            amount: Vec2::Y * -1.0,
                            duration: 30,
                        }
                        .into()],
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
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::of_size(0.35, 0.25)),
                                joint: Some(Joint::FootR),
                                lifetime: Lifetime::frames(7),
                                block_type: Constant(High),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(17),
                        cancel_policy: CancelPolicy(vec![]),
                        mutator: None,
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
                        events: vec![Attack::new(
                            ToHit {
                                block_type: Grab,
                                hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 25,
                                on_hit: Launcher,
                                ..default()
                            },
                        )
                        .with_to_self_on_hit(vec![StartAction(ActionId::Mizku(
                            MizkuActionId::GroundThrowHit,
                        ))])
                        .with_to_target_on_hit(vec![
                            SnapToOpponent,
                            Animation(AnimationRequest {
                                animation: MizkuAnimation::GroundThrowTarget.into(),
                                invert: true,
                                ..default()
                            }),
                        ])
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
                        exit_requirement: ContinuationRequirement::Time(5),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::new(
                            ToHit {
                                block_type: Grab,
                                hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                                joint: Some(Joint::HandR),
                                lifetime: Lifetime::frames(5),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 25,
                                on_hit: Stun(30), // TODO: Not a launcher because target lands immediately. Needs more work
                                ..default()
                            },
                        )
                        .with_to_self_on_hit(vec![StartAction(ActionId::Mizku(
                            MizkuActionId::GroundThrowHit,
                        ))])
                        .with_to_target_on_hit(vec![
                            SnapToOpponent,
                            SideSwitch,
                            Animation(AnimationRequest {
                                animation: MizkuAnimation::GroundThrowTarget.into(),
                                invert: true,
                                ..default()
                            }),
                        ])
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(40),
                        ..default()
                    },
                ],
            ),
        ),
        (
            // TODO: Untested
            MizkuActionId::GroundThrowHit,
            Action::grounded(
                None,
                CancelCategory::Special,
                vec![ActionBlock {
                    events: vec![MizkuAnimation::GroundThrowHit.into()],
                    exit_requirement: ContinuationRequirement::Time(20),
                    cancel_policy: CancelPolicy::never(),
                    mutator: None,
                }],
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
                        events: vec![Attack::new(
                            ToHit {
                                block_type: Grab,
                                hitbox: Hitbox(Area::new(-0.3, 0.0, 1.0, 0.2)),
                                joint: Some(Joint::FootR),
                                lifetime: Lifetime::frames(3),
                                ..default()
                            },
                            CommonAttackProps {
                                knock_back: Vec2::Y * 8.0,
                                on_hit: Launcher,
                                ..default()
                            },
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(13),
                        cancel_policy: CancelPolicy::command_normal_recovery(),
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
                        events: vec![Attack::new(
                            ToHit {
                                block_type: Grab,
                                hitbox: Hitbox(Area::of_size(0.8, 0.8)),
                                joint: Some(Joint::HandL),
                                lifetime: Lifetime::frames(2),
                                ..default()
                            },
                            CommonAttackProps {
                                damage: 25,
                                on_hit: Launcher,
                                knock_back: Vec2::new(1.0, 2.0),
                                ..default()
                            },
                        )
                        .with_to_self_on_hit(vec![StartAction(ActionId::Mizku(
                            MizkuActionId::AirThrowHit,
                        ))])
                        .with_to_target_on_hit(vec![
                            SnapToOpponent,
                            Animation(AnimationRequest {
                                animation: MizkuAnimation::AirThrowTarget.into(),
                                invert: true,
                                ..default()
                            }),
                        ])
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(30),
                        ..default()
                    },
                ],
            ),
        ),
        (
            // TODO: Untested
            MizkuActionId::AirThrowHit,
            Action::grounded(
                None,
                CancelCategory::Special,
                vec![ActionBlock {
                    events: vec![MizkuAnimation::AirThrowHit.into()],
                    exit_requirement: ContinuationRequirement::Time(30),
                    cancel_policy: CancelPolicy::never(),
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
                        events: vec![ModifyResource(ResourceType::Sharpness, 1)],
                        exit_requirement: ContinuationRequirement::Time(32),
                        // Since there is no hitbox, you can't cancel this under normal circumstances
                        // as it can never hit, which is requried for neutral normal cancellation.
                        cancel_policy: CancelPolicy::neutral_normal_recovery(),
                        mutator: None,
                    },
                ],
            ),
        ),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (MizkuActionId, Action)> {
    vec![
        (
            MizkuActionId::GroundUpwardsSlash,
            Action::new(
                Some("[789]f"),
                CancelCategory::Special,
                vec![ActionBlock {
                    events: vec![
                        ForceStand,
                        MizkuAnimation::GroundUpwardsSlash.into(),
                        Condition(StatusCondition {
                            flag: StatusFlag::Intangible,
                            effect: None,
                            expiration: Some(20),
                        }),
                        Flash(FlashRequest {
                            duration: 0.5,
                            ..default()
                        }),
                    ],
                    exit_requirement: ContinuationRequirement::Time(45),
                    cancel_policy: CancelPolicy::never(),
                    mutator: None,
                }],
                vec![
                    ActionRequirement::Grounded,
                    ActionRequirement::ResourceFull(ResourceType::Charge),
                ],
            ),
        ),
        (
            MizkuActionId::AirUpwardsSlash,
            Action::new(
                Some("[789]f"),
                CancelCategory::Special,
                vec![ActionBlock {
                    events: vec![
                        MizkuAnimation::AirUpwardsSlash.into(),
                        Flash(FlashRequest {
                            duration: 0.5,
                            ..default()
                        }),
                    ],
                    exit_requirement: ContinuationRequirement::Time(45),
                    cancel_policy: CancelPolicy::never(),
                    mutator: None,
                }],
                vec![
                    ActionRequirement::Airborne,
                    ActionRequirement::ResourceFull(ResourceType::Charge),
                ],
            ),
        ),
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
                                amount: -Vec2::X * 5.0,
                                duration: 5,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(5),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Movement {
                            amount: -Vec2::X * 3.0,
                            duration: 15,
                        }
                        .into()],
                        exit_requirement: ContinuationRequirement::Conditions(vec![
                            ActionRequirement::ButtonNotPressed(GameButton::Fast),
                        ]),
                        cancel_policy: CancelPolicy::specific(
                            vec![
                                MizkuActionId::SwayDash,
                                MizkuActionId::ShortHighSlice,
                                MizkuActionId::ShortHorizontalSlice,
                                MizkuActionId::ShortLowSlice,
                            ]
                            .into_iter()
                            .map(ActionId::Mizku)
                            .collect(),
                        ),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::LongBackSway,
            Action::grounded(
                Some("214s"),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::BackSway.into(),
                            Movement {
                                amount: -Vec2::X * 10.0,
                                duration: 5,
                            }
                            .into(),
                            Flash(FlashRequest {
                                duration: 0.3,
                                ..default()
                            }),
                        ],
                        exit_requirement: ContinuationRequirement::Time(5),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Movement {
                            amount: -Vec2::X * 5.0,
                            duration: 15,
                        }
                        .into()],
                        exit_requirement: ContinuationRequirement::Conditions(vec![
                            ActionRequirement::ButtonNotPressed(GameButton::Strong),
                        ]),
                        cancel_policy: CancelPolicy::specific(
                            vec![
                                MizkuActionId::SwayDash,
                                MizkuActionId::LongHighSlice,
                                MizkuActionId::LongHorizontalSlice,
                                MizkuActionId::LongLowSlice,
                            ]
                            .into_iter()
                            .map(ActionId::Mizku)
                            .collect(),
                        ),
                        mutator: None,
                    },
                ],
            ),
        ),
        (
            MizkuActionId::SwayDash,
            Action::new(
                Some("656"),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![
                            MizkuAnimation::SwayDash.into(),
                            ClearMovement,
                            Movement {
                                amount: Vec2::X * 10.0,
                                duration: 12,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(4),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Movement {
                            amount: Vec2::X * 2.0,
                            duration: 8,
                        }
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(8),
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
                        exit_requirement: ContinuationRequirement::Time(8),
                        ..default()
                    },
                ],
                vec![ActionRequirement::OngoingAction(vec![
                    ActionId::Mizku(MizkuActionId::ShortBackSway),
                    ActionId::Mizku(MizkuActionId::LongBackSway),
                ])],
            ),
        ),
    ]
    .into_iter()
    .chain(sway_slashes())
}

fn sway_slashes() -> impl Iterator<Item = (MizkuActionId, Action)> {
    // TODO: Unique hitboxes and props for the slashes
    vec![
        (
            MizkuActionId::ShortHighSlice,
            "[789]+F",
            MizkuAnimation::HighSlice,
            Area::new(1.0, 0.7, 2.0, 0.7),
            MizkuActionId::ShortBackSway,
        ),
        (
            MizkuActionId::LongHighSlice,
            "[789]+S",
            MizkuAnimation::HighSlice,
            Area::new(1.0, 0.7, 2.0, 0.7),
            MizkuActionId::LongBackSway,
        ),
        (
            MizkuActionId::ShortHorizontalSlice,
            "[456]+F",
            MizkuAnimation::HorizontalSlice,
            Area::new(1.0, 0.0, 2.0, 0.5),
            MizkuActionId::ShortBackSway,
        ),
        (
            MizkuActionId::LongHorizontalSlice,
            "[456]+S",
            MizkuAnimation::HorizontalSlice,
            Area::new(1.0, 0.0, 2.0, 0.5),
            MizkuActionId::LongBackSway,
        ),
        (
            MizkuActionId::ShortLowSlice,
            "[123]+F",
            MizkuAnimation::LowSlice,
            Area::new(1.0, -0.2, 2.0, 0.3),
            MizkuActionId::ShortBackSway,
        ),
        (
            MizkuActionId::LongLowSlice,
            "[123]+S",
            MizkuAnimation::LowSlice,
            Area::new(1.0, -0.2, 2.0, 0.3),
            MizkuActionId::LongBackSway,
        ),
    ]
    .into_iter()
    .map(|(id, input, anim, area, criteria)| {
        (
            id,
            Action::new(
                Some(input),
                CancelCategory::Special,
                vec![
                    ActionBlock {
                        events: vec![anim.into(), ClearMovement],
                        exit_requirement: ContinuationRequirement::Time(4),
                        ..default()
                    },
                    ActionBlock {
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(area),
                                joint: Some(Joint::Katana),
                                lifetime: Lifetime::frames(2),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()],
                        exit_requirement: ContinuationRequirement::Time(8),
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
                        exit_requirement: ContinuationRequirement::Time(8),
                        ..default()
                    },
                ],
                vec![ActionRequirement::OngoingAction(vec![ActionId::Mizku(
                    criteria,
                )])],
            ),
        )
    })
}

// TODO: Add items
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
                        events: vec![Attack::new(
                            ToHit {
                                hitbox: Hitbox(Area::new(1.0, 1.2, 0.3, 0.3)),
                                velocity: Some(Vec2::new(6.0, -0.4)),
                                lifetime: Lifetime::until_owner_hit(),
                                projectile: Some(Projectile {
                                    model: Model::Kunai,
                                }),
                                ..default()
                            },
                            CommonAttackProps::default(),
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
                            Attack::new(
                                ToHit {
                                    hitbox: Hitbox(Area::new(-0.2, 0.0, 1.2, 0.2)),
                                    joint: Some(Joint::FootR),
                                    lifetime: Lifetime::frames(5),
                                    block_type: Constant(High),
                                    ..default()
                                },
                                CommonAttackProps { ..default() },
                            )
                            .into(),
                            Movement {
                                amount: Vec2::X * 3.0,
                                duration: 10,
                            }
                            .into(),
                        ],
                        exit_requirement: ContinuationRequirement::Time(35),
                        cancel_policy: CancelPolicy::command_normal_recovery(),
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
                cost: 100,
                explanation: "qcf+f to throw, comes in handy\n\nThat's the power...of a president!"
                    .into(),
                category: ItemCategory::Consumable,
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
    ]
    .into_iter()
    .chain(universal_items())
    .collect()
}
