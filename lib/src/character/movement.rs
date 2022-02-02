use bevy::prelude::*;

use input_parsing::InputParser;
use player_state::PlayerState;
use types::{LRDirection, StickPosition};

pub fn movement(mut query: Query<(&InputParser, &mut PlayerState)>) {
    for (reader, mut state) in query.iter_mut() {
        if state.is_grounded() && state.get_move_state().is_none() && !state.stunned() {
            match reader.get_absolute_stick_position() {
                StickPosition::W => state.walk(LRDirection::Left),
                StickPosition::E => state.walk(LRDirection::Right),
                StickPosition::SW | StickPosition::S | StickPosition::SE => state.crouch(),
                StickPosition::Neutral => state.stand(),
                _ => {}
            }
        }
    }
}
