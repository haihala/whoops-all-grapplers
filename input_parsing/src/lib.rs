mod input_reader;
pub use input_reader::InputReader;
mod types;
pub(crate) use types::*;
pub use types::{GameButton, SpecialMove, StickPosition};

use bevy::prelude::*;
use std::collections::VecDeque;

pub(crate) static MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS: f32 = 0.3; // In seconds
pub(crate) static EVENT_REPEAT_PERIOD: f32 = 0.3; // In seconds
pub(crate) static STICK_DEAD_ZONE: f32 = 0.2;

pub struct InputParsingPlugin;

impl Plugin for InputParsingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(VecDeque::<Gamepad>::default())
            .add_system(input_reader::parse_controller_input.system());
    }
}
