use std::{collections::HashMap, iter::empty};

use bevy::prelude::*;
use map_macro::map;

use types::Area;

use crate::{
    moves::MoveType, AttackHeight, Branch, Cost, GrabDescription, Hitbox, Item, ItemId, Lifetime,
    Move, MoveId, MoveMobility, Phase, PhaseKind, Requirements, SpawnDescriptor,
};

use super::{
    dash,
    equipment::{get_gunshot, get_handmedownken, get_shot},
    jump, Character,
};

pub fn dummy() -> Character {
    Character::new(dummy_moves(), dummy_items())
}

// Dashing
const DASH_DURATION: usize = (0.5 * constants::FPS) as usize;
const DASH_IMPULSE: f32 = 10.0;

fn dummy_moves() -> HashMap<MoveId, Move> {
    empty()
        .chain(items().into_iter())
        .chain(movement().into_iter())
        .chain(attacks().into_iter())
        .collect()
}

fn items() -> HashMap<MoveId, Move> {
    map! {
        MoveId::HandMeDownKen => get_handmedownken(),
        MoveId::Gunshot => get_gunshot(),
        MoveId::Shoot => get_shot(),
    }
}

fn movement() -> HashMap<MoveId, Move> {
    map! {
            MoveId::BackJump => jump(
                "7",
                Vec2::new(-constants::DIAGONAL_JUMP_X, constants::DIAGONAL_JUMP_Y),
            ),
            MoveId::NeutralJump => jump("8", Vec2::Y * constants::NEUTRAL_JUMP_Y),
            MoveId::ForwardJump => jump(
                "9",
                Vec2::new(constants::DIAGONAL_JUMP_X, constants::DIAGONAL_JUMP_Y),
            ),
            MoveId::BackSuperJump => jump(
                "[123]7",
                Vec2::new(
                    -constants::DIAGONAL_SUPERJUMP_X,
                    constants::DIAGONAL_SUPERJUMP_Y,
                ),
            ),
            MoveId::NeutralSuperJump => jump("[123]8", Vec2::Y * constants::NEUTRAL_SUPERJUMP_Y),
            MoveId::ForwardSuperJump => jump(
                "[123]9",
                Vec2::new(
                    constants::DIAGONAL_SUPERJUMP_X,
                    constants::DIAGONAL_SUPERJUMP_Y,
                ),
            ),
            MoveId::DashForward => dash("656", DASH_DURATION, DASH_IMPULSE),
        MoveId::DashBack => dash("454", DASH_DURATION, -DASH_IMPULSE)
    }
}

