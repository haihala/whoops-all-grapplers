use bevy::prelude::*;
use wag_core::{ItemId, INVENTORY_SIZE};

use crate::{Item, ItemCategory};

#[derive(Debug, Component, Eq, PartialEq, Reflect, Clone)]
pub struct Inventory {
    pub money: usize,
    #[reflect(ignore)]
    pub items: Vec<ItemId>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            money: 250,
            items: Vec::with_capacity(INVENTORY_SIZE),
        }
    }
}
impl Inventory {
    pub fn contains(&self, item: &ItemId) -> bool {
        self.items.contains(item)
    }

    pub fn can_buy(&self, item: &Item) -> bool {
        if item.cost > self.money {
            return false;
        }

        if self.items.len() >= INVENTORY_SIZE {
            return false;
        }

        if let ItemCategory::Upgrade(dependencies) = &item.category {
            if !dependencies
                .iter()
                .all(|dependency| self.items.contains(dependency))
            {
                return false;
            }
        }

        true
    }

    pub fn buy(&mut self, id: ItemId, item: Item) {
        self.money -= item.cost;
        self.items.push(id);
    }

    pub fn sell(&mut self, index: usize, refund: usize) {
        // Remove done by id since inventory can contain duplicates
        self.money += refund;
        self.items.remove(index);
    }
}
