use bevy::{prelude::*, utils::HashMap};

use wag_core::{
    ActionCategory, ActionId, Animation, AnimationType, Area, GameButton, Icon, ItemId,
    MizkuActionId, MizkuAnimation, Model, Stats, MIZUKI_ALT_HELMET_COLOR, MIZUKI_ALT_JEANS_COLOR,
    MIZUKI_ALT_SHIRT_COLOR,
};

use crate::{
    actions::ActionRequirement,
    dashes, jumps,
    resources::{RenderInstructions, ResourceType},
    throw_hit, throw_target, universal_item_actions, Action, ActionEvent, Attack, AttackBuilder,
    AttackHeight::*,
    BlockType::*,
    CharacterBoxes, CharacterStateBoxes, ConsumableType, CounterVisual, Hitbox, IntermediateStrike,
    Item, ItemCategory, Lifetime, Movement, Situation, ToHit, WAGResource,
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
        mizku_boxes(),
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
            AttackBuilder::normal("f")
                .with_animation(MizkuAnimation::KneeThrust)
                .with_frame_data(5, 2, 16)
                .with_hitbox(Area::new(0.5, 1.0, 0.35, 0.35))
                .with_damage(5)
                .with_advantage_on_block(-1)
                .with_advantage_on_hit(4)
                .build(),
        ),
        (
            MizkuActionId::LowKick,
            AttackBuilder::normal("[123]+f")
                .hits_low()
                .with_animation(MizkuAnimation::LowKick)
                .with_frame_data(3, 3, 12)
                .with_hitbox(Area::new(0.4, 0.1, 0.9, 0.2))
                .with_damage(8)
                .with_advantage_on_block(-1)
                .with_advantage_on_hit(6)
                .build(),
        ),
        (
            MizkuActionId::HeelKick,
            AttackBuilder::normal("s")
                .with_animation(MizkuAnimation::HeelKick)
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
            MizkuActionId::Uppercut,
            AttackBuilder::normal("[123]+s")
                .with_animation(MizkuAnimation::Uppercut)
                .with_frame_data(8, 8, 40)
                .with_hitbox(Area::new(0.3, 0.7, 0.3, 0.5))
                .with_damage(16)
                .with_distance_on_block(0.5)
                .launches(Vec2::new(1.0, 6.0))
                .with_advantage_on_block(-30)
                .build(),
        ),
        (
            MizkuActionId::HighStab,
            AttackBuilder::normal("g")
                .with_animation(MizkuAnimation::HighStab)
                .with_frame_data(7, 6, 46)
                .with_hitbox(Area::new(1.5, 1.3, 1.8, 0.2))
                .with_damage(10)
                .sword()
                .with_advantage_on_block(-16)
                .with_advantage_on_hit(-6)
                .build(),
        ),
        (
            MizkuActionId::SkySlash,
            AttackBuilder::normal("[123]+g")
                .with_animation(MizkuAnimation::SkyStab)
                .with_frame_data(8, 5, 32)
                .with_hitbox(Area::new(1.8, 0.9, 1.0, 1.0))
                .with_damage(8)
                .sword()
                .with_advantage_on_block(-7)
                .with_advantage_on_hit(10)
                .build(),
        ),
        (
            MizkuActionId::AirSlice,
            AttackBuilder::normal("g")
                .air_only()
                .with_animation(MizkuAnimation::AirStab)
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
            MizkuActionId::FalconKnee,
            AttackBuilder::normal("f")
                .air_only()
                .with_animation(MizkuAnimation::FalconKnee)
                .with_frame_data(2, 5, 23)
                .with_hitbox(Area::new(0.3, 0.2, 0.35, 0.25))
                .with_damage(5)
                // TODO: These are misleading due to landing cancels
                .with_advantage_on_block(-20)
                .with_advantage_on_hit(-10)
                .build(),
        ),
        (
            MizkuActionId::FootDive,
            Action {
                input: Some("s"),
                category: ActionCategory::Normal,
                script: Box::new(|situation: &Situation| {
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
                            MizkuAnimation::FootDiveRelease.into(),
                            // TODO: There used to be a 3f delay after the animation, but new
                            // system makes that hard, maybe think of a way to reintroduce that.
                            Attack {
                                to_hit: ToHit {
                                    hitbox: Hitbox(Area::new(0.8, -0.2, 0.7, 0.3)),
                                    lifetime: Lifetime::frames(7),
                                    block_type: Strike(High),
                                    ..default()
                                },
                                ..IntermediateStrike {
                                    base_damage: 18,
                                    attacker_push_on_block: 0.33,
                                    defender_push_on_block: 0.66,
                                    attacker_push_on_hit: 0.2,
                                    hit_stun_event: if situation
                                        .inventory
                                        .contains(&ItemId::SpaceSuitBoots)
                                    {
                                        ActionEvent::LaunchStun(Vec2::new(-1.0, 15.0))
                                    } else {
                                        ActionEvent::HitStun(40)
                                    },
                                    block_stun: 25,
                                    ..default()
                                }
                                .build_attack(situation)
                            }
                            .into(),
                        ];
                    }

                    vec![]
                }),
                requirements: vec![ActionRequirement::Airborne],
            },
        ),
        (
            MizkuActionId::ForwardThrow,
            AttackBuilder::normal("w")
                .forward_throw()
                .throw_hit_action(MizkuActionId::StandThrowHit)
                .throw_target_action(MizkuActionId::StandThrowTarget)
                .with_frame_data(3, 3, 34)
                .with_animation(MizkuAnimation::StandThrowStartup)
                .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5))
                .build(),
        ),
        (
            MizkuActionId::BackThrow,
            AttackBuilder::normal("4+w")
                .back_throw()
                .throw_hit_action(MizkuActionId::StandThrowHit)
                .throw_target_action(MizkuActionId::StandThrowTarget)
                .with_frame_data(3, 3, 34)
                .with_animation(MizkuAnimation::StandThrowStartup)
                .with_hitbox(Area::new(0.5, 1.0, 0.5, 0.5))
                .build(),
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
            AttackBuilder::normal("[123]+w")
                .forward_throw()
                .throw_hit_action(MizkuActionId::CrouchThrowHit)
                .throw_target_action(MizkuActionId::CrouchThrowTarget)
                .with_frame_data(5, 3, 55)
                .with_animation(MizkuAnimation::CrouchThrowStartup)
                .with_hitbox(Area::new(0.7, 0.1, 0.5, 0.2))
                .build(),
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
            AttackBuilder::normal("w")
                .forward_throw()
                .air_only()
                .throw_hit_action(MizkuActionId::AirThrowHit)
                .throw_target_action(MizkuActionId::AirThrowTarget)
                .with_frame_data(4, 2, 36)
                .with_animation(MizkuAnimation::AirThrowStartup)
                .with_hitbox(Area::new(0.4, 0.5, 0.8, 0.8))
                .build(),
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
        use wag_core::{ActionId, CancelType, CancelWindow, StatusCondition, StatusFlag};
        use $crate::FlashRequest;

        Action {
            input: Some(if $strong { "214s" } else { "214f" }),
            category: ActionCategory::Special,
            script: Box::new(|situation: &Situation| {
                let mut events = vec![MizkuAnimation::SwordStance.into()];

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
                    return vec![ActionEvent::AllowCancel(CancelWindow {
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
                    })];
                }

                situation.end_at(38)
            }),
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
        script: Box::new(|situation: &Situation| {
            if situation.elapsed() == 0 {
                return vec![MizkuAnimation::Sharpen.into()];
            }

            if situation.elapsed() >= 43 {
                return vec![
                    ActionEvent::ModifyResource(ResourceType::Sharpness, 1),
                    ActionEvent::End,
                ];
            }

            vec![]
        }),
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
    AttackBuilder::special("s")
        .follow_up_from(vec![
            ActionId::Mizku(MizkuActionId::FSwordStance),
            ActionId::Mizku(MizkuActionId::SSwordStance),
        ])
        .with_frame_data(8, 6, 64)
        .with_animation(MizkuAnimation::ViperStrike)
        .with_extra_initial_events(vec![Movement {
            amount: Vec2::X * 8.0,
            duration: 7,
        }
        .into()])
        .with_hitbox(Area::new(1.2, 0.225, 1.6, 0.45))
        .hits_low()
        .with_damage(20)
        .sword()
        .with_advantage_on_hit(-10)
        .with_advantage_on_block(-40)
        .build()
}

