use std::sync::Arc;

use bevy::{platform::collections::HashMap, prelude::*};

use foundation::{
    ActionCategory, ActionId, Animation, AnimationType, Area, CPOAction, CPOAnimation, CancelType,
    GameButton, ItemId, Model, Sound, SpecialVersion, Stats, StatusCondition, StatusFlag,
    StickPosition, VfxRequest, VisualEffect, VoiceLine, CHARGE_BAR_FULL_SEGMENT_COLOR,
    CHARGE_BAR_PARTIAL_SEGMENT_COLOR, CPO_ALT_SHIRT_COLOR, CPO_ALT_SOCKS_COLOR, FPS,
    JACKPOT_HIGH_POINT_PERCENTAGE, JACKPOT_TOTAL_DURATION,
};

use crate::{
    items::{universal_item_actions, universal_items},
    jumps,
    resources::ChargePerfection,
    Action, ActionBuilder, ActionEvent, ActionRequirement, AttackBuilder, AttackHeight,
    CharacterBoxes, CharacterStateBoxes, CharacterUniversals, ChargeProperty, DashBuilder,
    DynamicEvents, Gauge, GaugeType, HitBuilder, Item, Movement, RenderInstructions,
    ResourceBarVisual, Situation, SpecialProperty, ThrowEffectBuilder,
};

use super::Character;

const CHARACTER_UNIVERSALS: CharacterUniversals = CharacterUniversals {
    normal_grunt: Sound::MaleGrunt,
};

pub fn cpo() -> Character {
    let (jumps, gravity) = jumps(1.4, 1.1, Animation::CPO(CPOAnimation::Jump));

    Character::new(
        Model::CPO,
        Sound::Motivation, // TODO: Theme music
        vec![
            // Jacket has a texture which makes it hard
            ("Shirt", CPO_ALT_SHIRT_COLOR),
            ("Sleeves", CPO_ALT_SHIRT_COLOR),
            ("Socks", CPO_ALT_SOCKS_COLOR),
        ]
        .into_iter()
        .collect(),
        cpo_anims(),
        cpo_moves(jumps),
        cpo_items(),
        cpo_boxes(),
        Stats {
            // TODO: Check values
            walk_speed: 1.8,
            back_walk_speed_multiplier: 0.8,
            gravity,
            ..Stats::character_default()
        },
        vec![(
            GaugeType::Charge,
            Gauge {
                render_instructions: RenderInstructions::Bar(ResourceBarVisual {
                    height: 10.0,
                    // TODO: Visual for perfect charge
                    default_color: CHARGE_BAR_PARTIAL_SEGMENT_COLOR,
                    full_color: Some(CHARGE_BAR_FULL_SEGMENT_COLOR),
                    segments: 1,
                    segment_gap: 0.0,
                }),
                max: Some(60),
                special: Some(SpecialProperty::Charge(ChargeProperty {
                    directions: vec![StickPosition::NW, StickPosition::SW, StickPosition::W],
                    ..default()
                })),
                ..default()
            },
        )],
        vec![
            (VoiceLine::Defeat, Sound::MaleNo),
            (VoiceLine::BigHit, Sound::MaleArgh),
            (VoiceLine::SmallHit, Sound::MalePain),
        ]
        .into_iter()
        .collect(),
    )
}

fn cpo_anims() -> HashMap<AnimationType, Animation> {
    vec![
        (AnimationType::AirIdle, CPOAnimation::IdleAir),
        (AnimationType::AirStun, CPOAnimation::HitAir),
        (AnimationType::StandIdle, CPOAnimation::IdleStand),
        (AnimationType::StandBlock, CPOAnimation::BlockStand),
        (AnimationType::StandStun, CPOAnimation::HitStand),
        (AnimationType::WalkBack, CPOAnimation::WalkBack),
        (AnimationType::WalkForward, CPOAnimation::WalkForward),
        (AnimationType::CrouchIdle, CPOAnimation::IdleCrouch),
        (AnimationType::CrouchBlock, CPOAnimation::BlockCrouch),
        (AnimationType::CrouchStun, CPOAnimation::HitCrouch),
        (AnimationType::Getup, CPOAnimation::Getup),
        (AnimationType::Default, CPOAnimation::NeutralStandPose),
    ]
    .into_iter()
    .map(|(k, v)| (k, Animation::from(v)))
    .collect()
}

