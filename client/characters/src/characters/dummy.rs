use std::iter::empty;

use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionCategory, ActionId, Animation, AnimationType, Area, DummyActionId, DummyAnimation,
    ItemId, Joint, Model, Stats, StatusCondition, StatusFlag, CHARGE_BAR_FULL_SEGMENT_COLOR,
    CHARGE_BAR_PARTIAL_SEGMENT_COLOR, FPS,
};

use crate::{
    actions::ActionRequirement,
    air_action, dashes, ground_action, jumps,
    resources::{RenderInstructions, ResourceType},
    universal_item_actions, Action,
    ActionEvent::{self, *},
    AnimationRequest, Attack,
    AttackHeight::*,
    BlockType::*,
    ChargeProperty, CommonAttackProps, Hitbox, Item,
    ItemCategory::*,
    Lifetime, Movement, Projectile, ResourceBarVisual, Situation, SpecialProperty,
    StunType::*,
    ToHit, WAGResource,
};

use super::{equipment::universal_items, Character};

// Honestly, this character shouldn't really see use, but keep it around for testing
// So it's meant to just be able to compile.
// Could go back on that if that proves to be too much of a hassle
pub fn dummy() -> Character {
    let (jumps, gravity) = jumps!(2.0, 1.0, Animation::Dummy(DummyAnimation::Jump));

    Character::new(
        Model::Dummy,
        HashMap::new(),
        dummy_animations(),
        dummy_moves(jumps),
        dummy_items(),
        Stats {
            gravity,
            ..default()
        },
        vec![(
            ResourceType::Charge,
            WAGResource {
                max: Some(FPS as i32), // Frames to full,
                special: Some(SpecialProperty::Charge(ChargeProperty::default())),
                render_instructions: RenderInstructions::Bar(ResourceBarVisual {
                    default_color: CHARGE_BAR_PARTIAL_SEGMENT_COLOR,
                    full_color: Some(CHARGE_BAR_FULL_SEGMENT_COLOR),
                    ..default()
                }),
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
        (AnimationType::Default, DummyAnimation::TPose),
    ]
    .into_iter()
    .map(|(k, v)| (k, Animation::Dummy(v)))
    .collect()
}

// Dashing

fn dummy_moves(jumps: impl Iterator<Item = (ActionId, Action)>) -> HashMap<ActionId, Action> {
    empty()
        .chain(item_actions())
        .chain(dashes!(
            DummyAnimation::DashForward,
            DummyAnimation::DashBack
        ))
        .chain(jumps)
        .chain(
            normals()
                .chain(specials())
                .map(|(k, v)| (ActionId::Dummy(k), v)),
        )
        .collect()
}

fn normals() -> impl Iterator<Item = (DummyActionId, Action)> {
    let vec = vec![
        (
            DummyActionId::Slap,
            ground_action!(
                "f",
                ActionCategory::Normal,
                DummyAnimation::Slap,
                9,
                Attack::strike(
                    ToHit {
                        hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.35)),
                        joint: Some(Joint::HandR),
                        lifetime: Lifetime::frames(5),
                        ..default()
                    },
                    CommonAttackProps::default(),
                ),
                10
            ),
        ),
        (
            DummyActionId::LowChop,
            ground_action!(
                "[123]f",
                ActionCategory::Normal,
                DummyAnimation::CrouchChop,
                8,
                Attack::strike(
                    ToHit {
                        hitbox: Hitbox(Area::new(0.1, -0.2, 0.3, 0.2)),
                        joint: Some(Joint::HandL),
                        lifetime: Lifetime::frames(5),
                        ..default()
                    },
                    CommonAttackProps::default(),
                ),
                7
            ),
        ),
        (
            DummyActionId::BurnStraight,
            Action {
                input: Some("s"),
                category: ActionCategory::Normal,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![DummyAnimation::BurnStraight.into()];
                    }

                    if situation.elapsed() == 10 {
                        let has_roids = situation.inventory.contains(&ItemId::Roids);

                        return vec![
                            Attack::strike(
                                ToHit {
                                    hitbox: Hitbox(Area::new(-0.3, 0.0, 1.0, 0.2)),
                                    joint: Some(Joint::HandR),
                                    lifetime: Lifetime::frames(8),
                                    ..default()
                                },
                                CommonAttackProps {
                                    damage: 20,
                                    on_hit: Stun(20),
                                    knock_back: if has_roids { 1.0 } else { 2.0 },
                                    push_back: if has_roids { 0.0 } else { 3.0 },
                                    ..default()
                                },
                            )
                            .into(),
                            Movement {
                                amount: Vec2::X * 2.0,
                                duration: 1,
                            }
                            .into(),
                        ];
                    }

                    if situation.elapsed() == 20 {
                        return vec![ActionEvent::End];
                    }

                    vec![]
                },
                requirements: vec![],
            },
        ),
        (
            DummyActionId::AntiAir,
            ground_action!(
                "[123]s",
                ActionCategory::Normal,
                DummyAnimation::AntiAir,
                13,
                Attack::strike(
                    ToHit {
                        hitbox: Hitbox(Area::of_size(0.3, 0.5)),
                        joint: Some(Joint::HandR),
                        lifetime: Lifetime::frames(4),
                        ..default()
                    },
                    CommonAttackProps { ..default() },
                ),
                13
            ),
        ),
        (
            DummyActionId::AirSlap,
            air_action!(
                "f",
                ActionCategory::Normal,
                DummyAnimation::AirSlap,
                8,
                Attack::strike(
                    ToHit {
                        hitbox: Hitbox(Area::new(0.1, 0.0, 0.35, 0.25)),
                        joint: Some(Joint::HandR),
                        lifetime: Lifetime::frames(5),
                        block_type: Strike(High),
                        ..default()
                    },
                    CommonAttackProps::default(),
                ),
                10
            ),
        ),
        (
            DummyActionId::Divekick,
            Action {
                input: Some("s"),
                category: ActionCategory::Normal,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![DummyAnimation::Divekick.into()];
                    }

                    if situation.elapsed() == 5 {
                        return vec![Attack::strike(
                            ToHit {
                                hitbox: Hitbox(Area::of_size(0.35, 0.25)),
                                joint: Some(Joint::FootR),
                                lifetime: Lifetime::frames(10),
                                block_type: Strike(High),
                                ..default()
                            },
                            CommonAttackProps::default(),
                        )
                        .into()];
                    }

                    if situation.elapsed() == 15 {
                        vec![ActionEvent::End];
                    }

                    vec![]
                },
                requirements: vec![
                    ActionRequirement::ItemsOwned(vec![ItemId::Boots]),
                    ActionRequirement::Airborne,
                ],
            },
        ),
        (
            DummyActionId::ForwardThrow,
            ground_action!(
                "w",
                ActionCategory::Normal,
                DummyAnimation::NormalThrow,
                5,
                Attack::strike(
                    ToHit {
                        block_type: Grab,
                        hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                        joint: Some(Joint::HandL),
                        lifetime: Lifetime::frames(5),
                        ..default()
                    },
                    CommonAttackProps {
                        damage: 25,
                        on_hit: Knockdown,
                        ..default()
                    },
                )
                .with_to_target_on_hit(vec![
                    ActionEvent::SnapToOpponent { sideswitch: false },
                    Animation(AnimationRequest {
                        animation: DummyAnimation::NormalThrowRecipient.into(),
                        invert: true,
                        ..default()
                    }),
                ]),
                40
            ),
        ),
        (
            DummyActionId::BackThrow,
            ground_action!(
                "4w",
                ActionCategory::Normal,
                DummyAnimation::NormalThrow,
                5,
                Attack::strike(
                    ToHit {
                        block_type: Grab,
                        hitbox: Hitbox(Area::of_size(0.5, 0.5)),
                        joint: Some(Joint::HandL),
                        lifetime: Lifetime::frames(5),
                        ..default()
                    },
                    CommonAttackProps {
                        damage: 25,
                        on_hit: Knockdown,
                        ..default()
                    },
                )
                .with_to_target_on_hit(vec![
                    ActionEvent::SnapToOpponent { sideswitch: true },
                    Animation(AnimationRequest {
                        animation: DummyAnimation::NormalThrowRecipient.into(),
                        invert: true,
                        ..default()
                    }),
                ]),
                40
            ),
        ),
        (
            DummyActionId::Sweep,
            ground_action!(
                "[123]w",
                ActionCategory::Normal,
                DummyAnimation::Sweep,
                10,
                Attack::strike(
                    ToHit {
                        block_type: Grab,
                        hitbox: Hitbox(Area::new(-0.3, 0.0, 1.0, 0.2)),
                        joint: Some(Joint::HandR),
                        lifetime: Lifetime::frames(5),
                        ..default()
                    },
                    CommonAttackProps {
                        on_hit: Launch(Vec2::new(1.0, 8.0)),
                        ..default()
                    },
                ),
                15
            ),
        ),
        (
            DummyActionId::AirThrow,
            air_action!(
                "w",
                ActionCategory::Throw,
                DummyAnimation::AirThrow,
                9,
                Attack::strike(
                    ToHit {
                        block_type: Grab,
                        hitbox: Hitbox(Area::of_size(0.8, 0.8)),
                        joint: Some(Joint::HandR),
                        lifetime: Lifetime::frames(5),
                        ..default()
                    },
                    CommonAttackProps {
                        damage: 25,
                        on_hit: Launch(Vec2::new(1.0, -4.0)),
                        ..default()
                    },
                )
                .with_to_target_on_hit(vec![
                    SnapToOpponent { sideswitch: false },
                    Animation(AnimationRequest {
                        animation: DummyAnimation::AirThrowRecipient.into(),
                        invert: true,
                        ..default()
                    }),
                ]),
                30
            ),
        ),
    ];
    vec.into_iter()
}

