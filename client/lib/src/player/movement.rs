use bevy::prelude::*;

use characters::ActionEvent;
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{Facing, StickPosition};

pub fn movement(mut query: Query<(&InputParser, &mut PlayerState, &Facing)>) {
    for (reader, mut state, facing) in &mut query {
        if state.is_grounded() && state.get_move_history().is_none() && !state.stunned() {
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

        for _ in state.drain_matching_actions(|action| {
            if *action == ActionEvent::ForceStand {
                Some(action.to_owned())
            } else {
                None
            }
        }) {
            state.force_stand()
        }
    }
}
