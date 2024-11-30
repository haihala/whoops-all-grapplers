use std::{f32::consts::PI, sync::Arc};

use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionId, Animation, AnimationType, Area, CancelType, Facing, GameButton, Icon, ItemId, Model,
    SamuraiAction, SamuraiAnimation, SoundEffect, SpecialVersion, Stats, StatusCondition,
    StatusFlag, VfxRequest, VisualEffect, VoiceLine, FAST_SWORD_VFX, METERED_SWORD_VFX,
    METER_BAR_SEGMENT, SAMURAI_ALT_HELMET_COLOR, SAMURAI_ALT_JEANS_COLOR, SAMURAI_ALT_SHIRT_COLOR,
    STRONG_SWORD_VFX,
};

use crate::{
    actions::ActionRequirement,
    items::{universal_item_actions, universal_items},
    jumps,
    resources::{RenderInstructions, ResourceType},
    Action, ActionBuilder, ActionEvent, Attack, AttackBuilder,
    AttackHeight::*,
    BlockType::*,
    CharacterBoxes, CharacterStateBoxes, CharacterUniversals, ConsumableType, CounterVisual,
    DashBuilder, Hitbox, Item, ItemCategory, Lifetime, Movement, Situation, StrikeEffectBuilder,
    ThrowEffectBuilder, ToHit, WAGResource,
};

use super::Character;

const CHARACTER_UNIVERSALS: CharacterUniversals = CharacterUniversals {
    normal_grunt: SoundEffect::FemaleExhale,
};

pub fn samurai() -> Character {
    let (jumps, gravity) = jumps(1.7, 1.0, Animation::Samurai(SamuraiAnimation::Jump));

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
            walk_speed: 2.0,
            back_walk_speed_multiplier: 0.5,
            kunais: 2,
            gravity,
            ..Stats::character_default()
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
        .chain(dashes())
        .chain(item_actions())
        .chain(
            normals()
                .chain(throws())
                .chain(specials())
                .map(|(k, v)| (ActionId::Samurai(k), v)),
        )
        .collect()
}

fn dashes() -> impl Iterator<Item = (ActionId, Action)> {
    let forward = DashBuilder::forward()
        .with_animation(SamuraiAnimation::DashForward)
        .with_character_universals(CHARACTER_UNIVERSALS)
        .on_frame(
            0,
            Movement {
                amount: Vec2::X * 2.0,
                duration: 4,
            },
        )
        .on_frame(5, Movement::impulse(Vec2::new(2.0, 5.0)))
        .end_at(17)
        .build();

    let back = DashBuilder::back()
        .with_animation(SamuraiAnimation::DashBack)
        .with_character_universals(CHARACTER_UNIVERSALS)
        .on_frame(0, Movement::impulse(Vec2::X * 6.9))
        .end_at(20)
        .build();

    forward.chain(back)
}

