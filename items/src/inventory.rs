use bevy::prelude::*;
use moves::PhaseCondition;

use crate::Item;

#[derive(Debug, Default, Component)]
pub struct Inventory {
    pub tier: usize,
    pub money: usize,
    pub owned: Vec<Item>,
    pub bought: Vec<Item>,
    pub shop_items: Vec<ShopItem>,
}
impl Inventory {
    pub fn new(shop_items: Vec<ShopItem>) -> Self {
        Self {
            shop_items,
            ..Default::default()
        }
    }

    pub fn roll_shop(&self, max_amount: usize) -> Vec<&ShopItem> {
        self.shop_items
            .iter()
            .filter(|si| !self.owned.contains(&si.item))
            .take(max_amount)
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

    pub fn phase_flags(&self) -> PhaseCondition {
        self.owned
            .iter()
            .filter_map(|item| item.move_flag)
            .reduce(|acc, new| acc | new)
            .unwrap_or(PhaseCondition::empty())
    }
}

#[derive(Debug)]
pub struct ShopItem {
    pub tier: usize,
    pub cost: usize,
    pub is_starter: bool,
    pub item: Item,
}
