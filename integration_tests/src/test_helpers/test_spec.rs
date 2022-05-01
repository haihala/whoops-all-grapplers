use std::time::Duration;

use bevy::utils::HashMap;
use input_parsing::{testing::PreWrittenInputBundle, InputEvent};
use kits::{ryan_kit, MoveId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputClump {
    InputStream(&'static str),
    Idle(Duration),
    // Plan was to have a "wait" and an "assert" type here
}

pub struct TestSpec {
    p1_events: Vec<Option<InputEvent>>,
    p1_inputs: HashMap<MoveId, &'static str>,
    p2_events: Vec<Option<InputEvent>>,
    p2_inputs: HashMap<MoveId, &'static str>,
}
impl TestSpec {
    pub fn new(p1_events: Vec<InputClump>, p2_events: Vec<InputClump>) -> Self {
        let kit = ryan_kit();
        let inputs = kit.get_inputs();
        Self {
            p1_events: Self::flatten_events(p1_events),
            p1_inputs: inputs.clone(),
            p2_events: Self::flatten_events(p2_events),
            p2_inputs: inputs.clone(),
        }
    }

    pub fn len(&self) -> usize {
        self.p1_events.len().max(self.p2_events.len())
    }
    pub fn p1_bundle(&self) -> PreWrittenInputBundle {
        PreWrittenInputBundle::new(self.p1_events.to_owned(), self.p1_inputs.to_owned())
    }
    pub fn p2_bundle(&self) -> PreWrittenInputBundle {
        PreWrittenInputBundle::new(self.p2_events.to_owned(), self.p2_inputs.to_owned())
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
                    vec![None; (frames.as_secs_f32() * constants::FPS) as usize]
                }
            })
            .collect()
    }
}
