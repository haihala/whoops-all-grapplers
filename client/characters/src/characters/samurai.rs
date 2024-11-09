use std::{f32::consts::PI, sync::Arc};

use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionCategory, ActionId, Animation, AnimationType, Area, CancelType, CancelWindow, Facing,
    GameButton, Icon, ItemId, Model, SamuraiAction, SamuraiAnimation, SoundEffect, SpecialVersion,
    Stats, StatusCondition, StatusFlag, VfxRequest, VisualEffect, VoiceLine,
    SAMURAI_ALT_HELMET_COLOR, SAMURAI_ALT_JEANS_COLOR, SAMURAI_ALT_SHIRT_COLOR,
};

use crate::{
    actions::ActionRequirement,
    build_strike_effect, dashes,
    resources::{RenderInstructions, ResourceType},
    throw_hit, throw_target, Action, ActionEvent, Attack, AttackBuilder,
    AttackHeight::*,
    BlockType::*,
    CharacterBoxes, CharacterStateBoxes, ConsumableType, CounterVisual, FlashRequest, HitInfo,
    Hitbox, Item, ItemCategory, Lifetime, Movement, Situation, ToHit, WAGResource,
};

use super::{
    equipment::{universal_item_actions, universal_items},
    helpers::jumps,
    Character,
};

pub fn samurai() -> Character {
    let (jumps, gravity) = jumps(2.1, 1.1, Animation::Samurai(SamuraiAnimation::Jump));

    Character::new(
        Model::Samurai,
        vec![
            ("T-shirt", SAMURAI_ALT_SHIRT_COLOR),
            ("Jeans", SAMURAI_ALT_JEANS_COLOR),
            ("Samurai Helmet.1", SAMURAI_ALT_HELMET_COLOR),
        ]
        .into_iter()
        .collect(),
        samurai_anims(),
        samurai_moves(jumps),
        samurai_items(),
        samurai_boxes(),
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
        vec![
            (VoiceLine::Defeat, SoundEffect::FemaleNoooo),
            (VoiceLine::BigHit, SoundEffect::FemaleGutPunch),
            (VoiceLine::SmallHit, SoundEffect::FemaleOw),
        ]
        .into_iter()
        .collect(),
    )
}

fn samurai_anims() -> HashMap<AnimationType, Animation> {
    vec![
        (AnimationType::AirIdle, SamuraiAnimation::Air),
        (AnimationType::AirStun, SamuraiAnimation::AirStagger),
        (AnimationType::StandIdle, SamuraiAnimation::Idle),
        (AnimationType::StandBlock, SamuraiAnimation::Block),
        (AnimationType::StandStun, SamuraiAnimation::Stagger),
        (AnimationType::WalkBack, SamuraiAnimation::WalkBack),
        (AnimationType::WalkForward, SamuraiAnimation::WalkForward),
        (AnimationType::CrouchIdle, SamuraiAnimation::Crouch),
        (AnimationType::CrouchBlock, SamuraiAnimation::CrouchBlock),
        (AnimationType::CrouchStun, SamuraiAnimation::CrouchStagger),
        (AnimationType::Getup, SamuraiAnimation::Getup),
        (AnimationType::Default, SamuraiAnimation::StandPose),
    ]
    .into_iter()
    .map(|(k, v)| (k, Animation::from(v)))
    .collect()
}

fn samurai_moves(jumps: impl Iterator<Item = (ActionId, Action)>) -> HashMap<ActionId, Action> {
    jumps
        .chain(dashes!(
            SamuraiAnimation::DashForward,
            SamuraiAnimation::DashBack
        ))
        .chain(item_actions())
        .chain(
            normals()
                .chain(specials())
                .map(|(k, v)| (ActionId::Samurai(k), v)),
        )
        .collect()
}

