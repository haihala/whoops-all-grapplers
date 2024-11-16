use std::f32::consts::PI;

use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionCategory, ActionId, Animation, AnimationType, Area, CancelType, CancelWindow, Facing,
    GameButton, Icon, ItemId, Model, SamuraiAction, SamuraiAnimation, SoundEffect, SpecialVersion,
    Stats, StatusCondition, StatusFlag, VfxRequest, VisualEffect, VoiceLine, FAST_SWORD_VFX,
    METERED_SWORD_VFX, SAMURAI_ALT_HELMET_COLOR, SAMURAI_ALT_JEANS_COLOR, SAMURAI_ALT_SHIRT_COLOR,
    STRONG_SWORD_VFX,
};

use crate::{
    actions::ActionRequirement,
    dashes,
    resources::{RenderInstructions, ResourceType},
    Action, ActionEvent, Attack, AttackBuilder,
    AttackHeight::*,
    BlockType::*,
    CharacterBoxes, CharacterStateBoxes, ConsumableType, CounterVisual, FlashRequest, Hitbox, Item,
    ItemCategory, Lifetime, Movement, Situation, StrikeEffectBuilder, ThrowEffectBuilder, ToHit,
    WAGResource,
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
            kunais: 2,
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
                .chain(throws())
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
                .with_hitbox(Area::new(0.5, 1.2, 0.35, 0.35))
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
                .with_hitbox(Area::new(0.7, 0.1, 0.9, 0.2))
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
                .with_hitbox(Area::new(0.7, 1.0, 1.0, 0.2))
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
            Action {
                input: Some("s|123"),
                script: Box::new(|situation: &Situation| {
                    if situation.on_frame(0) {
                        return vec![SamuraiAnimation::Uppercut.into()];
                    }

                    if situation.on_frame(8) {
                        let hitbox = Area::new(0.3, 0.7, 0.3, 0.5);
                        return vec![
                            ActionEvent::ExpandHurtbox(hitbox.grow(0.1), 8),
                            ActionEvent::AllowCancel(CancelWindow {
                                require_hit: true,
                                duration: 30,
                                cancel_type: CancelType::Special,
                            }),
                            ActionEvent::SpawnHitbox(Attack {
                                to_hit: ToHit {
                                    hitbox: Hitbox(hitbox),
                                    lifetime: Lifetime::frames(4),
                                    ..default()
                                },
                                on_hit: StrikeEffectBuilder::new(
                                    40,
                                    Mid,
                                    ActionEvent::LaunchStun(Vec2::new(-1.0, 6.0)),
                                    9,
                                )
                                .with_distance_on_block(0.5)
                                .with_pushback_on_hit(0.9) // Inaccurate due to launch
                                .with_extra_on_hit_events(
                                    if situation.inventory.contains(&ItemId::IceCube) {
                                        vec![
                                            ActionEvent::ClearMovement,
                                            ActionEvent::RelativeVisualEffect(VfxRequest {
                                                effect: VisualEffect::Icon(Icon::IceCube),
                                                tf: Transform::from_translation(Vec3::Y * 1.0),
                                                ..default()
                                            }),
                                        ]
                                    } else {
                                        vec![]
                                    },
                                )
                                .build(),
                            }),
                        ];
                    }

                    if situation.on_frame(12) {
                        let hitbox = Area::new(0.35, 1.45, 0.3, 1.2);
                        return vec![
                            ActionEvent::ExpandHurtbox(hitbox.grow(0.1), 8),
                            ActionEvent::SpawnHitbox(Attack {
                                to_hit: ToHit {
                                    hitbox: Hitbox(hitbox),
                                    lifetime: Lifetime::frames(4),
                                    ..default()
                                },

                                on_hit: StrikeEffectBuilder::new(
                                    30,
                                    Mid,
                                    ActionEvent::HitStun(38),
                                    6,
                                )
                                .with_distance_on_block(0.5)
                                .with_pushback_on_hit(0.1) // Inaccurate due to launch
                                .build(),
                            }),
                        ];
                    }

                    situation.end_at(48)
                }),
                requirement: ActionRequirement::And(vec![
                    ActionRequirement::Grounded,
                    ActionRequirement::Starter(ActionCategory::Normal),
                ]),
            },
        ),
        (
            SamuraiAction::HighStab,
            AttackBuilder::normal("g")
                .with_animation(SamuraiAnimation::HighStab)
                .with_frame_data(7, 6, 46)
                .with_hitbox(Area::new(1.0, 1.2, 1.8, 0.2))
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
                .with_hitbox(Area::new(1.0, 2.0, 1.0, 1.0))
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
                .with_hitbox(Area::new(0.0, 0.0, 1.0, 0.4))
                .with_damage(10)
                .sword()
                .with_blockstun(20)
                .with_hitstun(30)
                .build(),
        ),
        (
            SamuraiAction::FalconKnee,
            AttackBuilder::normal("f")
                .air_only()
                .with_animation(SamuraiAnimation::FalconKnee)
                .with_frame_data(2, 5, 23)
                .with_hitbox(Area::new(0.3, 0.4, 0.35, 0.25))
                .with_damage(5)
                .with_blockstun(10)
                .with_hitstun(15)
                .build(),
        ),
        (
            SamuraiAction::FootDiveHold,
            Action {
                input: Some("s"),
                script: Box::new(|situation: &Situation| {
                    if situation.on_frame(0) {
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
                    if situation.on_frame(60 * 60) {
                        return vec![ActionEvent::End];
                    }

                    if situation.after_frame(30)
                        && !situation.held_buttons.contains(&GameButton::Strong)
                    {
                        return vec![ActionEvent::StartAction(
                            SamuraiAction::FootDiveRelease.into(),
                        )];
                    }

                    vec![]
                }),
                requirement: ActionRequirement::And(vec![
                    ActionRequirement::Airborne,
                    ActionRequirement::Starter(ActionCategory::Normal),
                ]),
            },
        ),
        (
            SamuraiAction::FootDiveRelease,
            Action {
                input: None,
                requirement: ActionRequirement::default(),
                script: Box::new(|situation: &Situation| {
                    if situation.on_frame(0) {
                        return vec![SamuraiAnimation::FootDiveRelease.into()];
                    }

                    if situation.on_frame(3) {
                        return vec![ActionEvent::SpawnHitbox(Attack {
                            to_hit: ToHit {
                                hitbox: Hitbox(Area::new(0.8, -0.2, 0.7, 0.3)),
                                lifetime: Lifetime::frames(7),
                                block_type: Strike(High),
                                ..default()
                            },
                            on_hit: StrikeEffectBuilder::new(
                                25,
                                High,
                                ActionEvent::HitStun(40),
                                18,
                            )
                            .with_distance_on_block(1.0)
                            .with_pushback_on_hit(0.3)
                            .build(),
                        })];
                    }

                    situation.end_at(20)
                }),
            },
        ),
    ]
    .into_iter()
}

