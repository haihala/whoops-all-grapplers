use bevy::prelude::*;

use input_parsing::InputParser;
use player_state::PlayerState;
use types::{Facing, StickPosition};

pub fn movement(mut query: Query<(&InputParser, &mut PlayerState)>) {
    for (reader, mut state) in &mut query {
        if state.is_grounded() && state.get_move_history().is_none() && !state.stunned() {
            match reader.get_absolute_stick_position() {
                StickPosition::W => state.walk(Facing::Left),
                StickPosition::E => state.walk(Facing::Right),
                StickPosition::SW | StickPosition::S | StickPosition::SE => state.crouch(),
                StickPosition::Neutral => state.stand(),
                _ => {}
            }
        }
    }
}
