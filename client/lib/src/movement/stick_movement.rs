use bevy::prelude::*;

use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{Facing, StatusFlag, StickPosition};

pub fn movement_input(mut query: Query<(&InputParser, &mut PlayerState)>) {
    for (reader, mut state) in &mut query {
        if state.has_flag(StatusFlag::MovementLock) {
            continue;
        }

        if state.is_grounded() && !state.action_in_progress() && !state.stunned() {
            match reader.get_stick_pos() {
                StickPosition::W => state.walk(Facing::Left),
                StickPosition::E => state.walk(Facing::Right),
                StickPosition::SW | StickPosition::S | StickPosition::SE => state.crouch(),
                _ => state.stand(),
            }
        }
    }
}
