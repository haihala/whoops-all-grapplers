use bevy::{prelude::*, utils::HashMap};
use wag_core::{ActionId, InMatch, RollbackSchedule, WAGStage};

mod helper_types;
mod input_parser;
mod input_stream;
mod motion_input;

pub use helper_types::InputEvent;
pub use input_parser::InputParser;
pub use input_stream::{PadStream, ParrotStream};

use input_stream::{update_pads, update_parrots};

pub struct InputParsingPlugin;

impl Plugin for InputParsingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RollbackSchedule,
            (
                update_pads,
                update_parrots::<PadStream>,
                // Very important for this to happen after we've updated parrots
                // If an entity has a parrot stream, it will drain the basic pad stream
                input_parser::parse_input::<PadStream>,
                input_parser::parse_input::<ParrotStream>,
            )
                .chain()
                .in_set(WAGStage::Inputs)
                .run_if(in_state(InMatch)),
        );
    }
}

#[derive(Bundle)]
pub struct PadBundle {
    reader: PadStream,
    parser: InputParser,
    parrot: ParrotStream,
}
impl PadBundle {
    pub fn new(mut inputs: HashMap<ActionId, String>) -> Self {
        inputs.extend(generic_inputs());
        Self {
            reader: PadStream::default(),
            parser: InputParser::new(inputs),
            parrot: ParrotStream::default(),
        }
    }
}

fn generic_inputs() -> impl Iterator<Item = (ActionId, String)> {
    vec![
        (ActionId::Up, "8|5"),
        (ActionId::Down, "2|5"),
        (ActionId::Left, "4|A5"),
        (ActionId::Right, "6|A5"),
        (ActionId::Back, "4|5"),
        (ActionId::Forward, "6|5"),
        (ActionId::Primary, "f"),
        (ActionId::Secondary, "g"),
        (ActionId::Cancel, "s"),
        (ActionId::Start, "."), // It was at this point when I realized this shit was stupid for the UI thingies.
    ]
    .into_iter()
    .map(|(id, dsl)| (id, dsl.to_string()))
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
        parrot: ParrotStream,
    }
    impl PreWrittenInputBundle {
        pub fn new(events: Vec<Vec<InputEvent>>, inputs: HashMap<ActionId, String>) -> Self {
            Self {
                reader: PreWrittenStream::new(events),
                parser: InputParser::new(inputs),
                parrot: ParrotStream::default(),
            }
        }
    }

    #[derive(Bundle)]
    pub struct TestInputBundle {
        reader: TestStream,
        parser: InputParser,
        parrot: ParrotStream,
    }
    impl TestInputBundle {
        pub fn new(inputs: HashMap<ActionId, String>) -> Self {
            Self {
                reader: TestStream::default(),
                parser: InputParser::new(inputs),
                parrot: ParrotStream::default(),
            }
        }
    }
}
