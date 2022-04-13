use moves::{MoveFlags, MoveId};

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
                new_moves: vec![MoveId::HandMeDownKen],
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
                new_moves: vec![MoveId::Gunshot, MoveId::Shoot],
                ..Default::default()
            },
        },
    ])
}
