use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::{Inventory, Item, ItemId, Move, MoveId};

#[derive(Debug, Default, Component, Clone)]
pub struct Kit {
    moves: HashMap<MoveId, Move>,
    items: HashMap<ItemId, Item>,
}
impl Kit {
    pub(crate) fn new(moves: Vec<(MoveId, Move)>, items: Vec<(ItemId, Item)>) -> Kit {
        Kit {
            moves: moves.into_iter().collect(),
            items: items.into_iter().collect(),
        }
    }

    pub fn get_move(&self, id: MoveId) -> Move {
        self.moves.get(&id).unwrap().to_owned()
    }

    pub fn get_inputs(&self) -> HashMap<MoveId, &'static str> {
        self.moves
            .iter()
            .filter_map(|(key, move_data)| move_data.input.map(|input| (*key, input)))
            .collect()
    }

    pub fn roll_items(&self, max_amount: usize, inventory: &Inventory) -> Vec<(ItemId, Item)> {
        self.items
            .iter()
            .filter(|(id, _)| !inventory.contains(id))
            .take(max_amount)
            .map(|(id, item)| (id.to_owned(), item.to_owned()))
            .collect()
        // TODO random selection that doesn't break rollback
    }
}