fn normals() -> impl Iterator<Item = (SamuraiAction, Action)> {
    debug!("Samurai normals");

    vec![
        (
            SamuraiAction::KneeThrust,
            AttackBuilder::button(GameButton::Fast)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(SamuraiAnimation::KneeThrust)
                .with_frame_data(5, 2, 16)
                .with_hitbox(Area::new(0.5, 1.2, 0.35, 0.35))
                .with_damage(5)
                .with_advantage_on_block(-1)
                .with_advantage_on_hit(4)
                .with_extra_initial_events(vec![ActionEvent::Condition(StatusCondition::kara_to(
                    vec![ActionId::GiParry],
                ))])
                .build(),
        ),
        (
            SamuraiAction::LowKick,
            AttackBuilder::button(GameButton::Fast)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .crouching()
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
            AttackBuilder::button(GameButton::Strong)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_animation(SamuraiAnimation::HeelKick)
                .with_frame_data(9, 6, 28)
                .with_hitbox(Area::new(0.7, 1.0, 1.0, 0.2))
                .with_damage(15)
                .with_advantage_on_block(-8)
                .with_advantage_on_hit(3)
                .with_extra_initial_events(vec![
                    Movement {
                        amount: Vec2::X * 10.0,
                        duration: 20,
                    }
                    .into(),
                    ActionEvent::Condition(StatusCondition::kara_to(vec![ActionId::GiParry])),
                ])
                .with_extra_activation_events(vec![Movement {
                    amount: Vec2::X * 3.0,
                    duration: 10,
                }
                .into()])
                .build(),
        ),
        (
            SamuraiAction::Uppercut,
            ActionBuilder::button(GameButton::Strong)
                .crouching()
                .with_animation(SamuraiAnimation::Uppercut)
                .immediate_events(vec![ActionEvent::ExpandHurtbox(
                    Area::new(0.1, 1.0, 0.6, 0.8),
                    30,
                )])
                .dyn_events_on_frame(
                    8,
                    Arc::new(|situation: &Situation| {
                        let hitbox = Area::new(0.3, 0.7, 0.3, 0.5);
                        vec![
                            ActionEvent::ExpandHurtbox(hitbox.grow(0.1), 8),
                            ActionEvent::Condition(StatusCondition {
                                flag: StatusFlag::Cancel(CancelType::Special),
                                expiration: Some(30),
                                ..default()
                            }),
                            ActionEvent::SpawnHitbox(Attack {
                                to_hit: ToHit {
                                    hitbox: Hitbox(hitbox),
                                    lifetime: Lifetime::frames(4),
                                    ..default()
                                },
                                on_hit: StrikeEffectBuilder::default()
                                    .with_height(Mid)
                                    .with_blockstun(40)
                                    .with_damage(9)
                                    .with_distance_on_hit(0.9)
                                    .with_on_hit_events({
                                        let mut evs = vec![ActionEvent::LaunchStun(Vec2::Y * 6.0)];

                                        if situation.inventory.contains(&ItemId::IceCube) {
                                            evs.extend(vec![
                                                ActionEvent::ClearMovement,
                                                ActionEvent::RelativeVisualEffect(VfxRequest {
                                                    effect: VisualEffect::Icon(Icon::IceCube),
                                                    tf: Transform::from_translation(Vec3::Y * 1.0),
                                                    ..default()
                                                }),
                                            ]);
                                        }

                                        evs
                                    })
                                    .build(),
                            }),
                        ]
                    }),
                )
                .events_on_frame(12, {
                    let hitbox = Area::new(0.35, 1.45, 0.3, 1.2);
                    vec![
                        ActionEvent::ExpandHurtbox(hitbox.grow(0.1), 8),
                        ActionEvent::SpawnHitbox(Attack {
                            to_hit: ToHit {
                                hitbox: Hitbox(hitbox),
                                lifetime: Lifetime::frames(4),
                                ..default()
                            },

                            on_hit: StrikeEffectBuilder::default()
                                .with_height(Mid)
                                .with_on_hit_events(vec![ActionEvent::HitStun(38)])
                                .with_blockstun(30)
                                .with_damage(6)
                                .with_distance_on_hit(0.1)
                                .build(),
                        }),
                    ]
                })
                .end_at(48)
                .build(),
        ),
        (
            SamuraiAction::HighStab,
            AttackBuilder::button(GameButton::Gimmick)
                .with_character_universals(CHARACTER_UNIVERSALS)
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
            AttackBuilder::button(GameButton::Gimmick)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .crouching()
                .with_animation(SamuraiAnimation::SkyStab)
                .with_frame_data(8, 5, 32)
                .with_hitbox(Area::new(1.0, 2.0, 1.0, 1.0))
                .with_damage(8)
                .sword()
                .with_advantage_on_block(-7)
                .with_advantage_on_hit(10)
                .with_extra_initial_events(vec![ActionEvent::ExpandHurtbox(
                    Area::new(0.1, 1.0, 0.6, 0.8),
                    40,
                )])
                .build(),
        ),
        (
            SamuraiAction::AirSlice,
            AttackBuilder::button(GameButton::Gimmick)
                .with_character_universals(CHARACTER_UNIVERSALS)
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
            AttackBuilder::button(GameButton::Fast)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .air_only()
                .with_animation(SamuraiAnimation::FalconKnee)
                .with_frame_data(2, 5, 23)
                .with_hitbox(Area::new(0.4, 0.5, 0.35, 0.25))
                .with_damage(5)
                .with_blockstun(10)
                .with_hitstun(15)
                .build(),
        ),
        (
            SamuraiAction::FootDiveHold,
            ActionBuilder::button(GameButton::Strong)
                .with_animation(SamuraiAnimation::FootDiveHold)
                .immediate_events(vec![Movement {
                    amount: Vec2::Y * -1.0,
                    duration: 7,
                }
                .into()])
                .air_only()
                .end_at(60 * 60) // TODO: Get the max length of the animation here
                .dyn_events_after_frame(
                    30,
                    Arc::new(|situation: &Situation| {
                        if !situation.held_buttons.contains(&GameButton::Strong) {
                            return vec![ActionEvent::StartAction(
                                SamuraiAction::FootDiveRelease.into(),
                            )];
                        }
                        vec![]
                    }),
                )
                .build(),
        ),
        (
            SamuraiAction::FootDiveRelease,
            AttackBuilder::normal()
                .with_animation(SamuraiAnimation::FootDiveRelease)
                .with_frame_data(3, 7, 17)
                .with_hitbox(Area::new(0.8, -0.2, 0.7, 0.3))
                .air_only()
                .with_blockstun(25)
                .with_hitstun(40)
                .with_damage(18)
                .with_pushback_on_hit(0.3)
                .build(),
        ),
    ]
    .into_iter()
}