fn normals() -> impl Iterator<Item = (SamuraiAction, Action)> {
    vec![
        (
            SamuraiAction::KneeThrust,
            AttackBuilder::normal("f")
                .with_animation(SamuraiAnimation::KneeThrust)
                .with_frame_data(5, 2, 16)
                .with_hitbox(Area::new(0.5, 1.0, 0.35, 0.35))
                .with_damage(5)
                .with_advantage_on_block(-1)
                .with_advantage_on_hit(4)
                .build(),
        ),
        (
            SamuraiAction::LowKick,
            AttackBuilder::normal("f|123")
                .hits_low()
                .with_animation(SamuraiAnimation::LowKick)
                .with_frame_data(3, 3, 12)
                .with_hitbox(Area::new(0.4, 0.1, 0.9, 0.2))
                .with_damage(8)
                .with_advantage_on_block(-1)
                .with_advantage_on_hit(6)
                .build(),
        ),
        (
            SamuraiAction::HeelKick,
            AttackBuilder::normal("s")
                .with_animation(SamuraiAnimation::HeelKick)
                .with_frame_data(9, 6, 28)
                .with_hitbox(Area::new(1.2, 1.0, 1.2, 0.2))
                .with_damage(15)
                .with_advantage_on_block(-8)
                .with_advantage_on_hit(3)
                .with_extra_initial_events(vec![Movement {
                    amount: Vec2::X * 10.0,
                    duration: 20,
                }
                .into()])
                .with_extra_activation_events(vec![Movement {
                    amount: Vec2::X * 3.0,
                    duration: 10,
                }
                .into()])
                .build(),
        ),
        (
            SamuraiAction::Uppercut,
            AttackBuilder::normal("s|123")
                .with_animation(SamuraiAnimation::Uppercut)
                .with_frame_data(8, 8, 40)
                .with_hitbox(Area::new(0.3, 0.7, 0.3, 0.5))
                .with_damage(16)
                .with_distance_on_block(0.5)
                .launches(Vec2::new(1.0, 6.0))
                .with_advantage_on_block(-30)
                .build(),
        ),
        (
            SamuraiAction::HighStab,
            AttackBuilder::normal("g")
                .with_animation(SamuraiAnimation::HighStab)
                .with_frame_data(7, 6, 46)
                .with_hitbox(Area::new(1.5, 1.3, 1.8, 0.2))
                .with_damage(10)
                .sword()
                .with_advantage_on_block(-16)
                .with_advantage_on_hit(-6)
                .build(),
        ),
        (
            SamuraiAction::SkySlash,
            AttackBuilder::normal("g|123")
                .with_animation(SamuraiAnimation::SkyStab)
                .with_frame_data(8, 5, 32)
                .with_hitbox(Area::new(1.8, 0.9, 1.0, 1.0))
                .with_damage(8)
                .sword()
                .with_advantage_on_block(-7)
                .with_advantage_on_hit(10)
                .build(),
        ),
        (
            SamuraiAction::AirSlice,
            AttackBuilder::normal("g")
                .air_only()
                .with_animation(SamuraiAnimation::AirStab)
                .with_frame_data(7, 12, 63)
                .with_hitbox(Area::new(0.0, -0.5, 1.0, 0.4))
                .with_damage(10)
                .sword()
                // TODO: These are misleading due to landing cancels
                .with_advantage_on_block(-30)
                .with_advantage_on_hit(-20)
                .build(),
        ),
        (
            SamuraiAction::FalconKnee,
            AttackBuilder::normal("f")
                .air_only()
                .with_animation(SamuraiAnimation::FalconKnee)
                .with_frame_data(2, 5, 23)
                .with_hitbox(Area::new(0.3, 0.2, 0.35, 0.25))
                .with_damage(5)
                // TODO: These are misleading due to landing cancels
                .with_advantage_on_block(-20)
                .with_advantage_on_hit(-10)
                .build(),
        ),
        (
            SamuraiAction::FootDive,
            Action {
                input: Some("s"),
                category: ActionCategory::Normal,
                script: Box::new(|situation: &Situation| {
                    if situation.elapsed() == 0 {
                        return vec![
                            SamuraiAnimation::FootDiveHold.into(),
                            Movement {
                                amount: Vec2::Y * -1.0,
                                duration: 7,
                            }
                            .into(),
                        ];
                    }

                    // Fallback (also for tests) of ending after a minute
                    if situation.elapsed() > 60 * 60 {
                        return vec![ActionEvent::End];
                    }

                    // TODO: Add an item to speed this up for instant overheads
                    // FIXME: This will likely spawn a hitbox every frame
                    if situation.elapsed() >= 20
                        && !situation.held_buttons.contains(&GameButton::Strong)
                    {
                        return vec![
                            SamuraiAnimation::FootDiveRelease.into(),
                            // TODO: There used to be a 3f delay after the animation, but new
                            // system makes that hard, maybe think of a way to reintroduce that.
                            ActionEvent::SpawnHitbox(Attack {
                                to_hit: ToHit {
                                    hitbox: Hitbox(Area::new(0.8, -0.2, 0.7, 0.3)),
                                    lifetime: Lifetime::frames(7),
                                    block_type: Strike(High),
                                    ..default()
                                },
                                on_hit: Arc::new(|situation: &Situation, hit_data: &HitInfo| {
                                    build_strike_effect(
                                        25,
                                        High,
                                        0.33,
                                        0.66,
                                        1,
                                        if situation.inventory.contains(&ItemId::SpaceSuitBoots) {
                                            ActionEvent::LaunchStun(Vec2::new(-1.0, 15.0))
                                        } else {
                                            ActionEvent::HitStun(40)
                                        },
                                        0.2,
                                        18,
                                        0,
                                    )(situation, hit_data)
                                }),
                            }),
                        ];
                    }

                    vec![]
                }),
                requirement: ActionRequirement::Airborne,
            },
        ),
        (
            SamuraiAction::ForwardThrow,
            AttackBuilder::normal("w")
                .forward_throw()
                .throw_hit_action(SamuraiAction::StandThrowHit)
                .throw_target_action(SamuraiAction::StandThrowTarget)
                .with_frame_data(3, 3, 34)
                .with_animation(SamuraiAnimation::StandThrowStartup)
                .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5))
                .build(),
        ),
        (
            SamuraiAction::BackThrow,
            AttackBuilder::normal("4+w")
                .back_throw()
                .throw_hit_action(SamuraiAction::StandThrowHit)
                .throw_target_action(SamuraiAction::StandThrowTarget)
                .with_frame_data(3, 3, 34)
                .with_animation(SamuraiAnimation::StandThrowStartup)
                .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5))
                .build(),
        ),
        (
            SamuraiAction::StandThrowHit,
            throw_hit!(SamuraiAnimation::StandThrowHit, 80),
        ),
        (
            SamuraiAction::StandThrowTarget,
            throw_target!(
                SamuraiAnimation::StandThrowTarget,
                30,
                10,
                Vec2::new(-2.0, 6.0)
            ),
        ),
        (
            SamuraiAction::CrouchThrow,
            AttackBuilder::normal("w|123")
                .forward_throw()
                .throw_hit_action(SamuraiAction::CrouchThrowHit)
                .throw_target_action(SamuraiAction::CrouchThrowTarget)
                .with_frame_data(5, 3, 55)
                .with_animation(SamuraiAnimation::CrouchThrowStartup)
                .with_hitbox(Area::new(0.7, 0.1, 0.5, 0.2))
                .build(),
        ),
        (
            SamuraiAction::CrouchThrowHit,
            throw_hit!(SamuraiAnimation::CrouchThrowHit, 80),
        ),
        (
            SamuraiAction::CrouchThrowTarget,
            throw_target!(
                SamuraiAnimation::CrouchThrowTarget,
                34,
                10,
                Vec2::new(-5.0, 2.0)
            ),
        ),
        (
            SamuraiAction::AirThrow,
            AttackBuilder::normal("w")
                .forward_throw()
                .air_only()
                .throw_hit_action(SamuraiAction::AirThrowHit)
                .throw_target_action(SamuraiAction::AirThrowTarget)
                .with_frame_data(4, 2, 36)
                .with_animation(SamuraiAnimation::AirThrowStartup)
                .with_hitbox(Area::new(0.4, 0.5, 0.8, 0.8))
                .build(),
        ),
        (
            SamuraiAction::AirThrowHit,
            throw_hit!(SamuraiAnimation::AirThrowHit, 50),
        ),
        (
            SamuraiAction::AirThrowTarget,
            throw_target!(
                SamuraiAnimation::AirThrowTarget,
                30,
                50,
                10,
                Vec2::new(-2.0, 2.0)
            ),
        ),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (SamuraiAction, Action)> {
    sword_stance().chain(kunai_throws())
}

