use bevy::prelude::*;
use input_parsing::InputParser;
use player_state::PlayerState;

pub fn set_flags(mut query: Query<(&InputParser, &mut PlayerState)>) {
    for (parser, mut state) in query.iter_mut() {
        state.set_flags(parser.get_pressed());
    }
}
