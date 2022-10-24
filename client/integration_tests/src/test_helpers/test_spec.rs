use std::time::Duration;

use characters::dummy;
use input_parsing::{testing::PreWrittenInputBundle, InputEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputClump {
    InputStream(&'static str),
    Idle(Duration),
    // Plan was to have a "wait" and an "assert" type here
}

pub struct TestSpec {
    pub p1_bundle: PreWrittenInputBundle,
    pub p2_bundle: PreWrittenInputBundle,
    pub len: usize,
}
impl TestSpec {
    pub fn new(p1: Vec<InputClump>, p2: Vec<InputClump>) -> Self {
        let character = dummy();
        let p1_events = Self::flatten_events(p1);
        let p2_events = Self::flatten_events(p2);
        let inputs = character.get_inputs();

        Self {
            len: p1_events.len().max(p2_events.len()),
            p1_bundle: PreWrittenInputBundle::new(p1_events, inputs.clone()),
            p2_bundle: PreWrittenInputBundle::new(p2_events, inputs.clone()),
        }
    }

    fn flatten_events(events: Vec<InputClump>) -> Vec<Option<InputEvent>> {
        events
            .into_iter()
            .flat_map(|clump| match clump {
                InputClump::InputStream(inputs) => inputs
                    .chars()
                    .map(|char| Some(InputEvent::from(char)))
                    .collect(),
                InputClump::Idle(frames) => {
                    vec![None; (frames.as_secs_f32() * wag_core::FPS) as usize]
                }
            })
            .collect()
    }
}
