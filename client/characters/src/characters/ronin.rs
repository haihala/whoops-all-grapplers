use std::{f32::consts::PI, sync::Arc};

use bevy::{platform::collections::HashMap, prelude::*};

use foundation::{
    ActionId, Animation, AnimationType, Area, CancelType, GameButton, Icon, ItemId, Model, Pickup,
    PickupRequest, RoninAction, RoninAnimation, Sound, SpecialVersion, Stats, StatusCondition,
    StatusFlag, VfxRequest, VisualEffect, VoiceLine, FAST_SWORD_VFX, FPS, METERED_SWORD_VFX,
    METER_BAR_SEGMENT, RONIN_ALT_HELMET_COLOR, RONIN_ALT_JEANS_COLOR, RONIN_ALT_SHIRT_COLOR,
    STRONG_SWORD_VFX,
};

use crate::{
    actions::ActionRequirement,
    items::{universal_item_actions, universal_items},
    jumps,
    resources::{GaugeType, RenderInstructions},
    Action, ActionBuilder, ActionEvent, Attack, AttackBuilder,
    AttackHeight::*,
    CharacterBoxes, CharacterStateBoxes, CharacterUniversals, ConsumableType, CounterVisual,
    DashBuilder, Gauge, HitBuilder, Hitbox, Item, ItemCategory, Lifetime, Movement, Situation,
    StrikeEffectBuilder, Stun, ThrowEffectBuilder, ToHit,
};

use super::Character;

const CHARACTER_UNIVERSALS: CharacterUniversals = CharacterUniversals {
    normal_grunt: Sound::FemaleExhale,

    // TODO: I just threw these here, pick something smart later on
    primary_color: RONIN_ALT_HELMET_COLOR,
    secondary_color: RONIN_ALT_JEANS_COLOR,
};

pub fn ronin() -> Character {
    let (jumps, gravity) = jumps(1.7, 1.0, Animation::Ronin(RoninAnimation::Jump));

    Character::new(
        Model::Ronin,
        Sound::Motivation,
        vec![
            ("T-shirt", RONIN_ALT_SHIRT_COLOR),
            ("Jeans", RONIN_ALT_JEANS_COLOR),
            ("Samurai Helmet.1", RONIN_ALT_HELMET_COLOR),
        ]
        .into_iter()
        .collect(),
        ronin_anims(),
        ronin_moves(jumps),
        ronin_items(),
        ronin_boxes(),
        Stats {
            walk_speed: 1.2,
            back_walk_speed_multiplier: 0.8,
            kunais: 2,
            gravity,
            ..Stats::character_default()
        },
        vec![
            (
                GaugeType::Sharpness,
                Gauge {
                    render_instructions: RenderInstructions::Counter(CounterVisual {
                        label: "Sharpness",
                    }),
                    ..default()
                },
            ),
            (
                GaugeType::KunaiCounter,
                Gauge {
                    render_instructions: RenderInstructions::Counter(CounterVisual {
                        label: "Kunais",
                    }),
                    ..default()
                },
            ),
        ],
        vec![
            (VoiceLine::Defeat, Sound::FemaleNoooo),
            (VoiceLine::BigHit, Sound::FemaleGutPunch),
            (VoiceLine::SmallHit, Sound::FemaleOw),
        ]
        .into_iter()
        .collect(),
    )
}

fn ronin_anims() -> HashMap<AnimationType, Animation> {
    vec![
        (AnimationType::AirIdle, RoninAnimation::Air),
        (AnimationType::AirStun, RoninAnimation::AirStagger),
        (AnimationType::StandIdle, RoninAnimation::Idle),
        (AnimationType::StandBlock, RoninAnimation::Block),
        (AnimationType::StandStun, RoninAnimation::Stagger),
        (AnimationType::WalkBack, RoninAnimation::WalkBack),
        (AnimationType::WalkForward, RoninAnimation::WalkForward),
        (AnimationType::CrouchIdle, RoninAnimation::Crouch),
        (AnimationType::CrouchBlock, RoninAnimation::CrouchBlock),
        (AnimationType::CrouchStun, RoninAnimation::CrouchStagger),
        (AnimationType::Getup, RoninAnimation::Getup),
        (AnimationType::Default, RoninAnimation::StandPose),
    ]
    .into_iter()
    .map(|(k, v)| (k, Animation::from(v)))
    .collect()
}

