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

const MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS: f32 = 0.2; // In seconds

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
                input_parser::flip_parsers,
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
    pub fn new(mut inputs: HashMap<ActionId, &'static str>) -> Self {
        inputs.extend(generic_inputs());
        Self {
            reader: PadStream::default(),
            parser: InputParser::new(inputs),
            parrot: ParrotStream::default(),
        }
    }
}

fn generic_inputs() -> impl Iterator<Item = (ActionId, &'static str)> {
    vec![
        (ActionId::Up, "58"),
        (ActionId::Down, "52"),
        (ActionId::Back, "54"),
        (ActionId::Forward, "56"),
        (ActionId::Primary, "f"),
        (ActionId::Secondary, "g"),
        (ActionId::Cancel, "s"),
        (ActionId::Start, "."), // It was at this point when I realized this shit was stupid for the UI thingies.
    ]
    .into_iter()
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
        pub fn new(
            events: Vec<Option<InputEvent>>,
            inputs: HashMap<ActionId, &'static str>,
        ) -> Self {
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
        pub fn new(inputs: HashMap<ActionId, &'static str>) -> Self {
            Self {
                reader: TestStream::default(),
                parser: InputParser::new(inputs),
                parrot: ParrotStream::default(),
            }
        }
    }
}
