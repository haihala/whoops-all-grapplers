use bevy::{prelude::*, utils::HashMap};
use std::collections::VecDeque;
use time::WAGStage;
use types::MoveId;

mod helper_types;
mod input_parser;
mod input_stream;
mod motion_input;

pub use input_parser::InputParser;

use input_stream::PadStream;
#[cfg(test)]
pub use input_stream::TestStream;

const MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS: f32 = 0.2; // In seconds
const CHARGE_TIME: f32 = 1.0; // In seconds
const EVENT_REPEAT_PERIOD: f32 = 0.3; // In seconds
const STICK_DEAD_ZONE: f32 = 0.2;

#[derive(Debug, SystemLabel, Clone, Copy, PartialEq, Eq, Hash)]
enum InputSystemLabel {
    Collect,
    Parse,
}

pub struct InputParsingPlugin;

impl Plugin for InputParsingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VecDeque::<Gamepad>::default())
            .add_system_set_to_stage(
                WAGStage::Inputs,
                SystemSet::new()
                    .with_system(input_stream::update_pads.label(InputSystemLabel::Collect))
                    .with_system(
                        input_parser::parse_input::<PadStream>
                            .label(InputSystemLabel::Parse)
                            .after(InputSystemLabel::Collect),
                    ),
            );
    }
}

#[derive(Bundle)]
pub struct PadBundle {
    reader: PadStream,
    parser: InputParser,
}
impl PadBundle {
    pub fn new(inputs: HashMap<MoveId, &str>) -> Self {
        Self {
            reader: PadStream::default(),
            parser: InputParser::load(inputs),
        }
    }
}

#[cfg(test)]
#[derive(Bundle)]
pub struct TestInputBundle {
    reader: TestStream,
    parser: InputParser,
}
#[cfg(test)]
impl TestInputBundle {
    pub fn new(inputs: HashMap<MoveId, &str>) -> Self {
        Self {
            reader: TestStream::default(),
            parser: InputParser::load(inputs),
        }
    }
}