fn ronin_moves(jumps: impl Iterator<Item = (ActionId, Action)>) -> HashMap<ActionId, Action> {
    jumps
        .chain(dashes())
        .chain(item_actions())
        .chain(
            normals()
                .chain(throws())
                .chain(specials())
                .map(|(k, v)| (ActionId::Ronin(k), v)),
        )
        .collect()
}

fn dashes() -> impl Iterator<Item = (ActionId, Action)> {
    [
        // Grounded forward dash
        DashBuilder::forward()
            .with_animation(RoninAnimation::GroundForwardDash)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(
                0,
                Movement {
                    amount: Vec2::X * 2.0,
                    duration: 4,
                },
            )
            .on_frame(5, Movement::impulse(Vec2::new(2.0, 5.0)))
            .end_at(20)
            .build(),
        // Grounded back dash
        DashBuilder::back()
            .with_animation(RoninAnimation::BackDash)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(0, Movement::impulse(Vec2::X * 6.9))
            .end_at(20)
            .build(),
        // Air forward dash
        DashBuilder::forward()
            .air_only()
            .with_animation(RoninAnimation::AirForwardDash)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(0, Movement::impulse(Vec2::X * 3.0))
            .end_at(20)
            .build(),
        // Air back dash
        DashBuilder::back()
            .air_only()
            .with_animation(RoninAnimation::BackDash)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(0, Movement::impulse(Vec2::X * 3.0))
            .end_at(20)
            .build(),
    ]
    .into_iter()
    .flatten()
}

