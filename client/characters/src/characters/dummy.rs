use std::{collections::HashMap, iter::empty};

use bevy::prelude::*;
use map_macro::map;

use types::{Area, ItemId, MoveId};

use crate::{
    moves::{Action, FlowControl, MoveType, Situation},
    AttackHeight, Cost, GrabDescription, Hitbox, Item, Lifetime, Move, SpawnDescriptor,
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
            can_start: |situation: Situation| situation.grounded,
            phases: vec![
                FlowControl::Wait(5, false).into(),
                Action::Hitbox(SpawnDescriptor {
                    hitbox: Hitbox(Area::new(0.5, 1.2, 0.3, 0.2)),
                    ..default()
                }).into(),
                FlowControl::Wait(15, true).into(),
            ],
        },
        MoveId::Low => Move {
            input: Some("[123]f"),
            move_type: MoveType::Normal,
            can_start: |situation: Situation| situation.grounded,
            phases: vec![
                FlowControl::Wait(5, false).into(),
                Action::Hitbox(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 0.2, 0.3, 0.2)),
                        ..default()
                    }).into(),
                FlowControl::Wait(15, true).into(),
            ],
        },
        MoveId::CommandPunch => Move {
            input: Some("6f"),
            move_type: MoveType::Normal,
            can_start: |situation: Situation| situation.grounded,
            phases: vec![
                Action::Perpetual(Vec2::X*1.0, 10).into(),
                FlowControl::Wait(10, false).into(),
                Action::Hitbox(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 1.5, 0.5, 0.5)),
                        ..default()
                    }).into(),
                Action::Perpetual(Vec2::X*2.0, 10).into(),
                FlowControl::Wait(20, false),
                FlowControl::Dynamic(|situation: Situation| {
                    if situation.history.unwrap().has_hit {
                        FlowControl::Wait(10, true)
                    } else {
                        FlowControl::Wait(60, false)
                    }
                })
            ],
        },
        MoveId::BudgetBoom => Move {
            input: Some("[41]6f"),
            move_type: MoveType::Special,
            can_start: |situation: Situation| situation.grounded,
            phases: vec![
                FlowControl::Wait(10, false).into(),
                Action::Hitbox(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 1.2, 0.3, 0.2)),
                        speed: 5.0 * Vec3::X,
                        lifetime: Lifetime::Frames((constants::FPS * 0.25) as usize),
                        attached_to_player: false,
                        ..default()
                    }).into(),
                FlowControl::Wait(5, true).into(),
            ],
        },
        MoveId::SonicBoom => Move {
            input: Some("[41]6f"),
            move_type: MoveType::Special,
            can_start: |situation: Situation| {
                situation.grounded && situation.resources.can_afford(&Some(Cost{ charge: true, ..default()}))
            },
            phases: vec![
                Action::Pay(Cost{charge: true, ..default()}).into(),
                FlowControl::Wait(10, false).into(),
                Action::Hitbox(SpawnDescriptor {
                    hitbox: Hitbox(Area::new(0.5, 1.2, 0.4, 0.3)),
                    speed: 6.0 * Vec3::X,
                    lifetime: Lifetime::UntilHit,
                    damage: (20, 3).into(),
                    attached_to_player: false,
                    ..default()
                }).into(),
                FlowControl::Wait(5, true).into(),
            ],
        },
        MoveId::Hadouken => Move {
            input: Some("236f"),
            move_type: MoveType::Special,
            can_start: |situation: Situation| situation.grounded,
            phases: vec![
                FlowControl::Wait(30, false).into(),
                Action::Hitbox(SpawnDescriptor {
                    hitbox: Hitbox(Area::new(0.5, 1.0, 0.3, 0.3)),
                    speed: 4.0 * Vec3::X,
                    lifetime: Lifetime::UntilHit,
                    attached_to_player: false,
                    ..default()
                }).into(),
                FlowControl::Wait(30, true).into(),
            ],
        },
        MoveId::HeavyHadouken => Move {
            input: Some("236s"),
            move_type: MoveType::Special,
            can_start: |situation: Situation| {
                situation.resources.can_afford(&Some(Cost{ meter: 30, ..default()}))
            },
            phases: vec![
                Action::Pay(Cost{meter: 30, ..default()}).into(),
                FlowControl::Wait(30, false).into(),
                Action::Hitbox(SpawnDescriptor {
                    hitbox: Hitbox(Area::new(0.5, 1.0, 0.4, 0.5)),
                    speed: 5.0 * Vec3::X,
                    lifetime: Lifetime::UntilHit,
                    hits: 2,
                    attached_to_player: false,
                    ..default()
                }).into(),
                FlowControl::Wait(20, false).into(),
            ],
        },
        MoveId::AirPunch => Move {
            input: Some("f"),
            move_type: MoveType::Normal,
            can_start: |situation: Situation| !situation.grounded,
            phases: vec![
                FlowControl::Wait(5, false).into(),
                Action::Hitbox(SpawnDescriptor {
                        hitbox: Hitbox(Area::new(0.5, 0.1, 0.3, 0.5)),
                        fixed_height: Some(AttackHeight::High),
                        ..default()
                    }).into(),
                FlowControl::Wait(10, true).into(),
            ],
        },
        MoveId::Grab => Move {
            input: Some("g"),
            move_type: MoveType::Normal,
            can_start: |situation: Situation| situation.grounded,
            phases: vec![
                FlowControl::Wait(5, false).into(),
                Action::Grab(GrabDescription {
                    damage: 25,
                    ..default()
                }).into(),
                FlowControl::Wait(40, true).into(),
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
