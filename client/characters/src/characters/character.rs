use bevy::{prelude::*, utils::HashMap};
use wag_core::{ActionId, Animation, AnimationType, ItemId, Model, Player, Stats};

use crate::{resources::ResourceType, Action, CharacterBoxes, Item, WAGResource};

#[derive(Debug, Component, Clone)]
pub struct Character {
    pub(crate) moves: HashMap<ActionId, Action>,
    pub colors: HashMap<Player, HashMap<&'static str, Color>>,
    pub items: HashMap<ItemId, Item>,
    pub model: Model,
    pub boxes: CharacterBoxes,
    pub generic_animations: HashMap<AnimationType, Animation>,
    pub base_stats: Stats,
    pub special_properties: Vec<(ResourceType, WAGResource)>,
}
impl Character {
    // TODO: Consider making a builder for this
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        model: Model,
        p2_colors: HashMap<&'static str, Color>,
        generic_animations: HashMap<AnimationType, Animation>,
        moves: HashMap<ActionId, Action>,
        items: HashMap<ItemId, Item>,
        boxes: CharacterBoxes,
        base_stats: Stats,
        special_properties: Vec<(ResourceType, WAGResource)>,
    ) -> Character {
        Self {
            model,
            colors: vec![(Player::One, HashMap::new()), (Player::Two, p2_colors)]
                .into_iter()
                .collect(),
            generic_animations,
            moves,
            items,
            special_properties,
            boxes,
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

#[cfg(test)]
mod test {
    use crate::{characters::mizku, dummy, ActionEvent, ActionTracker, Situation};

    use super::*;

    #[test]
    fn all_moves_end() {
        for char in [mizku(), dummy()] {
            for (id, mov) in char.moves.iter() {
                let sit = Situation {
                    tracker: Some(ActionTracker {
                        start_frame: 0,
                        ..default()
                    }),
                    frame: 9999,
                    ..default()
                };
                let end_events = (mov.script)(&sit);
                assert!(
                    end_events.contains(&ActionEvent::End),
                    "{:?} - {:?} not in {:?}",
                    id,
                    &mov,
                    end_events,
                );
            }
        }
    }
}
