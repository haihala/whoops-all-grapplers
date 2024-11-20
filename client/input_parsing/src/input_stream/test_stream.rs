use bevy::prelude::*;

use super::InputStream;
use crate::helper_types::InputEvent;

#[derive(Debug, Component, Default)]
pub struct TestStream {
    next_read: Vec<InputEvent>,
}
impl TestStream {
    pub fn push(&mut self, change: InputEvent) {
        debug!("Pushed input: {:?}", &change);
        self.next_read.push(change);
    }
}
impl InputStream for TestStream {
    fn read(&mut self) -> Vec<InputEvent> {
        let temp = self.next_read.clone();
        debug!("Test stream output: {:?}", &temp);
        self.next_read.clear();
        temp
    }
}
