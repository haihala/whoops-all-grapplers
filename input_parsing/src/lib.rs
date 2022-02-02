mod helper_types;
mod input_parser;
mod input_reader;
mod motion_input;

pub use input_parser::InputParser;
pub use input_reader::InputReader;

use bevy::prelude::*;
use std::collections::VecDeque;

pub const MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS: f32 = 0.2; // In seconds
pub const CHARGE_TIME: f32 = 1.0; // In seconds
pub const EVENT_REPEAT_PERIOD: f32 = 0.3; // In seconds
pub const STICK_DEAD_ZONE: f32 = 0.2;

pub struct InputParsingPlugin;

impl Plugin for InputParsingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VecDeque::<Gamepad>::default())
            .add_system(input_reader::update_readers.label("collect"))
            .add_system(input_parser::parse_input.after("collect"));
    }
}