fn rising_sun() -> Action {
    AttackBuilder::special("f")
        .follow_up_from(vec![
            ActionId::Mizku(MizkuActionId::FSwordStance),
            ActionId::Mizku(MizkuActionId::SSwordStance),
        ])
        .with_frame_data(3, 8, 74)
        .sword()
        .with_damage(20)
        .launches(Vec2::new(1.0, 3.0))
        .with_advantage_on_block(-30)
        .with_hitbox(Area::new(0.2, 1.5, 3.0, 1.5))
        .build()
}

fn kunai_throw() -> impl Iterator<Item = (MizkuActionId, Action)> {
    vec![(
        MizkuActionId::KunaiThrow,
        AttackBuilder::special("236f")
            .with_extra_initial_events(vec![
                ActionEvent::ForceStand,
                ActionEvent::ModifyResource(ResourceType::KunaiCounter, -1),
            ])
            .with_extra_requirements(vec![ActionRequirement::ResourceValue(
                ResourceType::KunaiCounter,
                1,
            )])
            // TODO: This is a clunky way to do active frames for projectiles
            .with_timings(13, 10)
            .with_hitbox(Area::new(1.0, 1.2, 0.3, 0.3))
            .with_projectile(Model::Kunai, Vec2::new(6.0, -0.4))
            // TODO: Misleading for projectiles,
            // this only applies if it hits on first frame
            .with_advantage_on_block(1)
            .with_advantage_on_hit(0)
            .build(),
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

fn mizku_boxes() -> CharacterBoxes {
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