fn normals() -> impl Iterator<Item = (RoninAction, Action)> {
    debug!("Ronin normals");

    vec![
        (
            RoninAction::KneeThrust,
            AttackBuilder::button(GameButton::Fast)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(RoninAnimation::KneeThrust)
                .with_total_duration(21)
                .with_hit_on_frame(
                    5,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.5, 1.2, 0.35, 0.35)),
                )
                .build(),
        ),
        (
            RoninAction::LowKick,
            AttackBuilder::button(GameButton::Fast)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .crouching()
                .with_animation(RoninAnimation::LowKick)
                .with_total_duration(32)
                .with_hit_on_frame(
                    6,
                    HitBuilder::normal()
                        .hits_low()
                        .with_active_frames(3)
                        .with_hitbox(Area::new(0.7, 0.1, 0.9, 0.2))
                        .with_damage(8)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(6),
                )
                .build(),
        ),
        (
            RoninAction::HeelKick,
            AttackBuilder::button(GameButton::Strong)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(RoninAnimation::HeelKick)
                .with_total_duration(37)
                .with_hit_on_frame(
                    9,
                    HitBuilder::normal()
                        .with_active_frames(6)
                        .with_hitbox(Area::new(0.7, 1.0, 1.0, 0.2))
                        .with_damage(15)
                        .with_advantage_on_block(-8)
                        .with_advantage_on_hit(3)
                        .with_additional_events(vec![Movement {
                            amount: Vec2::X * 3.0,
                            duration: 10,
                        }
                        .into()]),
                )
                .with_extra_initial_events(vec![
                    Movement {
                        amount: Vec2::X * 10.0,
                        duration: 20,
                    }
                    .into(),
                    ActionEvent::Condition(StatusCondition::kara_to(vec![ActionId::GiParry])),
                ])
                .build(),
        ),
        (
            RoninAction::Uppercut,
            AttackBuilder::button(GameButton::Strong)
                .crouching()
                .with_animation(RoninAnimation::Uppercut)
                .with_extra_initial_events(vec![ActionEvent::ExpandHurtbox(
                    Area::new(0.1, 1.0, 0.6, 0.8),
                    30,
                )])
                .with_total_duration(48)
                .with_hit_on_frame(
                    8,
                    HitBuilder::normal()
                        .with_hitbox(Area::new(0.3, 0.7, 0.3, 0.5))
                        .with_active_frames(4)
                        .with_advantage_on_block(0)
                        .with_distance_on_hit(0.9)
                        .with_damage(9)
                        .with_dynamic_on_hit_events(Arc::new(|situation: &Situation| {
                            if situation.inventory.contains(ItemId::IceCube) {
                                vec![
                                    ActionEvent::MultiplyMomentum(Vec2::splat(0.0)),
                                    ActionEvent::Hitstop(20),
                                    ActionEvent::RelativeVisualEffect(VfxRequest {
                                        effect: VisualEffect::Icon(Icon::IceCube),
                                        tf: Transform::from_translation(Vec3::Y * 1.0),
                                        ..default()
                                    }),
                                ]
                            } else {
                                vec![]
                            }
                        }))
                        .launches(Vec2::Y * 3.0),
                )
                .with_hit_on_frame(
                    12,
                    HitBuilder::normal()
                        .with_active_frames(4)
                        .with_hitbox(Area::new(0.35, 1.45, 0.3, 1.2))
                        .with_advantage_on_block(-5)
                        .with_advantage_on_hit(2)
                        .with_damage(6)
                        .with_distance_on_hit(0.1),
                )
                .build(),
        ),
        (
            RoninAction::HighStab,
            AttackBuilder::button(GameButton::Gimmick)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(RoninAnimation::HighStab)
                .with_extra_initial_events(vec![ActionEvent::Condition(StatusCondition::kara_to(
                    vec![ActionId::GiParry],
                ))])
                .with_total_duration(71)
                .with_hit_on_frame(
                    // Drawing hit
                    5,
                    HitBuilder::normal()
                        .with_active_frames(3)
                        .with_hitbox(Area::new(0.2, 1.4, 0.5, 1.2))
                        .with_damage(6)
                        .sword()
                        .with_advantage_on_block(-10)
                        .launches(Vec2::new(0.5, 4.0)),
                )
                .with_vfx_on_frame(
                    5,
                    VisualEffect::WaveFlat(FAST_SWORD_VFX),
                    Transform {
                        translation: Vec3::new(0.2, 1.4, 0.0),
                        rotation: Quat::from_euler(EulerRot::ZYX, 1.0, 0.3, -1.0),
                        scale: Vec3::splat(2.0),
                    },
                )
                .with_hit_on_frame(
                    // Swinging hit
                    21,
                    HitBuilder::normal()
                        .with_active_frames(4)
                        .with_hitbox(Area::new(1.0, 1.4, 1.8, 0.2))
                        .with_damage(6)
                        .sword()
                        .with_advantage_on_block(-16)
                        .with_advantage_on_hit(-6),
                )
                .with_vfx_on_frame(
                    21,
                    VisualEffect::WaveFlat(FAST_SWORD_VFX),
                    Transform {
                        translation: Vec3::new(1.0, 1.4, 0.0),
                        rotation: Quat::from_euler(EulerRot::ZYX, -PI / 2.0, -1.3, 0.2),
                        scale: Vec3::splat(3.0),
                    },
                )
                .build(),
        ),
        (
            RoninAction::SkySlash,
            AttackBuilder::button(GameButton::Gimmick)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .crouching()
                .with_animation(RoninAnimation::SkyStab)
                .with_total_duration(40)
                .with_extra_initial_events(vec![ActionEvent::ExpandHurtbox(
                    Area::new(0.1, 1.0, 0.6, 0.8),
                    40,
                )])
                .with_hit_on_frame(
                    8,
                    HitBuilder::normal()
                        .with_active_frames(5)
                        .with_hitbox(Area::new(1.0, 2.0, 1.0, 1.0))
                        .with_damage(8)
                        .sword()
                        .with_advantage_on_block(-7)
                        .with_advantage_on_hit(10),
                )
                .build(),
        ),
        (
            RoninAction::AirSlice,
            AttackBuilder::button(GameButton::Gimmick)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .air_only()
                .with_animation(RoninAnimation::AirStab)
                .with_total_duration(70)
                .with_hit_on_frame(
                    7,
                    HitBuilder::normal()
                        .with_active_frames(12)
                        .with_hitbox(Area::new(0.0, 0.0, 1.0, 0.4))
                        .with_damage(10)
                        .sword()
                        .with_blockstun(20)
                        .with_hitstun(30),
                )
                .build(),
        ),
        (
            RoninAction::FalconKnee,
            AttackBuilder::button(GameButton::Fast)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .air_only()
                .with_animation(RoninAnimation::FalconKnee)
                .with_total_duration(25)
                .with_hit_on_frame(
                    2,
                    HitBuilder::normal()
                        .with_active_frames(5)
                        .with_hitbox(Area::new(0.4, 0.5, 0.35, 0.25))
                        .with_damage(5)
                        .with_blockstun(10)
                        .with_hitstun(15),
                )
                .build(),
        ),
        (
            RoninAction::FootDiveHold,
            ActionBuilder::button(GameButton::Strong)
                .with_animation(RoninAnimation::FootDiveHold)
                .static_immediate_events(vec![Movement {
                    amount: Vec2::Y * -1.0,
                    duration: 7,
                }
                .into()])
                .air_only()
                .end_at(60 * 60)
                .dyn_events_after_frame(
                    30,
                    Arc::new(|situation: &Situation| {
                        if !situation.held_buttons.contains(&GameButton::Strong) {
                            return vec![ActionEvent::StartAction(
                                RoninAction::FootDiveRelease.into(),
                            )];
                        }
                        vec![]
                    }),
                )
                .build(),
        ),
        (
            RoninAction::FootDiveRelease,
            AttackBuilder::normal()
                .follow_up_from(vec![ActionId::Ronin(RoninAction::FootDiveHold)])
                .with_animation(RoninAnimation::FootDiveRelease)
                .with_total_duration(20)
                .air_only()
                .with_hit_on_frame(
                    3,
                    HitBuilder::normal()
                        .with_active_frames(7)
                        .with_hitbox(Area::new(0.8, -0.2, 0.7, 0.3))
                        .with_blockstun(25)
                        .with_hitstun(40)
                        .with_damage(18)
                        .with_pushback_on_hit(0.3),
                )
                .build(),
        ),
    ]
    .into_iter()
}

