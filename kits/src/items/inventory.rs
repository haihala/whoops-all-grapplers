use super::ItemId;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Default, Component, PartialEq, Inspectable, Clone)]
pub struct Inventory {
    pub tier: usize,
    pub money: usize,
    pub owned: Vec<ItemId>,
    pub bought: Vec<ItemId>,
}
impl Inventory {
    pub fn contains(&self, item: &ItemId) -> bool {
        self.owned.contains(item)
    }

    pub fn buy(&mut self, item: ItemId) {
        self.bought.push(item);
    }

    pub fn finish_shopping(&mut self) -> Vec<ItemId> {
        let new_items: Vec<ItemId> = self.bought.drain(..).collect();
        for item in new_items.iter() {
            self.owned.push(item.to_owned());
        }
        new_items
    }
}
