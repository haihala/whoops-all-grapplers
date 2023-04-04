use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};
use wag_core::{MoveId, WAGStage};

mod helper_types;
mod input_parser;
mod input_stream;
mod motion_input;

pub use helper_types::InputEvent;
pub use input_parser::InputParser;

use input_stream::{update_pads, update_parrots, PadStream, ParrotStream};

const MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS: f32 = 0.2; // In seconds
const STICK_DEAD_ZONE: f32 = 0.2;

pub struct InputParsingPlugin;

#[derive(Debug, Default, Resource, Deref, DerefMut)]
struct PadReserve(VecDeque<Gamepad>);
impl PadReserve {
    fn remove_pad(&mut self, pad: &Gamepad) {
        self.0.retain(|p| p != pad);
    }
}

impl Plugin for InputParsingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PadReserve::default()).add_systems(
            (
                update_pads,
                update_parrots::<PadStream>,
                // Very important for this to happen after we've updated parrots
                // If an entity has a parrot stream, it will drain the basic pad stream
                input_parser::parse_input::<PadStream>,
                input_parser::parse_input::<ParrotStream>,
                input_parser::flip_parsers_on_side_change,
            )
                .in_set(WAGStage::Inputs),
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
    pub fn new(mut inputs: HashMap<MoveId, &'static str>) -> Self {
        inputs.extend(generic_inputs());
        Self {
            reader: PadStream::default(),
            parser: InputParser::new(inputs),
            parrot: ParrotStream::default(),
        }
    }
}

fn generic_inputs() -> impl Iterator<Item = (MoveId, &'static str)> {
    vec![
        (MoveId::Up, "58"),
        (MoveId::Down, "52"),
        (MoveId::Back, "54"),
        (MoveId::Forward, "56"),
        (MoveId::Primary, "f"),
        (MoveId::Secondary, "g"),
        (MoveId::Cancel, "s"),
        (MoveId::Start, "."), // It was at this point when I realized this shit was stupid for the UI thingies.
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
        pub fn new(events: Vec<Option<InputEvent>>, inputs: HashMap<MoveId, &'static str>) -> Self {
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
        pub fn new(inputs: HashMap<MoveId, &'static str>) -> Self {
            Self {
                reader: TestStream::default(),
                parser: InputParser::new(inputs),
                parrot: ParrotStream::default(),
            }
        }
    }
}
