use bevy::prelude::*;

use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{Facing, StickPosition};

pub fn movement_input(mut query: Query<(&InputParser, &mut PlayerState, &Facing)>) {
    for (reader, mut state, facing) in &mut query {
        if state.active_cinematic().is_some() {
            continue;
        }

        if state.is_grounded() && !state.action_in_progress() && !state.stunned() {
            let relative_stick = reader.get_relative_stick_position();
            let stick = if facing.to_flipped() {
                relative_stick.mirror()
            } else {
                relative_stick
            };

            match stick {
                StickPosition::W => state.walk(Facing::Left),
                StickPosition::E => state.walk(Facing::Right),
                StickPosition::SW | StickPosition::S | StickPosition::SE => state.crouch(),
                _ => state.stand(),
            }
        }
    }
}