fn cpo_moves(jumps: impl Iterator<Item = (ActionId, Action)>) -> HashMap<ActionId, Action> {
    jumps
        .chain(dashes())
        .chain(item_actions())
        .chain(
            normals()
                .chain(throws())
                .chain(specials())
                .map(|(k, v)| (ActionId::CPO(k), v)),
        )
        .collect()
}

fn dashes() -> impl Iterator<Item = (ActionId, Action)> {
    [
        // Grounded forward dash
        DashBuilder::forward()
            .with_animation(CPOAnimation::DashGroundForward)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(
                0,
                Movement {
                    amount: Vec2::X * 4.0,
                    duration: 4,
                },
            )
            .on_frame(5, Movement::impulse(Vec2::X * 4.0))
            .end_at(20)
            .build(),
        // Grounded back dash
        DashBuilder::back()
            .with_animation(CPOAnimation::DashGroundBack)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(0, Movement::impulse(Vec2::X * 6.9))
            .end_at(20)
            .build(),
        // Air forward dash
        DashBuilder::forward()
            .air_only()
            .with_animation(CPOAnimation::DashAirForward)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(0, Movement::impulse(Vec2::X * 3.0))
            .end_at(20)
            .build(),
        // Air back dash
        DashBuilder::back()
            .air_only()
            .with_animation(CPOAnimation::DashAirBack)
            .with_character_universals(CHARACTER_UNIVERSALS)
            .on_frame(0, Movement::impulse(Vec2::X * 3.0))
            .end_at(20)
            .build(),
    ]
    .into_iter()
    .flatten()
}