fn throws() -> impl Iterator<Item = (RoninAction, Action)> {
    debug!("Ronin throws");

    let (stand_throw_target, stand_throw_activation) = ThrowEffectBuilder::new(
        RoninAnimation::StandThrowHit,
        80,
        RoninAnimation::StandThrowTarget,
        30,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(-2.0, 6.0))
    .build();

    let (crouch_throw_target, crouch_throw_activation) = ThrowEffectBuilder::new(
        RoninAnimation::CrouchThrowHit,
        80,
        RoninAnimation::CrouchThrowTarget,
        30,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(5.0, 2.0))
    .with_extra_target_events(vec![ActionEvent::Teleport(Vec2::new(2.0, 1.0))])
    .build();

    let (air_throw_target, air_throw_activation) = ThrowEffectBuilder::new(
        RoninAnimation::AirThrowHit,
        50,
        RoninAnimation::AirThrowTarget,
        50,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(-2.0, 2.0))
    .build();

    vec![
        (
            RoninAction::ForwardThrow,
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_total_duration(37)
                .with_animation(RoninAnimation::StandThrowStartup)
                .with_extra_initial_events(vec![ActionEvent::Condition(StatusCondition::kara_to(
                    vec![ActionId::GiParry],
                ))])
                .with_hit_on_frame(
                    3,
                    HitBuilder::normal()
                        .forward_throw()
                        .with_active_frames(3)
                        .throw_hit_action(RoninAction::StandThrowHit)
                        .throw_target_action(RoninAction::StandThrowTarget)
                        .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5)),
                )
                .build(),
        ),
        (
            RoninAction::BackThrow,
            AttackBuilder::normal()
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_input("{4}w")
                .with_animation(RoninAnimation::StandThrowStartup)
                .with_total_duration(37)
                .with_hit_on_frame(
                    3,
                    HitBuilder::normal()
                        .back_throw()
                        .with_active_frames(3)
                        .throw_hit_action(RoninAction::StandThrowHit)
                        .throw_target_action(RoninAction::StandThrowTarget)
                        .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5)),
                )
                .build(),
        ),
        (RoninAction::StandThrowHit, stand_throw_activation),
        (RoninAction::StandThrowTarget, stand_throw_target),
        (
            RoninAction::CrouchThrow,
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .crouching()
                .with_total_duration(60)
                .with_animation(RoninAnimation::CrouchThrowStartup)
                .with_hit_on_frame(
                    5,
                    HitBuilder::normal()
                        .with_active_frames(3)
                        .forward_throw()
                        .throw_hit_action(RoninAction::CrouchThrowHit)
                        .throw_target_action(RoninAction::CrouchThrowTarget)
                        .with_hitbox(Area::new(0.7, 0.1, 0.5, 0.2)),
                )
                .build(),
        ),
        (RoninAction::CrouchThrowHit, crouch_throw_activation),
        (RoninAction::CrouchThrowTarget, crouch_throw_target),
        (
            RoninAction::AirThrow,
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .air_only()
                .with_animation(RoninAnimation::AirThrowStartup)
                .with_total_duration(40)
                .with_hit_on_frame(
                    4,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .forward_throw()
                        .throw_hit_action(RoninAction::AirThrowHit)
                        .throw_target_action(RoninAction::AirThrowTarget)
                        .with_hitbox(Area::new(0.4, 0.8, 0.4, 0.4)),
                )
                .build(),
        ),
        (RoninAction::AirThrowHit, air_throw_activation),
        (RoninAction::AirThrowTarget, air_throw_target),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (RoninAction, Action)> {
    debug!("Ronin specials");
    stance_moves().chain(kunai_throws())
}

