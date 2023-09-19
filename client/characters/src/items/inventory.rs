use bevy::prelude::*;
use wag_core::{ItemId, Stats, INVENTORY_SIZE};

use crate::{Character, Item, ItemCategory};

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
            if filter_out(&self.items, dependencies).is_err() {
                return false;
            }
        }

        true
    }

    pub fn buy(&mut self, id: ItemId, item: Item) {
        self.money -= item.cost;

        // Remove dependencies from inventory
        if let ItemCategory::Upgrade(dependencies) = &item.category {
            self.items = filter_out(&self.items, dependencies).unwrap();
        }

        self.items.push(id);
    }

    pub fn sell(&mut self, index: usize, refund: usize) {
        // Remove done by id since inventory can contain duplicates
        self.money += refund;
        self.items.remove(index);
    }

    pub fn get_effects(&self, character: &Character) -> Stats {
        self.items.iter().fold(Stats::default(), |accumulator, id| {
            accumulator.combine(&get_recursive_effects(id, character))
        })
    }

    pub fn count(&self, item: ItemId) -> usize {
        self.items
            .iter()
            .filter(|owned_item| owned_item == &&item)
            .count()
    }

    pub fn consume(&mut self, item: ItemId) {
        if let Some(index) = self.items.iter().position(|owned_item| owned_item == &item) {
            self.items.remove(index);
        } else {
            panic!("Item not found in inventory");
        }
    }
}

fn get_recursive_effects(item_id: &ItemId, character: &Character) -> Stats {
    let item = character.items.get(item_id).unwrap();

    if let ItemCategory::Upgrade(dependencies) = &item.category {
        dependencies.iter().fold(item.effect, |accumulator, item| {
            accumulator.combine(&get_recursive_effects(item, character))
        })
    } else {
        item.effect
    }
}

// Could move elsewhere if need be. Written this way to handle duplicates.
fn filter_out<T: PartialEq + Clone>(container: &[T], to_remove: &[T]) -> Result<Vec<T>, ()> {
    let mut temp = container.to_owned();

    for dependency in to_remove {
        if let Some(index) = temp.iter().position(|owned_item| owned_item == dependency) {
            temp.remove(index);
        } else {
            return Err(());
        }
    }

    Ok(temp)
}
