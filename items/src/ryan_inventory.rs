use bevy::prelude::*;

use moves::{equipment, CancelLevel, Move, MoveCondition, Phase, PhaseCondition, PhaseKind};
use types::{Hitbox, Lifetime, SpawnDescriptor};

use crate::{Gi, Gun, Inventory, Item, ItemType, ShopItem};

pub fn ryan_inventory() -> Inventory {
    Inventory::new(vec![
        ShopItem {
            cost: 100,
            tier: 1,
            is_starter: false,
            item: Item {
                move_flag: Some(PhaseCondition::DRUGS),
                new_moves: vec![],
                item_type: ItemType::Drugs,
            },
        },
        ShopItem {
            cost: 0,
            tier: 0,
            is_starter: true,
            item: Item {
                move_flag: None,
                new_moves: vec![(
                    equipment::HANDMEDOWNKEN,
                    Move {
                        input: "236e",
                        cancel_level: CancelLevel::LightSpecial,
                        conditions: MoveCondition::GROUND,
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
                item_type: ItemType::Handmedownken,
            },
        },
        ShopItem {
            cost: 100,
            tier: 2,
            is_starter: true,
            item: Item {
                move_flag: None,
                new_moves: vec![],
                item_type: ItemType::Gi(Gi::default()),
            },
        },
        ShopItem {
            cost: 100,
            tier: 2,
            is_starter: true,
            item: Item {
                move_flag: None,
                new_moves: vec![], // TODO Gunshot is a move?
                item_type: ItemType::Gun(Gun::default()),
            },
        },
    ])
}
