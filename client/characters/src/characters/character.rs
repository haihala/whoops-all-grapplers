use bevy::{prelude::*, utils::HashMap};
use foundation::{
    ActionId, Animation, AnimationType, CharacterId, ItemId, Model, Player, Sound, Stats, VoiceLine,
};

use crate::{resources::GaugeType, Action, CharacterBoxes, Gauge, Item};

use super::samurai;

#[derive(Debug, Component)]
pub struct Character {
    pub(crate) moves: HashMap<ActionId, Action>,
    pub(crate) voicelines: HashMap<VoiceLine, Sound>,
    pub theme_song: Sound,
    pub colors: HashMap<Player, HashMap<&'static str, Color>>,
    pub items: HashMap<ItemId, Item>,
    pub model: Model,
    pub boxes: CharacterBoxes,
    pub generic_animations: HashMap<AnimationType, Animation>,
    pub base_stats: Stats,
    pub special_properties: Vec<(GaugeType, Gauge)>,
}
impl Character {
    // TODO: Consider making a builder for this
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        model: Model,
        theme_song: Sound,
        p2_colors: HashMap<&'static str, Color>,
        generic_animations: HashMap<AnimationType, Animation>,
        moves: HashMap<ActionId, Action>,
        items: HashMap<ItemId, Item>,
        boxes: CharacterBoxes,
        base_stats: Stats,
        special_properties: Vec<(GaugeType, Gauge)>,
        voicelines: HashMap<VoiceLine, Sound>,
    ) -> Character {
        debug_assert_eq!(boxes.standing.pushbox.bottom(), 0.0);

        Self {
            model,
            theme_song,
            colors: vec![(Player::One, HashMap::new()), (Player::Two, p2_colors)]
                .into_iter()
                .collect(),
            generic_animations,
            moves,
            items,
            special_properties,
            boxes,
            base_stats,
            voicelines,
        }
    }

    pub fn get_move(&self, id: ActionId) -> Option<&Action> {
        self.moves.get(&id)
    }

    pub fn get_inputs(&self) -> HashMap<ActionId, String> {
        self.moves
            .iter()
            .filter_map(|(key, move_data)| move_data.input.clone().map(|input| (*key, input)))
            .collect()
    }

    pub fn get_voiceline(&self, line: VoiceLine) -> Sound {
        *self.voicelines.get(&line).unwrap_or(&Sound::Silence)
    }
}

impl From<CharacterId> for Character {
    fn from(value: CharacterId) -> Self {
        match value {
            CharacterId::Samurai => samurai(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{characters::samurai, ActionEvent, ActionRequirement, ActionTracker, Situation};

    use super::*;

    #[test]
    fn all_moves_end() {
        for char in [samurai()] {
            for (id, mov) in char.moves.iter() {
                if mov.transient {
                    continue;
                }

                let sit = Situation {
                    tracker: Some(ActionTracker {
                        start_frame: 0,
                        ..default()
                    }),
                    frame: 9999,
                    ..default()
                };
                let end_events = (mov.script)(&sit);
                debug!("Move ID: {:?}", id);
                end_events
                    .iter()
                    .find(|ev| matches!(ev, ActionEvent::End | ActionEvent::StartAction(_)))
                    .expect("All moves to end (or start a move)");
            }
        }
    }

    #[test]
    fn moves_with_inputs_have_starter_requirement() {
        for char in [samurai()] {
            for (id, mov) in char.moves.iter() {
                debug!("Move ID: {:?}", id);

                if mov.input.is_some() {
                    assert!(contains_starter(&mov.requirement));
                }
            }
        }
    }

    fn contains_starter(req: &ActionRequirement) -> bool {
        match req {
            ActionRequirement::Starter(_) => true,
            ActionRequirement::And(opts) => opts.iter().any(contains_starter),
            ActionRequirement::Or(opts) => opts.iter().all(contains_starter),
            _ => false,
        }
    }
}