fn stance_moves() -> impl Iterator<Item = (RoninAction, Action)> {
    vec![
        SpecialVersion::Fast,
        SpecialVersion::Strong,
        SpecialVersion::Metered,
    ]
    .into_iter()
    .flat_map(|version| {
        vec![
            // Base kit
            (RoninAction::SwordStance(version), sword_stance(version)),
            (RoninAction::StanceCancel(version), stance_cancel(version)),
            (RoninAction::ViperStrike(version), viper_strike(version)),
            (RoninAction::RisingSun(version), rising_sun(version)),
            (RoninAction::Sharpen(version), sharpen(version)),
            // Require items
            (RoninAction::SwordSlam(version), sword_slam(version)),
            (
                RoninAction::StanceForwardDash(version),
                stance_dash(version, false),
            ),
            (
                RoninAction::StanceBackDash(version),
                stance_dash(version, true),
            ),
        ]
    })
}

fn sword_stance(version: SpecialVersion) -> Action {
    let (input, metered, buttons) = match version {
        SpecialVersion::Fast => ("{2}*4f", false, vec![GameButton::Fast]),
        SpecialVersion::Strong => ("{2}*4s", false, vec![GameButton::Strong]),
        SpecialVersion::Metered => (
            "{2}*4(fs)",
            true,
            vec![GameButton::Fast, GameButton::Strong],
        ),
    };

    let mut builder = ActionBuilder::special()
        .with_input(input)
        .static_immediate_events({
            let mut events = vec![RoninAnimation::SwordStance.into(), ActionEvent::ForceStand];

            events.push(if metered {
                ActionEvent::Condition(StatusCondition {
                    flag: StatusFlag::Intangible,
                    // 10f of sword stance + 11f of rising sun
                    expiration: Some(22),
                    ..default()
                })
            } else {
                ActionEvent::Condition(StatusCondition::kara_to(vec![ActionId::Ronin(
                    RoninAction::SwordStance(SpecialVersion::Metered),
                )]))
            });
            events
        })
        .static_events_on_frame(
            3,
            vec![ActionEvent::Condition(StatusCondition {
                flag: StatusFlag::Cancel(CancelType::Specific(
                    vec![
                        RoninAction::StanceForwardDash(version),
                        RoninAction::StanceBackDash(version),
                    ]
                    .into_iter()
                    .map(ActionId::Ronin)
                    .collect(),
                )),
                ..default()
            })],
        )
        .dyn_events_after_frame(
            4,
            Arc::new(move |situation: &Situation| {
                if situation.held_buttons.contains(&GameButton::Gimmick) {
                    return vec![ActionEvent::StartAction(
                        RoninAction::Sharpen(version).into(),
                    )];
                }

                if !buttons
                    .iter()
                    .any(|btn| situation.held_buttons.contains(btn))
                {
                    return vec![ActionEvent::StartAction(
                        if situation.stick_position.as_vec2().y == -1.0 {
                            RoninAction::ViperStrike(version)
                        } else if situation
                            .facing
                            .absolute
                            .mirror_vec2(situation.stick_position.as_vec2())
                            .x
                            == -1.0
                        {
                            RoninAction::RisingSun(version)
                        } else if situation
                            .facing
                            .absolute
                            .mirror_vec2(situation.stick_position.as_vec2())
                            .x
                            == 1.0
                            && situation.inventory.contains(ItemId::Fireaxe)
                        {
                            RoninAction::SwordSlam(version)
                        } else {
                            RoninAction::StanceCancel(version)
                        }
                        .into(),
                    )];
                }

                vec![]
            }),
        )
        // Effectively never end
        .end_at(99 * 60);

    if metered {
        builder = builder.with_meter_cost();
    }

    builder.build()
}

