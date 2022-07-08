use super::ItemId;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Default, Component, Eq, PartialEq, Inspectable, Clone)]
pub struct Inventory {
    pub tier: usize,
    pub money: usize,
    pub owned: Vec<ItemId>,
}
impl Inventory {
    pub fn contains(&self, item: &ItemId) -> bool {
        self.owned.contains(item)
    }

    pub fn add_item(&mut self, item: ItemId) {
        self.owned.push(item);
    }
}