fn specials() -> impl Iterator<Item = (DummyActionId, Action)> {
    vec![
        (
            DummyActionId::Dodge,
            Action {
                input: Some("252"),
                category: ActionCategory::Special,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![
                            ForceStand,
                            DummyAnimation::Dodge.into(),
                            Condition(StatusCondition {
                                flag: StatusFlag::Intangible,
                                effect: None,
                                expiration: Some(20),
                            }),
                        ];
                    }

                    if situation.elapsed() == 45 {
                        return vec![ActionEvent::End];
                    }

                    vec![]
                },
                requirements: vec![],
            },
        ),
        (
            DummyActionId::GroundSlam,
            Action {
                input: Some("[789]6s"),
                category: ActionCategory::Special,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![DummyAnimation::GroundSlam.into()];
                    }

                    if situation.elapsed() == 14 {
                        return vec![
                            Attack::strike(
                                ToHit {
                                    hitbox: Hitbox(Area::new(0.7, 1.25, 0.8, 0.8)),
                                    lifetime: Lifetime::frames(8),
                                    ..default()
                                },
                                CommonAttackProps {
                                    damage: 20,
                                    ..default()
                                },
                            )
                            .into(),
                            Movement {
                                amount: Vec2::X * 2.0,
                                duration: 1,
                            }
                            .into(),
                        ];
                    }

                    if situation.elapsed() == 34 {
                        return vec![ActionEvent::End];
                    }
                    vec![]
                },
                requirements: vec![ActionRequirement::Grounded],
            },
        ),
        (
            DummyActionId::AirSlam,
            Action {
                input: Some("[789]6s"),
                category: ActionCategory::Special,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![DummyAnimation::AirSlam.into()];
                    }

                    if situation.elapsed() == 14 {
                        return vec![
                            Attack::strike(
                                ToHit {
                                    hitbox: Hitbox(Area::new(0.9, 1.25, 0.8, 0.8)),
                                    lifetime: Lifetime::frames(8),
                                    ..default()
                                },
                                CommonAttackProps {
                                    damage: 20,
                                    ..default()
                                },
                            )
                            .into(),
                            Movement {
                                amount: Vec2::X * 1.0,
                                duration: 2,
                            }
                            .into(),
                        ];
                    }

                    if situation.elapsed() == 49 {
                        return vec![ActionEvent::End];
                    }
                    vec![]
                },
                requirements: vec![ActionRequirement::Grounded],
            },
        ),
        (
            DummyActionId::BudgetBoom,
            ground_action!(
                "[41]6f",
                ActionCategory::Special,
                DummyAnimation::TPose,
                10,
                Attack::strike(
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
                ),
                5
            ),
        ),
        (
            DummyActionId::SonicBoom,
            Action {
                input: Some("[41]6f"),
                category: ActionCategory::Special,
                requirements: vec![
                    ActionRequirement::Grounded,
                    ActionRequirement::ResourceFull(ResourceType::Charge),
                ],
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![ForceStand, ClearResource(ResourceType::Charge)];
                    }

                    if situation.elapsed() == 10 {
                        return vec![Attack::strike(
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
                        .into()];
                    }

                    if situation.elapsed() == 15 {
                        return vec![ActionEvent::End];
                    }
                    vec![]
                },
            },
        ),
        (
            DummyActionId::Hadouken,
            ground_action!(
                "236f",
                ActionCategory::Special,
                DummyAnimation::TPose,
                30,
                Attack::strike(
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
                ),
                30
            ),
        ),
        (
            DummyActionId::HeavyHadouken,
            Action {
                input: Some("236s"),
                category: ActionCategory::Special,
                script: |situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![ForceStand, ModifyResource(ResourceType::Meter, -30)];
                    }

                    if situation.elapsed() == 30 {
                        return vec![Attack::strike(
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
                        .into()];
                    }

                    if situation.elapsed() == 50 {
                        return vec![ActionEvent::End];
                    }

                    vec![]
                },
                requirements: vec![ActionRequirement::ResourceValue(ResourceType::Meter, 30)],
            },
        ),
    ]
    .into_iter()
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    empty().chain(universal_item_actions!(Animation::Dummy(
        DummyAnimation::TPose,
    )))
}

fn dummy_items() -> HashMap<ItemId, Item> {
    vec![(
        ItemId::Roids,
        Item {
            cost: 100,
            category: Consumable(crate::items::ConsumableType::OneRound),
            explanation: "Get yoked".into(),
            effect: Stats {
                action_speed_multiplier: 1.1,
                ..Stats::identity()
            },
            ..default()
        },
    )]
    .into_iter()
    .chain(universal_items())
    .collect()
}
