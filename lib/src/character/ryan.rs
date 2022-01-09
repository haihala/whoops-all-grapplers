use bevy::prelude::*;

use input_parsing::InputParser;
use moves::MoveBank;

use super::{move_to_activate, PlayerState};

pub struct Ryan;

pub fn move_starter(mut query: Query<(&mut InputParser, &mut PlayerState, &MoveBank), With<Ryan>>) {
    for (mut reader, mut state, bank) in query.iter_mut() {
        let events = reader.get_events();
        if events.is_empty() {
            continue;
        }

        if let Some((starting_move, move_data)) = move_to_activate(
            events,
            bank,
            state.cancel_requirement(),
            state.is_grounded(),
        ) {
            state.start_move(starting_move, move_data);
            reader.consume_event(starting_move);
        }
    }
}
