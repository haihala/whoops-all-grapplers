use bevy::prelude::*;

use characters::Action;
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{Facing, StickPosition};

pub fn movement(mut query: Query<(&InputParser, &mut PlayerState)>) {
    for (reader, mut state) in &mut query {
        if state.is_grounded() && state.get_move_history().is_none() && !state.stunned() {
            match reader.get_absolute_stick_position() {
                StickPosition::W => state.walk(Facing::Left),
                StickPosition::E => state.walk(Facing::Right),
                StickPosition::SW | StickPosition::S | StickPosition::SE => state.crouch(),
                _ => state.stand(),
            }
        }

        for _ in state.drain_matching_actions(|action| {
            if *action == Action::ForceStand {
                Some(*action)
            } else {
                None
            }
        }) {
            state.force_stand()
        }
    }
}