fn stance_cancel(version: SpecialVersion) -> Action {
    ActionBuilder::special()
        .with_animation(RoninAnimation::StanceCancel)
        .static_immediate_events(vec![ActionEvent::ClearCondition(StatusFlag::Intangible)])
        .follow_up_from(vec![ActionId::Ronin(RoninAction::SwordStance(version))])
        .end_at(8)
        .build()
}

fn stance_dash(version: SpecialVersion, back: bool) -> Action {
    ActionBuilder::special()
        .with_input(if back { "454" } else { "656" })
        .follow_up_from(vec![ActionId::Ronin(RoninAction::SwordStance(version))])
        .static_immediate_events(vec![
            ActionEvent::Teleport(Vec2::X * if back { -2.0 } else { 2.0 }),
            ActionEvent::RelativeVisualEffect(VfxRequest {
                effect: VisualEffect::SmokeBomb,
                tf: Transform::from_translation(Vec3::Y * 1.5),
                ..default()
            }),
        ])
        .static_events_after_frame(
            10,
            vec![ActionEvent::StartAction(ActionId::Ronin(
                RoninAction::SwordStance(version),
            ))],
        )
        .with_requirement(ActionRequirement::ItemOwned(ItemId::SmokeBomb))
        .build()
}

fn sharpen(version: SpecialVersion) -> Action {
    let (slow, sharpness_gain, meter_gain) = match version {
        SpecialVersion::Metered => (false, 2, 0),
        SpecialVersion::Strong => (true, 2, METER_BAR_SEGMENT),
        SpecialVersion::Fast => (false, 1, METER_BAR_SEGMENT),
    };

    ActionBuilder::special()
        .static_immediate_events(vec![
            if slow {
                RoninAnimation::SlowSharpen
            } else {
                RoninAnimation::FastSharpen
            }
            .into(),
            ActionEvent::Sound(Sound::KnifeChopstickDrag.into()),
        ])
        .static_events_on_frame(
            if slow { 50 } else { 35 },
            vec![
                ActionEvent::ModifyResource(GaugeType::Sharpness, sharpness_gain),
                ActionEvent::ModifyResource(GaugeType::Meter, meter_gain),
                ActionEvent::Sound(Sound::HangingKnifeFlick.into()),
            ],
        )
        .end_at(if slow { 60 } else { 45 })
        .follow_up_from(vec![ActionId::Ronin(RoninAction::SwordStance(version))])
        .build()
}

