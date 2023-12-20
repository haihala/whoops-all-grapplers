use std::collections::HashMap;

use bevy::prelude::*;
use wag_core::{ActionId, Animation, AnimationType, Area, ItemId, Model, Stats};

use crate::{resources::ResourceType, Action, Item, WAGResource};

#[derive(Debug, Component, Clone)]
pub struct Character {
    moves: HashMap<ActionId, Action>,
    pub items: HashMap<ItemId, Item>,
    pub model: Model,
    pub standing_pushbox: Area,
    pub crouching_pushbox: Area,
    pub air_pushbox: Area,
    pub generic_animations: HashMap<AnimationType, Animation>,
    pub base_stats: Stats,
    pub special_properties: Vec<(ResourceType, WAGResource)>,
}
impl Character {
    // TODO: Consider making a builder for this
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        model: Model,
        generic_animations: HashMap<AnimationType, Animation>,
        moves: HashMap<ActionId, Action>,
        items: HashMap<ItemId, Item>,
        base_stats: Stats,
        special_properties: Vec<(ResourceType, WAGResource)>,
    ) -> Character {
        Self {
            model,
            generic_animations,
            moves,
            items,
            standing_pushbox: Area::from_center_size(Vec2::Y * 0.7, Vec2::new(0.4, 1.4)),
            crouching_pushbox: Area::from_center_size(Vec2::new(0.2, 0.35), Vec2::new(0.6, 0.7)),
            air_pushbox: Area::from_center_size(Vec2::new(0.0, 0.55), Vec2::new(0.4, 0.6)),
            special_properties,
            base_stats,
        }
    }

    pub fn get_move(&self, id: ActionId) -> Option<Action> {
        self.moves.get(&id).map(|opt| opt.to_owned())
    }

    pub fn get_inputs(&self) -> HashMap<ActionId, &'static str> {
        self.moves
            .iter()
            .filter_map(|(key, move_data)| move_data.input.map(|input| (*key, input)))
            .collect()
    }
}