fn sword_stance() -> impl Iterator<Item = (SamuraiAction, Action)> {
    vec![
        SpecialVersion::Fast,
        SpecialVersion::Strong,
        SpecialVersion::Metered,
    ]
    .into_iter()
    .flat_map(|version| {
        vec![
            (
                SamuraiAction::SwordStance(version),
                enter_sword_stance(version),
            ),
            (
                SamuraiAction::StanceCancel(SpecialVersion::Fast),
                exit_sword_stance(SpecialVersion::Fast),
            ),
            (
                SamuraiAction::ViperStrike(SpecialVersion::Fast),
                viper_strike(SpecialVersion::Fast),
            ),
            (
                SamuraiAction::RisingSun(SpecialVersion::Fast),
                rising_sun(SpecialVersion::Fast),
            ),
        ]
    })
    .chain(vec![(SamuraiAction::Sharpen, sharpen())])
}

fn enter_sword_stance(version: SpecialVersion) -> Action {
    Action {
        input: Some(match version {
            SpecialVersion::Strong => "214+s",
            SpecialVersion::Fast => "214+f",
            SpecialVersion::Metered => "214+(fs)",
        }),
        category: ActionCategory::Special,
        script: Box::new(move |situation: &Situation| {
            let mut events = vec![SamuraiAnimation::SwordStanceEnter.into()];

            if version == SpecialVersion::Metered {
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
                return vec![ActionEvent::AllowCancel(CancelWindow {
                    cancel_type: CancelType::Specific(
                        vec![
                            SamuraiAction::Sharpen,
                            SamuraiAction::ViperStrike(version),
                            SamuraiAction::RisingSun(version),
                            SamuraiAction::StanceCancel(version),
                        ]
                        .into_iter()
                        .map(ActionId::Samurai)
                        .collect(),
                    ),
                    duration: 30,
                    require_hit: false,
                })];
            }

            situation.end_at(40)
        }),
        requirement: ActionRequirement::And(vec![
            ActionRequirement::Grounded,
            ActionRequirement::Starter(ActionCategory::Special),
            if version == SpecialVersion::Metered {
                ActionRequirement::ResourceValue(ResourceType::Meter, 20)
            } else {
                ActionRequirement::default()
            },
        ]),
    }
}