fn sword_slam(version: SpecialVersion) -> Action {
    let (slow, high_damage, color, launch) = match version {
        SpecialVersion::Strong => (true, true, STRONG_SWORD_VFX, true),
        SpecialVersion::Fast => (false, false, FAST_SWORD_VFX, false),
        SpecialVersion::Metered => (false, true, METERED_SWORD_VFX, true),
    };
    let activation_frame = if slow { 25 } else { 20 };

    AttackBuilder::special()
        .with_character_universals(CHARACTER_UNIVERSALS)
        .follow_up_from(vec![ActionId::Ronin(RoninAction::SwordStance(version))])
        .with_extra_requirement(ActionRequirement::ItemOwned(ItemId::Fireaxe))
        .with_sound(Sound::FemaleKiritsu)
        .with_animation(if slow {
            RoninAnimation::SlowSwordSlam
        } else {
            RoninAnimation::FastSwordSlam
        })
        .with_total_duration(if slow { 80 } else { 60 })
        .with_vfx_on_frame(
            activation_frame,
            VisualEffect::WaveFlat(color),
            Transform {
                translation: Vec3::new(1.0, 0.5, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(4.0),
            },
        )
        .with_hit_on_frame(activation_frame, {
            let mut hit = HitBuilder::special()
                .with_active_frames(2)
                .with_hitbox(Area::new(0.5, 1.0, 2.0, 1.0))
                .hits_overhead()
                .with_damage(if high_damage { 30 } else { 15 })
                .sword()
                .with_distance_on_block(0.1)
                .with_advantage_on_block(if slow { -40 } else { -30 });

            if launch {
                hit = hit.launches(Vec2::new(1.0, 4.0));
            } else {
                hit = hit.with_advantage_on_hit(6);
            }
            hit
        })
        .build()
}

fn viper_strike(version: SpecialVersion) -> Action {
    let (long_lunge, slow, high_damage, color) = match version {
        SpecialVersion::Strong => (true, true, true, STRONG_SWORD_VFX),
        SpecialVersion::Fast => (false, false, false, FAST_SWORD_VFX),
        SpecialVersion::Metered => (false, false, true, METERED_SWORD_VFX),
    };

    let activation_frame = if slow { 10 } else { 5 };

    AttackBuilder::special()
        .with_character_universals(CHARACTER_UNIVERSALS)
        .with_sound(Sound::FemaleShagamu)
        .follow_up_from(vec![ActionId::Ronin(RoninAction::SwordStance(version))])
        .with_animation(if slow {
            RoninAnimation::SlowViperStrike
        } else {
            RoninAnimation::FastViperStrike
        })
        .with_extra_initial_events(vec![Movement {
            amount: Vec2::X * if long_lunge { 12.0 } else { 8.0 },
            duration: 7,
        }
        .into()])
        .with_total_duration(if slow { 50 } else { 45 })
        .with_vfx_on_frame(
            activation_frame,
            VisualEffect::WaveFlat(color),
            Transform {
                translation: Vec3::new(if long_lunge { 1.5 } else { 1.0 }, 0.4, 0.0),
                rotation: Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, PI / 3.0),
                scale: Vec3::splat(4.0),
            },
        )
        .with_hit_on_frame(
            activation_frame,
            HitBuilder::special()
                .with_active_frames(2)
                .with_distance_on_block(0.1)
                .with_hitbox(Area::new(1.0, 0.225, 1.3, 0.45))
                .hits_low()
                .with_damage(if high_damage { 30 } else { 15 })
                .sword()
                .with_advantage_on_hit(if slow { 1 } else { 3 })
                .with_advantage_on_block(if slow { -40 } else { -30 }),
        )
        .build()
}

fn rising_sun(version: SpecialVersion) -> Action {
    let (slow, big, high_bounce, high_damage, color) = match version {
        SpecialVersion::Strong => (true, true, true, true, STRONG_SWORD_VFX),
        SpecialVersion::Fast => (false, false, false, false, FAST_SWORD_VFX),
        SpecialVersion::Metered => (false, false, false, true, METERED_SWORD_VFX),
    };

    let activation_frame = if slow { 14 } else { 4 };
    let size_multiplier = if big { 1.5 } else { 1.0 };

    AttackBuilder::special()
        .with_character_universals(CHARACTER_UNIVERSALS)
        .with_sound(Sound::FemaleHiYah)
        .with_animation(if slow {
            RoninAnimation::SlowRisingSun
        } else {
            RoninAnimation::FastRisingSun
        })
        .follow_up_from(vec![ActionId::Ronin(RoninAction::SwordStance(version))])
        .with_total_duration(if slow { 56 } else { 44 })
        .with_vfx_on_frame(
            activation_frame,
            VisualEffect::WaveDiagonal(color),
            Transform {
                translation: Vec3::new(1.0, 1.7, 0.0),
                rotation: Quat::from_rotation_z(PI / 3.0),
                scale: Vec3::splat(2.0 * size_multiplier),
            },
        )
        .with_hit_on_frame(
            activation_frame,
            HitBuilder::special()
                .with_active_frames(3)
                .sword()
                .with_damage(if high_damage { 20 } else { 15 })
                .launches(if high_bounce {
                    Vec2::new(0.1, 8.0)
                } else {
                    Vec2::new(1.0, 3.0)
                })
                .with_advantage_on_block(-30)
                .with_distance_on_block(0.1)
                .with_hitbox(Area::new(0.25, 1.5, 2.0, 1.5).mul_grow(size_multiplier)),
        )
        .build()
}

