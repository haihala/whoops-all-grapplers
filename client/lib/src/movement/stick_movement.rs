use bevy::prelude::*;

use characters::{ActionEvent, ActionEvents};
use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{Facing, StickPosition};

pub fn movement_input(mut query: Query<(&InputParser, &mut PlayerState, &ActionEvents, &Facing)>) {
    for (reader, mut state, events, facing) in &mut query {
        if state.active_cinematic().is_some() {
            continue;
        }

        if state.is_grounded() && state.get_action_tracker().is_none() && !state.stunned() {
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

        for _ in events.get_matching_events(|action| {
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