fn exit_sword_stance(version: SpecialVersion) -> Action {
    Action {
        input: Some(match version {
            SpecialVersion::Strong => "5+S",
            SpecialVersion::Fast => "5+F",
            SpecialVersion::Metered => "5+(FS)",
        }),
        category: ActionCategory::Special,
        script: Box::new(|situation: &Situation| {
            if situation.elapsed() == 0 {
                return vec![SamuraiAnimation::SwordStanceExit.into()];
            }

            situation.end_at(9)
        }),
        requirement: ActionRequirement::And(vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![ActionId::Samurai(SamuraiAction::SwordStance(
                version,
            ))]),
        ]),
    }
}

fn sharpen() -> Action {
    Action {
        input: Some("g"),
        category: ActionCategory::Special,
        script: Box::new(|situation: &Situation| {
            if situation.elapsed() == 0 {
                return vec![
                    SamuraiAnimation::Sharpen.into(),
                    ActionEvent::Sound(SoundEffect::KnifeChopstickDrag),
                ];
            }

            if situation.elapsed() == 50 {
                return vec![
                    ActionEvent::ModifyResource(ResourceType::Sharpness, 1),
                    ActionEvent::Sound(SoundEffect::HangingKnifeFlick),
                ];
            }

            situation.end_at(55)
        }),
        requirement: ActionRequirement::And(vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![
                ActionId::Samurai(SamuraiAction::SwordStance(SpecialVersion::Fast)),
                ActionId::Samurai(SamuraiAction::SwordStance(SpecialVersion::Strong)),
                ActionId::Samurai(SamuraiAction::SwordStance(SpecialVersion::Metered)),
            ]),
        ]),
    }
}

fn viper_strike(version: SpecialVersion) -> Action {
    AttackBuilder::special(match version {
        SpecialVersion::Strong => "S|123",
        SpecialVersion::Fast => "F|123",
        SpecialVersion::Metered => "(FS)|123",
    })
    .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
    .with_sound(SoundEffect::FemaleLoYah)
    .with_frame_data(10, 2, 50)
    .with_animation(SamuraiAnimation::SwordStanceLowSlash)
    .with_extra_initial_events(vec![Movement {
        amount: Vec2::X * 8.0,
        duration: 7,
    }
    .into()])
    .with_hitbox(Area::new(1.0, 0.225, 1.3, 0.45))
    .hits_low()
    .with_damage(30)
    .sword()
    .with_advantage_on_hit(-10)
    .with_advantage_on_block(-40)
    .with_dynamic_activation_events(|situation: &Situation| {
        vec![ActionEvent::RelativeVisualEffect(VfxRequest {
            effect: VisualEffect::WaveFlat,
            tf: Transform {
                translation: situation.facing.to_vec3() * 1.0 + Vec3::Y * 0.4,
                rotation: match situation.facing {
                    Facing::Left => Quat::from_euler(EulerRot::ZYX, PI, 0.0, -PI / 3.0),
                    Facing::Right => Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, PI / 3.0),
                },
                scale: Vec3::splat(4.0),
            },
            ..default()
        })]
    })
    .build()
}