fn kunai_throws() -> impl Iterator<Item = (RoninAction, Action)> {
    vec![
        SpecialVersion::Fast,
        SpecialVersion::Strong,
        SpecialVersion::Metered,
    ]
    .into_iter()
    .map(|version| {
        (RoninAction::KunaiThrow(version), {
            let (input, base_velocity, hits, metered) = match version {
                SpecialVersion::Fast => ("{2}*6f", Vec2::new(4.0, 2.0), 1, false),
                SpecialVersion::Strong => ("{2}*6s", Vec2::new(0.9, 4.0), 2, false),
                SpecialVersion::Metered => ("{2}*6(fs)", Vec2::new(10.0, 1.0), 2, true),
            };

            let mut builder = ActionBuilder::special()
                .with_input(input)
                .with_animation(RoninAnimation::KunaiThrow)
                .with_sound(Sound::FemaleKyatchi)
                .with_requirement(ActionRequirement::ResourceValue(GaugeType::KunaiCounter, 1))
                .dyn_events_on_frame(
                    11,
                    Arc::new(move |situation: &Situation| {
                        let extra_stun = situation.inventory.contains(ItemId::MiniTasers);

                        let stick_influence = if situation.inventory.contains(ItemId::Protractor) {
                            situation
                                .facing
                                .absolute
                                .mirror_vec2(situation.stick_position.as_vec2())
                                .normalize()
                                * 0.8
                        } else {
                            Vec2::ZERO
                        };

                        vec![
                            ActionEvent::ModifyResource(GaugeType::KunaiCounter, -1),
                            // TODO: Use hitbuilder
                            ActionEvent::SpawnHitbox(Attack {
                                to_hit: ToHit {
                                    hitbox: Hitbox(Area::new(0.2, 1.2, 0.3, 0.3)),
                                    lifetime: Lifetime::until_despawned(),
                                    velocity: base_velocity + stick_influence,
                                    gravity: 4.0,
                                    model: Some(Model::Kunai),
                                    hits,
                                    projectile: true,
                                    ..default()
                                },
                                on_hit: StrikeEffectBuilder::default()
                                    .with_height(Mid)
                                    .with_blockstun(Stun::Absolute(if extra_stun {
                                        20
                                    } else {
                                        15
                                    }))
                                    .with_damage(12)
                                    .with_defender_block_pushback(0.4)
                                    .with_chip_damage(2)
                                    .with_on_hit_events(vec![ActionEvent::SpawnPickup(
                                        PickupRequest {
                                            pickup: Pickup::Kunai,
                                            spawn_point: Vec2::new(0.5, 1.0),
                                            spawn_velocity: Vec2::Y,
                                            gravity: 4.0,
                                            size: Area::of_size(0.5, 0.5),
                                            lifetime: Some((2.0 * FPS) as usize),
                                            flip_owner: true,
                                        },
                                    )])
                                    .with_on_hit_events(if extra_stun {
                                        vec![
                                            ActionEvent::HitStun(if extra_stun { 30 } else { 20 }),
                                            ActionEvent::RelativeVisualEffect(VfxRequest {
                                                effect: VisualEffect::Lightning,
                                                tf: Transform::from_translation(Vec3::Y),
                                                mirror: true,
                                            }),
                                        ]
                                    } else {
                                        vec![]
                                    })
                                    .build(10),
                            }),
                        ]
                    }),
                )
                .end_at(21);

            builder = if metered {
                builder.with_meter_cost()
            } else {
                builder.static_immediate_events(vec![ActionEvent::Condition(
                    StatusCondition::kara_to(vec![ActionId::Ronin(RoninAction::KunaiThrow(
                        SpecialVersion::Metered,
                    ))]),
                )])
            };

            builder.build()
        })
    })
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    universal_item_actions(
        Animation::Ronin(RoninAnimation::GiParry),
        Animation::Ronin(RoninAnimation::RC),
    )
}

fn ronin_items() -> HashMap<ItemId, Item> {
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
                    ..default()
                },
                suggested: true,
                ..default()
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
                    ..default()
                },
                suggested: true,
                ..default()
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
                    ..default()
                },
                suggested: true,
                ..default()
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
                    ..default()
                },
                ..default()
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
                    ..default()
                },
                ..default()
            },
        ),
        (
            ItemId::Fireaxe,
            Item {
                category: ItemCategory::Basic,
                explanation: "Release stance while holding forward to do an overhead".into(),
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
                suggested: true,
                ..default()
            },
        ),
    ]
    .into_iter()
    .chain(universal_items())
    .collect()
}

fn ronin_boxes() -> CharacterBoxes {
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