fn normals() -> impl Iterator<Item = (CPOAction, Action)> {
    debug!("CPO normals");

    vec![
        (
            CPOAction::Chop,
            AttackBuilder::button(GameButton::Fast)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(CPOAnimation::Chop)
                .with_total_duration(20)
                .with_hit_on_frame(
                    6,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.2, 1.8, 0.5, 0.5)),
                )
                .with_hit_on_frame(
                    9,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.6, 1.1, 0.25, 0.35)),
                )
                .build(),
        ),
        (
            CPOAction::DickJab,
            AttackBuilder::button(GameButton::Fast)
                .crouching()
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(CPOAnimation::DickJab)
                .with_total_duration(15)
                .with_hit_on_frame(
                    4,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.75, 0.6, 0.35, 0.35)),
                )
                .build(),
        ),
        (
            CPOAction::JumpingKnees,
            AttackBuilder::button(GameButton::Fast)
                .air_only()
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(CPOAnimation::JumpingKnees)
                .with_total_duration(50)
                .with_hit_on_frame(
                    4,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.25, 0.65, 0.35, 0.35)),
                )
                .build(),
        ),
        (
            CPOAction::HookPunch,
            AttackBuilder::button(GameButton::Strong)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(CPOAnimation::HookPunch)
                .with_total_duration(25)
                .with_hit_on_frame(
                    10,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.6, 1.3, 0.35, 0.35)),
                )
                .build(),
        ),
        (
            CPOAction::Stomp1,
            AttackBuilder::button(GameButton::Strong)
                .crouching()
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(CPOAnimation::Stomp1)
                .with_total_duration(40)
                .with_hit_on_frame(
                    13,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.55, 0.2, 0.35, 0.35))
                        .with_cancels_to(CancelType::Specific(vec![CPOAction::Stomp2.into()]), 20),
                )
                .build(),
        ),
        (
            CPOAction::Stomp2,
            AttackBuilder::button(GameButton::Strong)
                .crouching()
                .follow_up_from(vec![CPOAction::Stomp1.into()])
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(CPOAnimation::Stomp2)
                .with_total_duration(35)
                .with_hit_on_frame(
                    10,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.6, 0.2, 0.35, 0.35))
                        .with_cancels_to(CancelType::Specific(vec![CPOAction::Stomp3.into()]), 20),
                )
                .build(),
        ),
        (
            CPOAction::Stomp3,
            AttackBuilder::button(GameButton::Strong)
                .crouching()
                .follow_up_from(vec![CPOAction::Stomp2.into()])
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(CPOAnimation::Stomp3)
                .with_total_duration(70)
                .with_hit_on_frame(
                    12,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.65, 0.2, 0.35, 0.35)),
                )
                .build(),
        ),
        (
            CPOAction::BodySplash,
            AttackBuilder::button(GameButton::Strong)
                .air_only()
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(CPOAnimation::BodySplash)
                .with_total_duration(150)
                .with_hit_on_frame(
                    5,
                    HitBuilder::normal()
                        .with_active_frames(150)
                        .with_damage(5)
                        .with_advantage_on_block(-1)
                        .with_advantage_on_hit(4)
                        .with_hitbox(Area::new(0.1, 1.2, 1.35, 0.5)),
                )
                .build(),
        ),
        (
            CPOAction::Jackpot,
            ActionBuilder::for_category(ActionCategory::MegaInterrupt)
                .with_input("g")
                .make_transient()
                .with_character_universals(CHARACTER_UNIVERSALS)
                .every_frame(crate::Events {
                    dynamic: Some(Arc::new(|situation: &Situation| {
                        vec![
                            ActionEvent::Condition(StatusCondition {
                                flag: StatusFlag::Jackpot {
                                    target_frame: situation.abs_frame
                                        + (JACKPOT_HIGH_POINT_PERCENTAGE
                                            * JACKPOT_TOTAL_DURATION
                                            * FPS)
                                            as usize,
                                },
                                expiration: Some((JACKPOT_TOTAL_DURATION * FPS) as usize),
                                ..default()
                            }),
                            ActionEvent::RelativeVisualEffect(VfxRequest {
                                effect: VisualEffect::JackpotRing,
                                tf: Transform::from_translation(Vec3::Y * 1.5),
                                ..default()
                            }),
                        ]
                    })),
                    ..default()
                })
                .with_requirement(ActionRequirement::NoStatusMatches(|sf| {
                    matches!(sf, &StatusFlag::Jackpot { target_frame: _ })
                }))
                .build(),
        ),
    ]
    .into_iter()
}