fn rising_sun(version: SpecialVersion) -> Action {
    AttackBuilder::special(match version {
        SpecialVersion::Strong => "S",
        SpecialVersion::Fast => "F",
        SpecialVersion::Metered => "(FS)",
    })
    .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
    .with_sound(SoundEffect::FemaleHiYah)
    .with_frame_data(10, 3, 50)
    .with_animation(SamuraiAnimation::SwordStanceHighSlash)
    .sword()
    .with_damage(20)
    .launches(Vec2::new(1.0, 3.0))
    .with_advantage_on_block(-30)
    .with_hitbox(Area::new(0.25, 1.5, 2.0, 1.5))
    .with_dynamic_activation_events(|situation: &Situation| {
        vec![ActionEvent::RelativeVisualEffect(VfxRequest {
            effect: VisualEffect::WaveDiagonal,
            tf: Transform {
                translation: situation.facing.to_vec3() + Vec3::Y * 1.7,
                rotation: match situation.facing {
                    Facing::Left => Quat::from_rotation_z(PI * 7.0 / 6.0),
                    Facing::Right => Quat::from_rotation_z(PI / 3.0),
                },
                scale: Vec3::splat(2.0),
            },
            ..default()
        })]
    })
    .build()
}

fn kunai_throws() -> impl Iterator<Item = (SamuraiAction, Action)> {
    vec![
        SpecialVersion::Fast,
        SpecialVersion::Strong,
        SpecialVersion::Metered,
    ]
    .into_iter()
    .flat_map(|version| {
        vec![(
            SamuraiAction::KunaiThrow(version),
            AttackBuilder::special(match version {
                SpecialVersion::Fast => "236+f",
                SpecialVersion::Strong => "236+s",
                SpecialVersion::Metered => "236+(fs)",
            })
            .with_animation(SamuraiAnimation::KunaiThrow)
            .with_sound(SoundEffect::FemaleKyatchi)
            .with_extra_initial_events(vec![
                ActionEvent::ForceStand,
                ActionEvent::ModifyResource(ResourceType::KunaiCounter, -1),
            ])
            .with_extra_requirements(vec![ActionRequirement::ResourceValue(
                ResourceType::KunaiCounter,
                1,
            )])
            .with_timings(11, 10)
            .with_hitbox(Area::new(0.2, 1.2, 0.3, 0.3))
            .with_spawn(Model::Kunai)
            .with_hitbox_velocity(match version {
                SpecialVersion::Fast => Vec2::new(4.0, 1.0),
                SpecialVersion::Strong => Vec2::new(0.2, 4.0),
                SpecialVersion::Metered => Vec2::new(10.0, 1.0),
            })
            .with_hitbox_gravity(4.0)
            .with_blockstun(15)
            .with_hitstun(20)
            .build(),
        )]
    })
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    universal_item_actions(Animation::Samurai(SamuraiAnimation::GiParry))
}

fn samurai_items() -> HashMap<ItemId, Item> {
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

fn samurai_boxes() -> CharacterBoxes {
    CharacterBoxes {
        standing: CharacterStateBoxes {
            head: Area::new(-0.05, 1.8, 0.4, 0.3),
            chest: Area::new(0.0, 1.3, 0.6, 0.8),
            legs: Area::new(0.05, 0.6, 0.65, 1.2),
            pushbox: Area::from_center_size(Vec2::Y * 0.7, Vec2::new(0.4, 1.4)),
        },
        crouching: CharacterStateBoxes {
            head: Area::new(0.2, 0.6, 0.4, 0.3),
            chest: Area::new(0.1, 0.45, 0.6, 0.3),
            legs: Area::new(0.0, 0.2, 1.0, 0.4),
            pushbox: Area::from_center_size(Vec2::new(0.1, 0.35), Vec2::new(0.6, 0.7)),
        },
        airborne: CharacterStateBoxes {
            head: Area::new(0.15, 1.25, 0.4, 0.3),
            chest: Area::new(0.1, 0.9, 1.1, 0.6),
            legs: Area::new(-0.2, 0.4, 0.9, 0.8),
            pushbox: Area::from_center_size(Vec2::new(0.0, 0.55), Vec2::new(0.4, 0.6)),
        },
    }
}
