use bevy::prelude::Component;

use super::InputStream;
use crate::helper_types::{Diff, InputChange};

#[derive(Debug, Component, Default)]
pub struct TestStream {
    next_read: Vec<InputChange>,
}
impl TestStream {
    #[cfg(test)]
    pub fn push(&mut self, change: InputChange) {
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
                .fold(Diff::default(), |diff, change| diff.apply(&change));
            Some(value)
        }
    }
}