fn throws() -> impl Iterator<Item = (SamuraiAction, Action)> {
    debug!("Samurai throws");

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
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
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
            AttackBuilder::normal()
                .with_character_universals(CHARACTER_UNIVERSALS)
                .with_input("{4}w")
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
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .crouching()
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
            AttackBuilder::button(GameButton::Wrestling)
                .with_character_universals(CHARACTER_UNIVERSALS)
                .forward_throw()
                .air_only()
                .throw_hit_action(SamuraiAction::AirThrowHit)
                .throw_target_action(SamuraiAction::AirThrowTarget)
                .with_frame_data(4, 2, 36)
                .with_animation(SamuraiAnimation::AirThrowStartup)
                .with_hitbox(Area::new(0.4, 0.8, 0.4, 0.4))
                .build(),
        ),
        (SamuraiAction::AirThrowHit, air_throw_activation),
        (SamuraiAction::AirThrowTarget, air_throw_target),
    ]
    .into_iter()
}

fn specials() -> impl Iterator<Item = (SamuraiAction, Action)> {
    debug!("Samurai specials");
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
        .immediate_events({
            let mut events = vec![
                SamuraiAnimation::SwordStance.into(),
                ActionEvent::ForceStand,
            ];

            events.push(if metered {
                ActionEvent::Condition(StatusCondition {
                    flag: StatusFlag::Intangible,
                    // 10f of sword stance + 11f of rising sun
                    expiration: Some(22),
                    ..default()
                })
            } else {
                ActionEvent::Condition(StatusCondition::kara_to(vec![ActionId::Samurai(
                    SamuraiAction::SwordStance(SpecialVersion::Metered),
                )]))
            });
            events
        })
        .events_on_frame(
            3,
            vec![ActionEvent::Condition(StatusCondition {
                flag: StatusFlag::Cancel(CancelType::Specific(
                    vec![
                        SamuraiAction::StanceForwardDash(version),
                        SamuraiAction::StanceBackDash(version),
                    ]
                    .into_iter()
                    .map(ActionId::Samurai)
                    .collect(),
                )),
                expiration: Some(30),
                ..default()
            })],
        )
        .dyn_events_after_frame(
            4,
            Arc::new(move |situation: &Situation| {
                if situation.held_buttons.contains(&GameButton::Gimmick) {
                    return vec![ActionEvent::StartAction(
                        SamuraiAction::Sharpen(version).into(),
                    )];
                }

                if !buttons
                    .iter()
                    .all(|btn| situation.held_buttons.contains(btn))
                {
                    return vec![ActionEvent::StartAction(
                        if situation.stick_position.as_vec2().y == -1.0 {
                            SamuraiAction::ViperStrike(version)
                        } else if situation
                            .facing
                            .mirror_vec2(situation.stick_position.as_vec2())
                            .x
                            == -1.0
                        {
                            SamuraiAction::RisingSun(version)
                        } else if situation
                            .facing
                            .mirror_vec2(situation.stick_position.as_vec2())
                            .x
                            == 1.0
                            && situation.inventory.contains(&ItemId::Fireaxe)
                        {
                            SamuraiAction::SwordSlam(version)
                        } else {
                            SamuraiAction::StanceCancel(version)
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
        .with_animation(SamuraiAnimation::StanceCancel)
        .immediate_events(vec![ActionEvent::ClearCondition(StatusFlag::Intangible)])
        .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
        .end_at(8)
        .build()
}

fn stance_dash(version: SpecialVersion, back: bool) -> Action {
    ActionBuilder::special()
        .with_input(if back { "454" } else { "656" })
        .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
        .immediate_events(vec![
            ActionEvent::Teleport(Vec2::X * if back { -2.0 } else { 2.0 }),
            ActionEvent::RelativeVisualEffect(VfxRequest {
                effect: VisualEffect::SmokeBomb,
                tf: Transform::from_translation(Vec3::Y * 1.5),
                ..default()
            }),
        ])
        .events_after_frame(
            10,
            vec![ActionEvent::StartAction(ActionId::Samurai(
                SamuraiAction::SwordStance(version),
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
        .immediate_events(vec![
            if slow {
                SamuraiAnimation::SlowSharpen
            } else {
                SamuraiAnimation::FastSharpen
            }
            .into(),
            ActionEvent::Sound(SoundEffect::KnifeChopstickDrag),
        ])
        .events_on_frame(
            if slow { 50 } else { 35 },
            vec![
                ActionEvent::ModifyResource(ResourceType::Sharpness, sharpness_gain),
                ActionEvent::ModifyResource(ResourceType::Meter, meter_gain),
                ActionEvent::Sound(SoundEffect::HangingKnifeFlick),
            ],
        )
        .end_at(if slow { 60 } else { 45 })
        .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
        .build()
}

fn sword_slam(version: SpecialVersion) -> Action {
    let (slow, high_damage, color) = match version {
        SpecialVersion::Strong => (true, true, STRONG_SWORD_VFX),
        SpecialVersion::Fast => (false, false, FAST_SWORD_VFX),
        SpecialVersion::Metered => (false, true, METERED_SWORD_VFX),
    };

    let mut builder = AttackBuilder::special()
        .with_character_universals(CHARACTER_UNIVERSALS)
        .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
        .with_extra_requirement(ActionRequirement::ItemOwned(ItemId::Fireaxe))
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
        .with_distance_on_block(0.1)
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
    let (long_lunge, slow, high_damage, color) = match version {
        SpecialVersion::Strong => (true, true, true, STRONG_SWORD_VFX),
        SpecialVersion::Fast => (false, false, false, FAST_SWORD_VFX),
        SpecialVersion::Metered => (false, false, true, METERED_SWORD_VFX),
    };

    AttackBuilder::special()
        .with_character_universals(CHARACTER_UNIVERSALS)
        .with_sound(SoundEffect::FemaleShagamu)
        .with_frame_data(if slow { 10 } else { 5 }, 2, if slow { 50 } else { 45 })
        .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
        .with_distance_on_block(0.1)
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
    let (slow, high_bounce, high_damage, color) = match version {
        SpecialVersion::Strong => (true, true, true, STRONG_SWORD_VFX),
        SpecialVersion::Fast => (false, false, false, FAST_SWORD_VFX),
        SpecialVersion::Metered => (false, false, true, METERED_SWORD_VFX),
    };

    AttackBuilder::special()
        .with_character_universals(CHARACTER_UNIVERSALS)
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
        .follow_up_from(vec![ActionId::Samurai(SamuraiAction::SwordStance(version))])
        .with_distance_on_block(0.1)
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
            let (input, base_velocity, hits, metered) = match version {
                SpecialVersion::Fast => ("{2}*6f", Vec2::new(4.0, 2.0), 1, false),
                SpecialVersion::Strong => ("{2}*6s", Vec2::new(0.9, 4.0), 2, false),
                SpecialVersion::Metered => ("{2}*6(fs)", Vec2::new(10.0, 1.0), 2, true),
            };

            let mut builder = ActionBuilder::special()
                .with_input(input)
                .with_animation(SamuraiAnimation::KunaiThrow)
                .with_sound(SoundEffect::FemaleKyatchi)
                .with_requirement(ActionRequirement::ResourceValue(
                    ResourceType::KunaiCounter,
                    1,
                ))
                .dyn_events_on_frame(
                    11,
                    Arc::new(move |situation: &Situation| {
                        let extra_stun = situation.inventory.contains(&ItemId::MiniTasers);

                        let stick_influence = if situation.inventory.contains(&ItemId::Protractor) {
                            situation
                                .facing
                                .mirror_vec2(situation.stick_position.as_vec2())
                                * 0.8
                        } else {
                            Vec2::ZERO
                        };

                        vec![
                            ActionEvent::ModifyResource(ResourceType::KunaiCounter, -1),
                            ActionEvent::SpawnHitbox(Attack {
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
                                on_hit: StrikeEffectBuilder::default()
                                    .with_height(Mid)
                                    .with_blockstun(if extra_stun { 20 } else { 15 })
                                    .with_damage(12)
                                    .with_defender_block_pushback(0.4)
                                    .with_chip_damage(2)
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
                                    .build(),
                            }),
                        ]
                    }),
                )
                .end_at(21);

            builder = if metered {
                builder.with_meter_cost()
            } else {
                builder.immediate_events(vec![ActionEvent::Condition(StatusCondition::kara_to(
                    vec![ActionId::Samurai(SamuraiAction::KunaiThrow(
                        SpecialVersion::Metered,
                    ))],
                ))])
            };

            builder.build()
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
                    ..default()
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
                    ..default()
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
                    ..default()
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
                    ..default()
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
                    ..default()
                },
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
