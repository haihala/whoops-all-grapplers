use bevy::prelude::*;
use bevy::utils::HashMap;
use types::{Area, StickPosition};

use crate::{Inventory, Item, ItemId, Move, MoveId};

#[derive(Debug, Component, Clone)]
pub struct Kit {
    moves: HashMap<MoveId, Move>,
    items: HashMap<ItemId, Item>,
    pub idle_animation: &'static str,
    pub low_block_height: f32,
    pub high_block_height: f32,
    pub standing_hurtbox: Area,
    pub crouching_hurtbox: Area,
    pub standing_pushbox: Area,
    pub crouching_pushbox: Area,
    pub charge_directions: Vec<StickPosition>,
}

impl Default for Kit {
    fn default() -> Self {
        Self {
            moves: Default::default(),
            items: Default::default(),
            idle_animation: Default::default(),
            low_block_height: 0.5,
            high_block_height: 1.2,
            charge_directions: vec![
                StickPosition::SE,
                StickPosition::S,
                StickPosition::SW,
                StickPosition::W,
            ],
            // TODO: Make theses a part of the constructor:
            standing_hurtbox: Area::from_center_size(Vec2::Y * 0.9, Vec2::new(0.5, 1.8)),
            crouching_hurtbox: Area::from_center_size(Vec2::Y * 0.6, Vec2::new(0.5, 1.2)),
            standing_pushbox: Area::from_center_size(Vec2::Y * 0.9, Vec2::new(0.5, 1.8)),
            crouching_pushbox: Area::from_center_size(Vec2::Y * 0.6, Vec2::new(0.5, 1.2)),
        }
    }
}
impl Kit {
    pub(crate) fn new(moves: Vec<(MoveId, Move)>, items: Vec<(ItemId, Item)>) -> Kit {
        Kit {
            moves: moves.into_iter().collect(),
            items: items.into_iter().collect(),
            idle_animation: "dummy-character.glb#Animation0",
            ..default()
        }
    }

    pub fn get_animations(&self) -> Vec<&'static str> {
        vec![self.idle_animation]
    }

    pub fn get_move(&self, id: MoveId) -> Move {
        self.moves.get(&id).unwrap().to_owned()
    }

    pub fn get_pushbox(&self, crouching: bool) -> Area {
        if crouching {
            self.crouching_pushbox
        } else {
            self.standing_pushbox
        }
    }

    pub fn get_hurtbox(&self, crouching: bool) -> Area {
        if crouching {
            self.crouching_hurtbox
        } else {
            self.standing_hurtbox
        }
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
