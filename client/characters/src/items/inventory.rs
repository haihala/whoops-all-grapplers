use bevy::{prelude::*, utils::HashSet};
use wag_core::{ItemId, Stats, SELL_RETURN, STARTING_MONEY};

use crate::{Character, ConsumableType, Item, ItemCategory};

#[derive(Debug, Component, Eq, PartialEq, Reflect, Clone)]
pub struct Inventory {
    pub money: usize,
    #[reflect(ignore)]
    pub items: HashSet<ItemId>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            money: STARTING_MONEY,
            items: HashSet::new(),
        }
    }
}
impl Inventory {
    pub fn contains(&self, item: &ItemId) -> bool {
        self.items.contains(item)
    }

    pub fn can_buy(&self, id: ItemId, item: &Item) -> bool {
        if item.cost > self.money {
            return false;
        }

        if self.contains(&id) {
            return false;
        }

        if let ItemCategory::Upgrade(dependencies) = &item.category {
            dependencies.iter().all(|dep| self.items.contains(dep))
        } else {
            true
        }
    }

    pub fn buy(&mut self, id: ItemId, item: Item) {
        self.money -= item.cost;
        self.items.insert(id);
    }

    pub fn sell(&mut self, character: &Character, id: ItemId) {
        // Remove done by id since inventory can contain duplicates
        let item = character.items.get(&id).unwrap();
        self.money += (item.cost as f32 * SELL_RETURN) as usize;
        self.items.remove(&id);

        // When selling an item, also sell everything that depends on it
        for owned_id in self.items.to_owned().iter() {
            let owned_item = character.items.get(owned_id).unwrap();
            if let ItemCategory::Upgrade(components) = &owned_item.category {
                if components.contains(&id) {
                    self.sell(character, *owned_id);
                }
            }
        }
    }

    pub fn sell_price(&self, character: &Character, id: ItemId) -> usize {
        let item = character.items.get(&id).unwrap();

        item.cost
            + self
                .items
                .to_owned()
                .iter()
                .filter_map(|owned_id| {
                    let owned_item = character.items.get(owned_id).unwrap();
                    if let ItemCategory::Upgrade(components) = &owned_item.category {
                        if components.contains(&id) {
                            return Some(owned_item.cost);
                        }
                    }
                    None
                })
                .sum::<usize>()
    }

    pub fn remove(&mut self, id: ItemId) {
        self.items.remove(&id);
    }

    pub fn get_effects(&self, character: &Character) -> Stats {
        self.items.iter().fold(Stats::default(), |accumulator, id| {
            accumulator.combine(&character.items[id].effect)
        })
    }

    pub fn count(&self, item: ItemId) -> usize {
        self.items
            .iter()
            .filter(|owned_item| owned_item == &&item)
            .count()
    }

    // Not a great name I'll admit
    pub fn remove_one_round_consumables(&mut self, character: &Character) {
        self.items.retain(|item| {
            !matches!(
                character.items.get(item).unwrap().category,
                ItemCategory::Consumable(ConsumableType::OneRound)
            )
        })
    }
}
