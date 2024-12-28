use bevy::{prelude::*, utils::HashMap};
use foundation::{ItemId, Stats, SELL_RETURN, STARTING_MONEY};

use crate::{Character, ConsumableType, Item, ItemCategory};

#[derive(Debug, Component, Eq, PartialEq, Reflect, Clone)]
pub struct Inventory {
    pub money: usize,
    #[reflect(ignore)]
    pub items: HashMap<ItemId, usize>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            money: STARTING_MONEY,
            items: HashMap::new(),
        }
    }
}
impl Inventory {
    pub fn count(&self, item: ItemId) -> usize {
        self.items.get(&item).cloned().unwrap_or_default()
    }

    pub fn contains(&self, item: ItemId) -> bool {
        self.count(item) > 0
    }

    pub fn can_buy(&self, id: ItemId, item: &Item) -> bool {
        if item.cost > self.money {
            return false;
        }

        if !self.has_space_for(id, item) {
            return false;
        }

        if let ItemCategory::Upgrade(dependencies) = &item.category {
            dependencies.iter().all(|dep| self.items.contains_key(dep))
        } else {
            true
        }
    }

    pub fn has_space_for(&self, id: ItemId, item: &Item) -> bool {
        self.count(id) <= item.max_stack
    }

    pub fn buy(&mut self, id: ItemId, item: Item) {
        self.money -= item.cost;
        let count = self.items.entry(id).or_insert(0);
        *count += 1;
    }

    pub fn sell(&mut self, character: &Character, id: ItemId) {
        // Remove done by id since inventory can contain duplicates
        let item = character.items.get(&id).unwrap();
        self.money += (item.cost as f32 * SELL_RETURN) as usize;
        let count = self.items.get_mut(&id).unwrap();
        if *count > 1 {
            *count -= 1;
        } else {
            self.items.remove(&id);
        }

        // When selling an item, also sell everything that depends on it
        for owned_id in self.items.to_owned().keys() {
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
                .filter_map(|(owned_id, count)| {
                    let owned_item = character.items.get(owned_id).unwrap();
                    if let ItemCategory::Upgrade(components) = &owned_item.category {
                        if components.contains(&id) {
                            return Some(count * self.sell_price(character, *owned_id));
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
        self.items
            .iter()
            .fold(Stats::default(), |accumulator, (id, count)| {
                accumulator.combine(&character.items[id].effect.multiply(*count))
            })
    }

    // Not a great name I'll admit
    pub fn remove_one_round_consumables(&mut self, character: &Character) {
        self.items.retain(|item, _| {
            !matches!(
                character.items.get(item).unwrap().category,
                ItemCategory::Consumable(ConsumableType::OneRound)
            )
        })
    }
}
