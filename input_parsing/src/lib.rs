mod helper_types;
mod input_reader;
mod motion_input;
mod special;
pub use input_reader::InputReader;

use bevy::prelude::*;
use std::collections::VecDeque;

pub struct InputParsingPlugin;

impl Plugin for InputParsingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(VecDeque::<Gamepad>::default())
            .add_system(input_reader::parse_controller_input.system());
    }
}
