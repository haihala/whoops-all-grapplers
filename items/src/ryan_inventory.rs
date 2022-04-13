use moves::{MoveFlags, MoveId};

use crate::{Inventory, Item};

pub fn ryan_inventory() -> Inventory {
    Inventory::new(vec![
        Item {
            cost: 100,
            tier: 1,
            is_starter: false,
            move_flag: Some(MoveFlags::DRUGS),
            ..Default::default()
        },
        Item {
            cost: 0,
            tier: 0,
            is_starter: true,
            new_moves: vec![MoveId::HandMeDownKen],
            ..Default::default()
        },
        Item {
            cost: 100,
            tier: 2,
            is_starter: true,
            ..Default::default()
        },
        Item {
            cost: 100,
            tier: 2,
            is_starter: true,
            new_moves: vec![MoveId::Gunshot, MoveId::Shoot],
            ..Default::default()
        },
    ])
}
