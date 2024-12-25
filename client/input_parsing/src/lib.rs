use bevy::{prelude::*, utils::HashMap};
use foundation::{ActionId, InMatch, RollbackSchedule, SystemStep};
use parrot_stream::update_parrots;

mod helper_types;
mod input_parser;
mod motion_input;
mod parrot_stream;

pub use input_parser::InputParser;
pub use parrot_stream::ParrotStream;

pub struct InputParsingPlugin;

impl Plugin for InputParsingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RollbackSchedule,
            (update_parrots, input_parser::parse_input)
                .chain()
                .in_set(SystemStep::Inputs)
                .run_if(in_state(InMatch)),
        );
    }
}

#[derive(Bundle)]
pub struct PadBundle {
    parser: InputParser,
    parrot: ParrotStream,
}
impl PadBundle {
    pub fn new(inputs: HashMap<ActionId, String>) -> Self {
        Self {
            parser: InputParser::new(inputs),
            parrot: ParrotStream::default(),
        }
    }
}
