use bevy::prelude::Component;

use super::InputStream;
use crate::helper_types::{Diff, InputEvent};

#[derive(Debug, Component, Default)]
pub struct TestStream {
    next_read: Vec<InputEvent>,
}
impl TestStream {
    #[cfg(test)]
    pub fn push(&mut self, change: InputEvent) {
        self.next_read.push(change);
    }
}
impl InputStream for TestStream {
    fn read(&mut self) -> Option<Diff> {
        if self.next_read.is_empty() {
            None
        } else {
            let value = self
                .next_read
                .drain(..)
                .fold(Diff::default(), |diff, change| diff.apply(change));
            Some(value)
        }
    }
}
