use bevy::prelude::*;

use input_parsing::InputReader;
use types::{FreedomLevel, PlayerState, RelativeDirection, StickPosition};

use crate::clock::Clock;

pub use moves::universal::{DASH_BACK, DASH_FORWARD};

pub fn movement(mut query: Query<(&mut InputReader, &mut PlayerState)>, clock: Res<Clock>) {
    for (mut reader, mut state) in query.iter_mut() {
        if reader.is_active()
            && state.freedom_level(clock.frame) >= FreedomLevel::LightBusy
            && state.is_grounded()
        {
            let events = reader.get_events();
            // Order matters here, since setting state can override other state to maintain legality
            // Walking
            match reader.get_relative_stick_position() {
                StickPosition::W => state.walk(RelativeDirection::Back),
                StickPosition::E => state.walk(RelativeDirection::Forward),
                _ => state.stop_walking(),
            }

            // Jumping
            match reader.get_relative_stick_position() {
                StickPosition::N => state.register_jump(None),
                StickPosition::NW => state.register_jump(Some(RelativeDirection::Back)),
                StickPosition::NE => state.register_jump(Some(RelativeDirection::Forward)),
                _ => {}
            }

            // Dashing
            if events.contains(&DASH_FORWARD) {
                reader.consume_event(&DASH_FORWARD);
                state.start_dash(RelativeDirection::Forward, clock.frame);
            } else if events.contains(&DASH_BACK) {
                reader.consume_event(&DASH_BACK);
                state.start_dash(RelativeDirection::Back, clock.frame);
            }
        }
    }
}
