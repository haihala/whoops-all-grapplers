use std::collections::VecDeque;

use bevy::prelude::Component;

use super::InputStream;
use crate::helper_types::InputEvent;

#[derive(Debug, Component, Default)]
pub struct PreWrittenStream {
    events: VecDeque<Vec<InputEvent>>,
}
impl PreWrittenStream {
    pub fn new(events: Vec<Vec<InputEvent>>) -> Self {
        Self {
            events: events.into(),
        }
    }
}
impl InputStream for PreWrittenStream {
    fn read(&mut self) -> Vec<InputEvent> {
        self.events.pop_front().unwrap_or_default()
    }
}
