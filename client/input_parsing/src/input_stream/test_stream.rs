use bevy::prelude::Component;

use super::InputStream;
use crate::helper_types::InputEvent;

#[derive(Debug, Component, Default)]
pub struct TestStream {
    next_read: Vec<InputEvent>,
}
impl TestStream {
    pub fn push(&mut self, change: InputEvent) {
        dbg!(&change);
        self.next_read.push(change);
    }
}
impl InputStream for TestStream {
    fn read(&mut self) -> Vec<InputEvent> {
        let temp = self.next_read.clone();
        self.next_read.clear();
        dbg!(temp)
    }
}
