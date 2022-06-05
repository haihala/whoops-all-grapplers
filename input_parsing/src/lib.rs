use bevy::{prelude::*, utils::HashMap};
use kits::MoveId;
use std::collections::VecDeque;
use time::WAGStage;

mod helper_types;
mod input_parser;
mod input_stream;
mod motion_input;

pub use helper_types::InputEvent;
pub use input_parser::InputParser;

use input_stream::PadStream;

const MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS: f32 = 0.2; // In seconds
const STICK_DEAD_ZONE: f32 = 0.2;

pub struct InputParsingPlugin;

impl Plugin for InputParsingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VecDeque::<Gamepad>::default())
            .add_system_set_to_stage(
                WAGStage::Inputs,
                SystemSet::new()
                    .with_system(input_stream::update_pads)
                    .with_system(
                        input_parser::parse_input::<PadStream>.after(input_stream::update_pads),
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

pub mod testing {
    use super::*;
    pub use input_parser::parse_input;
    pub use input_stream::PreWrittenStream;
    pub use input_stream::TestStream;

    #[derive(Bundle)]
    pub struct PreWrittenInputBundle {
        reader: PreWrittenStream,
        parser: InputParser,
    }
    impl PreWrittenInputBundle {
        pub fn new(events: Vec<Option<InputEvent>>, inputs: HashMap<MoveId, &str>) -> Self {
            Self {
                reader: PreWrittenStream::new(events),
                parser: InputParser::load(inputs),
            }
        }
    }

    #[derive(Bundle)]
    pub struct TestInputBundle {
        reader: TestStream,
        parser: InputParser,
    }
    impl TestInputBundle {
        pub fn new(inputs: HashMap<MoveId, &str>) -> Self {
            Self {
                reader: TestStream::default(),
                parser: InputParser::load(inputs),
            }
        }
    }
}
