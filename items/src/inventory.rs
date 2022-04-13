use bevy::prelude::*;
use moves::{MoveFlags, MoveId};

#[derive(Debug, Default, Component)]
pub struct Inventory {
    pub tier: usize,
    pub money: usize,
    pub owned: Vec<Item>,
    pub bought: Vec<Item>,
    pub shop_items: Vec<Item>,
}
impl Inventory {
    pub fn new(shop_items: Vec<Item>) -> Self {
        Self {
            shop_items,
            ..Default::default()
        }
    }

    pub fn roll_shop(&self, max_amount: usize) -> Vec<Item> {
        self.shop_items
            .iter()
            .filter(|item| !self.owned.contains(item))
            .take(max_amount)
            .map(|item| item.to_owned())
            .collect()
        // TODO random selection that doesn't break rollback
    }

    pub fn buy(&mut self, item: Item) {
        self.owned.push(item.clone());
        self.bought.push(item);
    }

    pub fn drain_bought(&mut self) -> Vec<Item> {
        self.bought.drain(..).collect()
    }

    pub fn phase_flags(&self) -> MoveFlags {
        self.owned
            .iter()
            .filter_map(|item| item.move_flag)
            .reduce(|acc, new| acc | new)
            .unwrap_or(MoveFlags::empty())
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Item {
    pub tier: usize,
    pub cost: usize,
    pub is_starter: bool,
    pub move_flag: Option<MoveFlags>,
    pub new_moves: Vec<MoveId>,
}