fn throws() -> impl Iterator<Item = (CPOAction, Action)> {
    debug!("CPO throws");

    let (forward_throw_recipient, forward_throw_hit) = ThrowEffectBuilder::new(
        CPOAnimation::ThrowGroundHit,
        80,
        CPOAnimation::ThrowGroundForwardRecipient,
        30,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(-2.0, 6.0))
    .build();

    let (back_throw_recipient, _) = ThrowEffectBuilder::new(
        CPOAnimation::ThrowGroundHit,
        80,
        CPOAnimation::ThrowGroundBackRecipient,
        30,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(5.0, 2.0))
    .with_extra_target_events(vec![ActionEvent::Teleport(Vec2::new(2.0, 1.0))])
    .build();

    let (air_throw_recipient, air_throw_hit) = ThrowEffectBuilder::new(
        CPOAnimation::ThrowAirHit,
        50,
        CPOAnimation::ThrowAirRecipient,
        50,
    )
    .with_damage(10)
    .with_launch_impulse(Vec2::new(-2.0, 2.0))
    .build();

    vec![
        (
            CPOAction::ForwardThrowStartup,
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_total_duration(37)
                .with_animation(CPOAnimation::ThrowGroundStartup)
                .with_extra_initial_events(vec![ActionEvent::Condition(StatusCondition::kara_to(
                    vec![ActionId::GiParry],
                ))])
                .with_hit_on_frame(
                    3,
                    HitBuilder::normal()
                        .forward_throw()
                        .with_active_frames(3)
                        .throw_hit_action(CPOAction::GroundThrowHit)
                        .throw_target_action(CPOAction::ForwardThrowRecipient)
                        .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5)),
                )
                .build(),
        ),
        (
            CPOAction::BackThrowStartup,
            AttackBuilder::normal()
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_input("{4}w")
                .with_animation(CPOAnimation::ThrowGroundStartup)
                .with_total_duration(37)
                .with_hit_on_frame(
                    3,
                    HitBuilder::normal()
                        .back_throw()
                        .with_active_frames(3)
                        .throw_hit_action(CPOAction::GroundThrowHit)
                        .throw_target_action(CPOAction::BackThrowRecipient)
                        .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5)),
                )
                .build(),
        ),
        (CPOAction::GroundThrowHit, forward_throw_hit),
        (CPOAction::ForwardThrowRecipient, forward_throw_recipient),
        (CPOAction::BackThrowRecipient, back_throw_recipient),
        (
            CPOAction::AirThrowStartup,
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .air_only()
                .with_animation(CPOAnimation::ThrowAirStartup)
                .with_total_duration(40)
                .with_hit_on_frame(
                    4,
                    HitBuilder::normal()
                        .with_active_frames(2)
                        .forward_throw()
                        .throw_hit_action(CPOAction::AirThrowHit)
                        .throw_target_action(CPOAction::AirThrowRecipient)
                        .with_hitbox(Area::new(0.4, 0.8, 0.4, 0.4)),
                )
                .build(),
        ),
        (CPOAction::AirThrowHit, air_throw_hit),
        (CPOAction::AirThrowRecipient, air_throw_recipient),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (CPOAction, Action)> {
    debug!("CPO specials");

    vec![
        SpecialVersion::Fast,
        SpecialVersion::Strong,
        SpecialVersion::Metered,
    ]
    .into_iter()
    .flat_map(|strength| {
        vec![
            (
                CPOAction::GroundTimeWinderStraight(strength),
                timewinder(strength, AttackHeight::Mid),
            ),
            (
                CPOAction::GroundTimeWinderLow(strength),
                timewinder(strength, AttackHeight::Low),
            ),
            (
                CPOAction::AirTimewinder(strength),
                timewinder(strength, AttackHeight::High),
            ),
        ]
    })
}

const TIMEWINDER_STRAIGHT_BASE_MOMENTUM: f32 = 15.0;
const TIMEWINDER_LOW_BASE_MOMENTUM: f32 = 10.0;
const TIMEWINDER_AIR_BASE_MOMENTUM: f32 = 7.0;

const FAST_TIMEWINDER_DISTANCE_MUL: f32 = 0.6;
const STRONG_TIMEWINDER_DISTANCE_MUL: f32 = 0.5;
const METERED_TIMEWINDER_DISTANCE_MUL: f32 = 0.7;

const TIMEWINDER_SECONDARY_MOVEMENT: f32 = 6.0;

