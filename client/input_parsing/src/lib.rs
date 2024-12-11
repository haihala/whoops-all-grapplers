use bevy::{prelude::*, utils::HashMap};
use parrot_stream::update_parrots;
use wag_core::{ActionId, InMatch, RollbackSchedule, SystemStep};

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
    pub fn new(mut inputs: HashMap<ActionId, String>) -> Self {
        inputs.extend(generic_inputs());
        Self::without_generic_inputs(inputs)
    }

    pub fn without_generic_inputs(inputs: HashMap<ActionId, String>) -> Self {
        Self {
            parser: InputParser::new(inputs),
            parrot: ParrotStream::default(),
        }
    }
}

fn generic_inputs() -> impl Iterator<Item = (ActionId, String)> {
    vec![
        (ActionId::Up, "{5}8"),
        (ActionId::Down, "{5}2"),
        (ActionId::Left, "{5}4|A"),
        (ActionId::Right, "{5}6|A"),
        (ActionId::Back, "{5}4"),
        (ActionId::Forward, "{5}6"),
        (ActionId::Primary, "f"),
        (ActionId::Secondary, "g"),
        (ActionId::Cancel, "s"),
        (ActionId::Start, "."), // It was at this point when I realized this shit was stupid for the UI thingies.
    ]
    .into_iter()
    .map(|(id, dsl)| (id, dsl.to_string()))
}
