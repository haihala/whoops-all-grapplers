use std::collections::HashSet;

use bevy::prelude::*;
use wag_core::ItemId;

#[derive(Debug, Default, Component, Eq, PartialEq, Reflect, Clone)]
pub struct Inventory {
    pub tier: usize,
    pub money: usize,
    #[reflect(ignore)]
    pub items: HashSet<ItemId>,
}
impl Inventory {
    pub fn contains(&self, item: &ItemId) -> bool {
        self.items.contains(item)
    }

    pub fn add_item(&mut self, item: ItemId) {
        self.items.insert(item);
    }
}