fn throws() -> impl Iterator<Item = (SamuraiAction, Action)> {
    let (stand_throw_target, stand_throw_activation) = ThrowEffectBuilder::new(
        SamuraiAnimation::StandThrowHit,
        80,
        SamuraiAnimation::StandThrowTarget,
        30,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(2.0, 6.0))
    .build();

    let (crouch_throw_target, crouch_throw_activation) = ThrowEffectBuilder::new(
        SamuraiAnimation::CrouchThrowHit,
        80,
        SamuraiAnimation::CrouchThrowTarget,
        30,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(-5.0, 2.0))
    .with_extra_target_events(vec![ActionEvent::Teleport(Vec2::new(2.0, 1.0))])
    .build();

    let (air_throw_target, air_throw_activation) = ThrowEffectBuilder::new(
        SamuraiAnimation::AirThrowHit,
        50,
        SamuraiAnimation::AirThrowTarget,
        50,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(2.0, 2.0))
    .build();

    vec![
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
        (SamuraiAction::StandThrowHit, stand_throw_activation),
        (SamuraiAction::StandThrowTarget, stand_throw_target),
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
        (SamuraiAction::CrouchThrowHit, crouch_throw_activation),
        (SamuraiAction::CrouchThrowTarget, crouch_throw_target),
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
        (SamuraiAction::AirThrowHit, air_throw_activation),
        (SamuraiAction::AirThrowTarget, air_throw_target),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (SamuraiAction, Action)> {
    stance_moves().chain(kunai_throws())
}

fn stance_moves() -> impl Iterator<Item = (SamuraiAction, Action)> {
    vec![
        SpecialVersion::Fast,
        SpecialVersion::Strong,
        SpecialVersion::Metered,
    ]
    .into_iter()
    .flat_map(|version| {
        vec![
            // Base kit
            (SamuraiAction::SwordStance(version), sword_stance(version)),
            (SamuraiAction::StanceCancel(version), stance_cancel(version)),
            (SamuraiAction::ViperStrike(version), viper_strike(version)),
            (SamuraiAction::RisingSun(version), rising_sun(version)),
            (SamuraiAction::Sharpen(version), sharpen(version)),
            // Require items
            (SamuraiAction::SwordSlam(version), sword_slam(version)),
            (
                SamuraiAction::StanceForwardDash(version),
                stance_dash(version, false),
            ),
            (
                SamuraiAction::StanceBackDash(version),
                stance_dash(version, true),
            ),
        ]
    })
}

fn sword_stance(version: SpecialVersion) -> Action {
    Action {
        input: Some(match version {
            SpecialVersion::Strong => "214s",
            SpecialVersion::Fast => "214f",
            SpecialVersion::Metered => "214(fs)",
        }),
        script: Box::new(move |situation: &Situation| {
            let mut events = vec![SamuraiAnimation::SwordStance.into()];

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

            if situation.on_frame(0) {
                return events;
            }

            if situation.on_frame(3) {
                return vec![ActionEvent::AllowCancel(CancelWindow {
                    cancel_type: CancelType::Specific(
                        vec![
                            SamuraiAction::SwordSlam(version),
                            SamuraiAction::StanceForwardDash(version),
                            SamuraiAction::StanceBackDash(version),
                            SamuraiAction::Sharpen(version),
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

fn stance_cancel(version: SpecialVersion) -> Action {
    Action {
        input: Some(match version {
            SpecialVersion::Strong => "S|5",
            SpecialVersion::Fast => "F|5",
            SpecialVersion::Metered => "(FS)|5",
        }),
        script: Box::new(|situation: &Situation| {
            if situation.on_frame(0) {
                return vec![
                    SamuraiAnimation::StanceCancel.into(),
                    ActionEvent::ClearCondition(StatusFlag::Intangible),
                ];
            }

            situation.end_at(9)
        }),
        requirement: ActionRequirement::And(vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![ActionId::Samurai(SamuraiAction::SwordStance(
                version,
            ))]),
            ActionRequirement::Starter(ActionCategory::Jump),
        ]),
    }
}

fn stance_dash(version: SpecialVersion, back: bool) -> Action {
    Action {
        input: Some(if back { "454" } else { "656" }),
        script: Box::new(move |situation: &Situation| {
            if situation.on_frame(0) {
                return vec![
                    ActionEvent::Teleport(Vec2::X * if back { -2.0 } else { 2.0 }),
                    ActionEvent::RelativeVisualEffect(VfxRequest {
                        effect: VisualEffect::SmokeBomb,
                        tf: Transform::from_translation(Vec3::Y * 1.5),
                        ..default()
                    }),
                ];
            }

            if situation.after_frame(10) {
                return vec![ActionEvent::StartAction(ActionId::Samurai(
                    SamuraiAction::SwordStance(version),
                ))];
            }

            vec![]
        }),
        requirement: ActionRequirement::And(vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![ActionId::Samurai(SamuraiAction::SwordStance(
                version,
            ))]),
            ActionRequirement::ItemOwned(ItemId::SmokeBomb),
            ActionRequirement::Starter(ActionCategory::Special),
        ]),
    }
}
fn sharpen(version: SpecialVersion) -> Action {
    let (slow, sharpness_gain, meter_gain) = match version {
        SpecialVersion::Metered => (false, 2, 0),
        SpecialVersion::Strong => (true, 1, 30),
        SpecialVersion::Fast => (false, 1, 20),
    };

    Action {
        input: Some("g"),
        script: Box::new(move |situation: &Situation| {
            if situation.on_frame(0) {
                return vec![
                    if slow {
                        SamuraiAnimation::SlowSharpen
                    } else {
                        SamuraiAnimation::FastSharpen
                    }
                    .into(),
                    ActionEvent::Sound(SoundEffect::KnifeChopstickDrag),
                ];
            }

            if situation.on_frame(if slow { 50 } else { 35 }) {
                return vec![
                    ActionEvent::ModifyResource(ResourceType::Sharpness, sharpness_gain),
                    ActionEvent::ModifyResource(ResourceType::Meter, meter_gain),
                    ActionEvent::Sound(SoundEffect::HangingKnifeFlick),
                ];
            }

            situation.end_at(if slow { 60 } else { 45 })
        }),
        requirement: ActionRequirement::And(vec![
            ActionRequirement::Grounded,
            ActionRequirement::ActionOngoing(vec![ActionId::Samurai(SamuraiAction::SwordStance(
                version,
            ))]),
            ActionRequirement::Starter(ActionCategory::Special),
        ]),
    }
}

fn sword_slam(version: SpecialVersion) -> Action {
    let (input, slow, high_damage, color) = match version {
        SpecialVersion::Strong => ("S|69", true, true, STRONG_SWORD_VFX),
        SpecialVersion::Fast => ("F|69", false, false, FAST_SWORD_VFX),
        SpecialVersion::Metered => ("(FS)|69", false, true, METERED_SWORD_VFX),
    };

    let mut builder = AttackBuilder::special(input)
        .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
        .with_extra_requirements(vec![ActionRequirement::ItemOwned(ItemId::Fireaxe)])
        .with_sound(SoundEffect::FemaleKiritsu)
        .with_frame_data(if slow { 25 } else { 20 }, 2, 60)
        .with_animation(if slow {
            SamuraiAnimation::SlowSwordSlam
        } else {
            SamuraiAnimation::FastSwordSlam
        })
        .with_hitbox(Area::new(0.5, 1.0, 2.0, 1.0))
        .hits_overhead()
        .with_damage(if high_damage { 30 } else { 15 })
        .sword()
        .with_advantage_on_block(if slow { -40 } else { -30 })
        .with_dynamic_activation_events(move |situation: &Situation| {
            vec![ActionEvent::RelativeVisualEffect(VfxRequest {
                effect: VisualEffect::WaveFlat(color),
                tf: Transform {
                    translation: situation.facing.to_vec3() + Vec3::Y * 0.5,
                    rotation: match situation.facing {
                        Facing::Right => Quat::IDENTITY,
                        Facing::Left => Quat::from_rotation_z(PI),
                    },
                    scale: Vec3::splat(4.0),
                },
                ..default()
            })]
        });

    builder = match version {
        SpecialVersion::Metered | SpecialVersion::Strong => builder.launches(Vec2::new(1.0, 6.0)),
        SpecialVersion::Fast => builder.with_advantage_on_hit(6),
    };

    builder.build()
}

fn viper_strike(version: SpecialVersion) -> Action {
    let (input, long_lunge, slow, high_damage, color) = match version {
        SpecialVersion::Strong => ("S|123", true, true, true, STRONG_SWORD_VFX),
        SpecialVersion::Fast => ("F|123", false, false, false, FAST_SWORD_VFX),
        SpecialVersion::Metered => ("(FS)|123", false, false, true, METERED_SWORD_VFX),
    };

    AttackBuilder::special(input)
        .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
        .with_sound(SoundEffect::FemaleShagamu)
        .with_frame_data(if slow { 10 } else { 5 }, 2, if slow { 50 } else { 45 })
        .with_animation(if slow {
            SamuraiAnimation::SlowViperStrike
        } else {
            SamuraiAnimation::FastViperStrike
        })
        .with_extra_initial_events(vec![Movement {
            amount: Vec2::X * if long_lunge { 12.0 } else { 8.0 },
            duration: 7,
        }
        .into()])
        .with_hitbox(Area::new(1.0, 0.225, 1.3, 0.45))
        .hits_low()
        .with_damage(if high_damage { 30 } else { 15 })
        .sword()
        .with_advantage_on_hit(if slow { 1 } else { 3 })
        .with_advantage_on_block(if slow { -40 } else { -30 })
        .with_dynamic_activation_events(move |situation: &Situation| {
            vec![ActionEvent::RelativeVisualEffect(VfxRequest {
                effect: VisualEffect::WaveFlat(color),
                tf: Transform {
                    translation: situation.facing.to_vec3() * if long_lunge { 1.5 } else { 1.0 }
                        + Vec3::Y * 0.4,
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
    let (input, slow, high_bounce, high_damage, color) = match version {
        SpecialVersion::Strong => ("S", true, true, true, STRONG_SWORD_VFX),
        SpecialVersion::Fast => ("F", false, false, false, FAST_SWORD_VFX),
        SpecialVersion::Metered => ("(FS)", false, false, true, METERED_SWORD_VFX),
    };

    AttackBuilder::special(input)
        .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
        .with_sound(SoundEffect::FemaleHiYah)
        .with_frame_data(if slow { 14 } else { 4 }, 3, if slow { 56 } else { 44 })
        .with_animation(if slow {
            SamuraiAnimation::SlowRisingSun
        } else {
            SamuraiAnimation::FastRisingSun
        })
        .sword()
        .with_damage(if high_damage { 20 } else { 15 })
        .launches(if high_bounce {
            Vec2::new(0.1, 10.0)
        } else {
            Vec2::new(1.0, 3.0)
        })
        .with_advantage_on_block(-30)
        .with_hitbox(Area::new(0.25, 1.5, 2.0, 1.5))
        .with_dynamic_activation_events(move |situation: &Situation| {
            vec![ActionEvent::RelativeVisualEffect(VfxRequest {
                effect: VisualEffect::WaveDiagonal(color),
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
    .map(|version| {
        (SamuraiAction::KunaiThrow(version), {
            let (input, base_velocity, hits) = match version {
                SpecialVersion::Fast => ("236f", Vec2::new(4.0, 2.0), 1),
                SpecialVersion::Strong => ("236s", Vec2::new(0.9, 4.0), 2),
                SpecialVersion::Metered => ("236(fs)", Vec2::new(10.0, 1.0), 2),
            };

            Action {
                input: Some(input),
                requirement: ActionRequirement::And(vec![
                    ActionRequirement::Grounded,
                    ActionRequirement::Starter(ActionCategory::Special),
                    ActionRequirement::ResourceValue(ResourceType::KunaiCounter, 1),
                ]),
                script: Box::new(move |situation: &Situation| {
                    if situation.on_frame(0) {
                        return vec![
                            Animation::Samurai(SamuraiAnimation::KunaiThrow).into(),
                            ActionEvent::ForceStand,
                            ActionEvent::ModifyResource(ResourceType::KunaiCounter, -1),
                            ActionEvent::Sound(SoundEffect::FemaleKyatchi),
                        ];
                    }

                    if situation.on_frame(11) {
                        let extra_stun = situation.inventory.contains(&ItemId::MiniTasers);

                        let stick_influence = if situation.inventory.contains(&ItemId::Protractor) {
                            situation
                                .facing
                                .mirror_vec2(situation.stick_position.as_vec2())
                                * 0.8
                        } else {
                            Vec2::ZERO
                        };

                        return vec![ActionEvent::SpawnHitbox(Attack {
                            to_hit: ToHit {
                                block_type: Strike(Mid),
                                hitbox: Hitbox(Area::new(0.2, 1.2, 0.3, 0.3)),
                                lifetime: Lifetime::until_owner_hit(),
                                velocity: base_velocity + stick_influence,
                                gravity: 4.0,
                                model: Some(Model::Kunai),
                                hits,
                                projectile: true,
                            },
                            on_hit: StrikeEffectBuilder::new(
                                if extra_stun { 20 } else { 15 },
                                Mid,
                                ActionEvent::HitStun(if extra_stun { 30 } else { 20 }),
                                12,
                            )
                            .with_defender_block_pushback(0.4)
                            .with_chip_damage(2)
                            .with_extra_on_hit_events(if extra_stun {
                                vec![ActionEvent::RelativeVisualEffect(VfxRequest {
                                    effect: VisualEffect::Lightning,
                                    tf: Transform::from_translation(Vec3::Y),
                                    mirror: true,
                                })]
                            } else {
                                vec![]
                            })
                            .build(),
                        })];
                    }

                    situation.end_at(21)
                }),
            }
        })
    })
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    universal_item_actions(Animation::Samurai(SamuraiAnimation::GiParry))
}

fn samurai_items() -> HashMap<ItemId, Item> {
    vec![
        (
            ItemId::IceCube,
            Item {
                cost: 400,
                explanation: "First hit of 2h against airborne opponent freezes their momentum.\n\nLand this for a good day".into(),
                category: ItemCategory::Basic,
                icon: Icon::IceCube,
                ..default()
            },
        ),
        (
            ItemId::SpareKunai,
            Item {
                cost: 250,
                explanation: "Three is better than two".into(),
                category: ItemCategory::Basic,
                icon: Icon::Kunai,
                effect: Stats {
                    kunais: 1,
                    ..Stats::identity()
                },
            },
        ),
        (
            ItemId::KunaiPouch,
            Item {
                cost: 400,
                explanation: "5 uses for Kunai.\n\nThe more the merrier".into(),
                category: ItemCategory::Upgrade(vec![ItemId::SpareKunai]),
                icon: Icon::KunaiPouch,
                effect: Stats {
                    kunais: 2,
                    ..Stats::identity()
                },
            },
        ),
        (
            ItemId::KunaiBelt,
            Item {
                cost: 1000,
                explanation: "8 uses for Kunai.\n\n8 is perfection.".into(),
                category: ItemCategory::Upgrade(vec![ItemId::KunaiPouch]),
                icon: Icon::KunaiBelt,
                effect: Stats {
                    kunais: 3,
                    ..Stats::identity()
                },
            },
        ),
        (
            ItemId::MiniTasers,
            Item {
                cost: 400,
                explanation: "Adds a shock effect to kunais (more stun)".into(),
                category: ItemCategory::Basic,
                icon: Icon::Taser,
                ..default()
            },
        ),
        (
            ItemId::Protractor,
            Item {
                cost: 250,
                explanation: "Stick position influences Kunai velocity\n\n. It's about angles."
                    .into(),
                category: ItemCategory::Basic,
                icon: Icon::Protractor,
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
        (
            ItemId::Fireaxe,
            Item {
                category: ItemCategory::Basic,
                explanation: "6X to do an overhead in sword stance".into(),
                cost: 400,
                icon: Icon::Fireaxe,
                ..default()
            },
        ),
        (
            ItemId::SmokeBomb,
            Item {
                category: ItemCategory::Basic,
                explanation: "Dash in sword stance".into(),
                cost: 1000,
                icon: Icon::SmokeBomb,
                ..default()
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