fn timewinder_dynamic_initial(version: SpecialVersion, height: AttackHeight) -> DynamicEvents {
    Arc::new(move |situation: &Situation| {
        let momentum = match ChargePerfection::from_gauge(
            situation.get_resource(GaugeType::Charge).unwrap(),
        ) {
            ChargePerfection::Early(val) => 1.5 * val,
            ChargePerfection::Perfect => 2.0,
            ChargePerfection::Over => 1.0,
        } * match height {
            AttackHeight::Low => TIMEWINDER_LOW_BASE_MOMENTUM,
            AttackHeight::Mid => TIMEWINDER_STRAIGHT_BASE_MOMENTUM,
            AttackHeight::High => TIMEWINDER_AIR_BASE_MOMENTUM,
        } * match version {
            SpecialVersion::Metered => METERED_TIMEWINDER_DISTANCE_MUL,
            SpecialVersion::Strong => STRONG_TIMEWINDER_DISTANCE_MUL,
            SpecialVersion::Fast => FAST_TIMEWINDER_DISTANCE_MUL,
        };

        vec![
            ActionEvent::ClearResource(GaugeType::Charge),
            ActionEvent::MultiplyMomentum(Vec2::new(0.8, 0.0)), // Stops falling / rising
            ActionEvent::Movement(Movement::impulse(Vec2::X * momentum)),
            ActionEvent::RelativeVisualEffect(VfxRequest {
                // TODO: Use bezier
                effect: VisualEffect::SpeedLines,
                ..default()
            }),
        ]
    })
}

fn fast_timewinder(height: AttackHeight) -> Action {
    let mut builder = AttackBuilder::special()
        .with_character_universals(CHARACTER_UNIVERSALS)
        .with_input(match height {
            AttackHeight::Low => "3+f",
            AttackHeight::Mid => "6+f",
            AttackHeight::High => "[369]+f",
        })
        .with_animation(match height {
            AttackHeight::Mid => CPOAnimation::TimewinderGroundStraight,
            AttackHeight::Low => CPOAnimation::TimewinderGroundLow,
            AttackHeight::High => CPOAnimation::TimewinderAirStrike,
        })
        .with_total_duration(60)
        .with_extra_requirement(ActionRequirement::ResourceValue(GaugeType::Charge, 20))
        .with_extra_initial_dyn_events(timewinder_dynamic_initial(SpecialVersion::Fast, height))
        .with_hit_on_frame(9, {
            let hb = HitBuilder::special()
                .with_attack_height(height)
                .with_active_frames(3)
                .with_hitbox(Area::new(
                    1.0,
                    match height {
                        AttackHeight::Mid => 1.1,
                        AttackHeight::Low => 0.2,
                        AttackHeight::High => 0.5,
                    },
                    1.2,
                    0.4,
                ))
                .with_damage(12);

            match height {
                AttackHeight::Low => hb
                    .with_advantage_on_block(-25)
                    .with_advantage_on_hit(2)
                    .knocks_down(),
                AttackHeight::Mid => hb
                    .with_advantage_on_block(-8)
                    .with_advantage_on_hit(5)
                    // In the rare case it hits airborne
                    .with_strike_builder(|sb| sb.with_juggle_impulse(Vec2::new(-5.0, 1.0))),
                AttackHeight::High => {
                    // Air version, guess numbers
                    hb.with_blockstun(20).with_hitstun(30)
                }
            }
        });

    if height == AttackHeight::High {
        builder = builder.air_only();
    }

    builder.build()
}

