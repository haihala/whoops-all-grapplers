mod helper_types;
mod input_parser;
mod input_reader;
mod motion_input;

pub use input_parser::InputParser;
pub use input_reader::InputReader;

use bevy::prelude::*;
use std::collections::VecDeque;

pub struct InputParsingPlugin;

impl Plugin for InputParsingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(VecDeque::<Gamepad>::default())
            .add_system(input_reader::update_readers.system().label("collect"))
            .add_system(input_parser::parse_input.system().after("collect"));
    }
}