fn attacks() -> HashMap<MoveId, Move> {
    map! {
        MoveId::Punch => Move {
            input: Some("f"),
            move_type: MoveType::Normal,
            requirements: Requirements {
                grounded: Some(true),
                ..default()
            },
            phases: vec![
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 5,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Attack(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 1.2, 0.3, 0.2)),
                        ..default()
                    }),
                    duration: 10,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 10,
                    cancellable: true,
                    ..default()
                }
                .into(),
            ],
        },
        MoveId::Low => Move {
            input: Some("[123]f"),
            move_type: MoveType::Normal,
            requirements: Requirements {
                grounded: Some(true),
                ..default()
            },
            phases: vec![
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 5,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Attack(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 0.2, 0.3, 0.2)),
                        ..default()
                    }),
                    duration: 10,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 10,
                    cancellable: true,
                    ..default()
                }
                .into(),
            ],
        },
        MoveId::CommandPunch => Move {
            input: Some("6f"),
            move_type: MoveType::Normal,
            requirements: Requirements {
                grounded: Some(true),
                ..default()
            },
            phases: vec![
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 10,
                    mobility: Some(MoveMobility::Perpetual(Vec3::new(1.0, 0.0, 0.0))),
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Attack(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 1.5, 0.5, 0.5)),
                        ..default()
                    }),
                    duration: 20,
                    mobility: Some(MoveMobility::Perpetual(Vec3::new(2.0, 0.0, 0.0))),
                    ..default()
                }
                .into(),
                Branch {
                    default: Phase {
                        kind: PhaseKind::Animation,
                        duration: 60,
                        ..default()
                    }
                    .into(),
                    branches: vec![(
                        Requirements::has_hit(),
                        Phase {
                            kind: PhaseKind::Animation,
                            duration: 1,
                            cancellable: true,
                            ..default()
                        }
                        .into(),
                    )],
                },
            ],
        },
        MoveId::BudgetBoom => Move {
            input: Some("[41]6f"),
            move_type: MoveType::Special,
            requirements: Requirements {
                grounded: Some(true),
                ..default()
            },
            phases: vec![
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 10,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Attack(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 1.2, 0.3, 0.2)),
                        speed: 5.0 * Vec3::X,
                        lifetime: Lifetime::Frames((constants::FPS * 0.25) as usize),
                        attached_to_player: false,
                        ..default()
                    }),
                    duration: 4,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 5,
                    cancellable: true,
                    ..default()
                }
                .into(),
            ],
        },
        MoveId::SonicBoom => Move {
            input: Some("[41]6f"),
            move_type: MoveType::Special,
            requirements: Requirements {
                cost: Some(Cost {
                    charge: true,
                    ..default()
                }),
                grounded: Some(true),
                ..default()
            },
            phases: vec![
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 10,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Attack(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 1.2, 0.4, 0.3)),
                        speed: 6.0 * Vec3::X,
                        lifetime: Lifetime::UntilHit,
                        damage: (20, 3).into(),
                        attached_to_player: false,
                        ..default()
                    }),
                    duration: 4,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 5,
                    cancellable: true,
                    ..default()
                }
                .into(),
            ],
        },
        MoveId::Hadouken => Move {
            input: Some("236f"),
            move_type: MoveType::Special,
            requirements: Requirements {
                grounded: Some(true),
                ..default()
            },
            phases: vec![
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 30,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Attack(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                        speed: 4.0 * Vec3::X,
                        lifetime: Lifetime::UntilHit,
                        attached_to_player: false,
                        ..default()
                    }),
                    duration: 4,
                    cancellable: true,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 30,
                    cancellable: true,
                    ..default()
                }
                .into(),
            ],
        },
        MoveId::HeavyHadouken => Move {
            input: Some("236s"),
            move_type: MoveType::Special,
            requirements: Requirements {
                cost: Some(Cost {
                    meter: 30,
                    ..default()
                }),
                ..default()
            },
            phases: vec![
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 30,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Attack(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 1.0, 0.4, 0.5)),
                        speed: 5.0 * Vec3::X,
                        lifetime: Lifetime::UntilHit,
                        hits: 2,
                        attached_to_player: false,
                        ..default()
                    }),
                    duration: 4,
                    cancellable: true,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 20,
                    cancellable: true,
                    ..default()
                }
                .into(),
            ],
        },
        MoveId::AirPunch => Move {
            input: Some("f"),
            move_type: MoveType::Normal,
            requirements: Requirements {
                grounded: Some(false),
                ..default()
            },
            phases: vec![
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 5,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Attack(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 0.1, 0.3, 0.5)),
                        fixed_height: Some(AttackHeight::High),
                        ..default()
                    }),
                    duration: 10,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 10,
                    cancellable: true,
                    ..default()
                }
                .into(),
            ],
        },
        MoveId::Grab => Move {
            input: Some("g"),
            move_type: MoveType::Normal,
            requirements: Requirements {
                grounded: Some(true),
                ..default()
            },
            phases: vec![
                Phase {
                    kind: PhaseKind::Animation,
                    duration: 5,
                    ..default()
                }
                .into(),
                Phase {
                    kind: PhaseKind::Grab(GrabDescription {
                        damage: 25,
                        ..default()
                    }),
                    duration: 40,
                    ..default()
                }
                .into(),
            ],
        },
    }
}

fn dummy_items() -> HashMap<ItemId, Item> {
    map!(
        ItemId::Drugs => Item {
            cost: 100,
            tier: 1,
            is_starter: false,
        },
        ItemId::HandMeDownKen => Item {
            cost: 0,
            tier: 0,
            is_starter: true,
        },
        ItemId::Gi => Item {
            cost: 100,
            tier: 2,
            is_starter: true,
        },
        ItemId::Gun => Item {
            cost: 100,
            tier: 2,
            is_starter: true,
        },
    )
}
