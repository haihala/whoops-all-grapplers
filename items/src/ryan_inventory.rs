use bevy::prelude::*;

use moves::{
    equipment, CancelLevel, ConditionResolver, Move, MoveCost, MoveFlags, MoveStartCondition,
    Phase, PhaseKind,
};
use types::{Hitbox, Lifetime, SpawnDescriptor};

use crate::{Gi, Gun, Inventory, Item, ItemType, ShopItem};

pub fn ryan_inventory() -> Inventory {
    Inventory::new(vec![
        ShopItem {
            cost: 100,
            tier: 1,
            is_starter: false,
            item: Item {
                move_flag: Some(MoveFlags::DRUGS),
                ..Default::default()
            },
        },
        ShopItem {
            cost: 0,
            tier: 0,
            is_starter: true,
            item: Item {
                new_moves: vec![(
                    equipment::HANDMEDOWNKEN,
                    Move {
                        input: Some("236e"),
                        cancel_level: CancelLevel::LightSpecial,
                        conditions: MoveStartCondition::GROUND,
                        phases: vec![
                            Phase {
                                kind: PhaseKind::Animation,
                                duration: 30,
                                ..Default::default()
                            }
                            .into(),
                            Phase {
                                kind: PhaseKind::Attack(SpawnDescriptor {
                                    hitbox: Hitbox::new(Vec2::new(0.5, 0.5), Vec2::new(0.3, 0.2)),
                                    speed: Some(1.0 * Vec3::X),
                                    lifetime: Lifetime::Forever,
                                    ..Default::default()
                                }),
                                duration: 4,
                                ..Default::default()
                            }
                            .into(),
                            Phase {
                                kind: PhaseKind::Animation,
                                duration: 10,
                                cancellable: true,
                                ..Default::default()
                            }
                            .into(),
                        ],
                        ..Default::default()
                    },
                )],
                ..Default::default()
            },
        },
        ShopItem {
            cost: 100,
            tier: 2,
            is_starter: true,
            item: Item {
                item_type: Some(ItemType::Gi(Gi::default())),
                ..Default::default()
            },
        },
        ShopItem {
            cost: 100,
            tier: 2,
            is_starter: true,
            item: Item {
                item_type: Some(ItemType::Gun(Gun::default())),
                new_moves: vec![
                    (
                        equipment::GUNSHOT, // Single shot, the repeating bit
                        Move {
                            cancel_level: CancelLevel::LightNormal,
                            conditions: MoveStartCondition::GROUND,
                            cost: MoveCost {
                                // TODO bullets go here
                                ..Default::default()
                            },
                            phases: vec![
                                Phase {
                                    kind: PhaseKind::Animation,
                                    duration: 10,
                                    ..Default::default()
                                }
                                .into(),
                                Phase {
                                    duration: 20,
                                    kind: PhaseKind::Attack(SpawnDescriptor {
                                        hitbox: Hitbox::new(
                                            Vec2::new(0.5, 0.5),
                                            Vec2::new(0.3, 0.2),
                                        ),
                                        speed: Some(10.0 * Vec3::X),
                                        lifetime: Lifetime::Forever,
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                }
                                .into(),
                                ConditionResolver {
                                    default: Phase {
                                        kind: PhaseKind::Animation,
                                        duration: 30,
                                        ..Default::default()
                                    }
                                    .into(),
                                    branches: vec![(
                                        MoveFlags::EQUIPMENT_PRESSED,
                                        equipment::GUNSHOT.into(),
                                    )],
                                },
                            ],
                            ..Default::default()
                        },
                    ),
                    (
                        equipment::SHOOT,
                        Move {
                            input: Some("e"),
                            cancel_level: CancelLevel::LightNormal,
                            conditions: MoveStartCondition::GROUND,
                            phases: vec![
                                Phase {
                                    kind: PhaseKind::Animation,
                                    duration: 30,
                                    ..Default::default()
                                }
                                .into(),
                                equipment::GUNSHOT.into(),
                            ],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
        },
    ])
}
