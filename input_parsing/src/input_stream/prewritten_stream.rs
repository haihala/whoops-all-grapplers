use std::collections::VecDeque;

use bevy::prelude::Component;

use super::InputStream;
use crate::{helper_types::Diff, InputEvent};

#[derive(Debug, Component, Default)]
pub struct PreWrittenStream {
    events: VecDeque<Option<InputEvent>>,
}
impl PreWrittenStream {
    pub fn new(events: Vec<Option<InputEvent>>) -> Self {
        Self {
            events: events.into(),
        }
    }
}
impl InputStream for PreWrittenStream {
    fn read(&mut self) -> Option<Diff> {
        if let Some(event) = self.events.pop_front() {
            event.map(|ev| Diff::default().apply_event(ev))
        } else {
            None
        }
    }
}