fn timewinder(version: SpecialVersion, height: AttackHeight) -> Action {
    if version == SpecialVersion::Fast {
        // Doesn't have the shoulder part, so we separate it this way
        return fast_timewinder(height);
    }

    let mut builder = AttackBuilder::special()
        .with_character_universals(CHARACTER_UNIVERSALS)
        .with_input(match (version, height) {
            (SpecialVersion::Strong, AttackHeight::Low) => "3+s",
            (SpecialVersion::Strong, AttackHeight::Mid) => "6+s",
            (SpecialVersion::Strong, AttackHeight::High) => "[369]+s",
            (SpecialVersion::Metered, AttackHeight::Low) => "3+s",
            (SpecialVersion::Metered, AttackHeight::Mid) => "6+s",
            (SpecialVersion::Metered, AttackHeight::High) => "[369]+s",
            _ => panic!("Unexpected version/height combo for timewinder: {version:?}/{height:?}"),
        })
        .with_animation(match height {
            AttackHeight::Mid | AttackHeight::Low => CPOAnimation::TimewinderGroundShoulder,
            AttackHeight::High => CPOAnimation::TimewinderAirShoulder,
        })
        .with_total_duration(82)
        .with_extra_requirement(ActionRequirement::ResourceValue(GaugeType::Charge, 20))
        .with_extra_initial_dyn_events(timewinder_dynamic_initial(version, height))
        .with_hit_on_frame(20, {
            HitBuilder::special()
                .with_active_frames(2)
                .launches(Vec2::new(0.5, 3.0))
                .with_damage(5)
                .with_hitbox(Area::new(0.6, 1.1, 0.5, 1.0))
                .with_blockstun(5) // Just needs to last long enough for second hit to connect
        })
        .with_extra_events(
            22,
            vec![
                ActionEvent::Animation(
                    Animation::CPO(match height {
                        AttackHeight::Mid => CPOAnimation::TimewinderGroundStraight,
                        AttackHeight::Low => CPOAnimation::TimewinderGroundLow,
                        AttackHeight::High => CPOAnimation::TimewinderAirStrike,
                    })
                    .into(),
                ),
                ActionEvent::Movement(Movement::impulse(Vec2::X * TIMEWINDER_SECONDARY_MOVEMENT)),
            ],
        )
        .with_hit_on_frame(29, {
            let mut hb = HitBuilder::special()
                .with_attack_height(height)
                .with_active_frames(3)
                .with_hitbox(Area::new(
                    1.0,
                    match height {
                        AttackHeight::Mid => 1.1,
                        AttackHeight::Low => 0.2,
                        AttackHeight::High => 0.5,
                    },
                    1.2,
                    0.4,
                ))
                .with_damage(12);

            match height {
                AttackHeight::Low => {
                    hb = hb.with_advantage_on_block(-15);
                    hb = hb.with_advantage_on_hit(4);
                }
                AttackHeight::Mid => {
                    hb = hb.with_advantage_on_block(-8);
                    hb = hb.with_advantage_on_hit(8);
                    hb = hb.with_strike_builder(|sb| sb.with_juggle_impulse(Vec2::new(-5.0, 6.0)))
                }
                AttackHeight::High => {
                    // Air version, guess numbers
                    hb = hb.with_blockstun(20);
                    hb = hb.with_hitstun(30);
                }
            }

            hb
        });

    if height == AttackHeight::High {
        builder = builder.air_only();
    }

    builder.build()
}

fn item_actions() -> impl Iterator<Item = (ActionId, Action)> {
    universal_item_actions(
        Animation::CPO(CPOAnimation::GiParry),
        Animation::CPO(CPOAnimation::RC),
    )
}

fn cpo_items() -> HashMap<ItemId, Item> {
    vec![].into_iter().chain(universal_items()).collect()
}

fn cpo_boxes() -> CharacterBoxes {
    CharacterBoxes {
        standing: CharacterStateBoxes {
            head: Area::new(0.0, 1.9, 0.6, 0.5),
            chest: Area::new(0.0, 1.3, 0.7, 0.8),
            legs: Area::new(0.0, 0.6, 0.8, 1.2),
            pushbox: Area::new(0.0, 0.7, 0.4, 1.4),
        },
        crouching: CharacterStateBoxes {
            head: Area::new(0.1, 1.5, 0.6, 0.5),
            chest: Area::new(0.0, 1.0, 0.7, 0.8),
            legs: Area::new(0.0, 0.5, 0.8, 1.0),
            pushbox: Area::new(0.0, 0.5, 0.4, 1.0),
        },
        airborne: CharacterStateBoxes {
            head: Area::new(0.0, 1.9, 0.6, 0.5),
            chest: Area::new(0.0, 1.3, 1.2, 0.8),
            legs: Area::new(0.0, 1.0, 1.3, 0.7),
            pushbox: Area::new(0.0, 1.6, 0.4, 0.6),
        },
    }
}
