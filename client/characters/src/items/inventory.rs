use std::collections::HashSet;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use types::ItemId;

#[derive(Debug, Default, Component, Eq, PartialEq, Inspectable, Clone)]
pub struct Inventory {
    pub tier: usize,
    pub money: usize,
    #[inspectable(ignore)]
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