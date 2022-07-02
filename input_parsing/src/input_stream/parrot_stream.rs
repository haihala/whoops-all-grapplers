use bevy::prelude::*;

use crate::helper_types::Diff;

use super::InputStream;

#[derive(PartialEq, Eq)]
enum ParrotMode {
    Listening,
    Repeating,
    Noop,
}
impl Default for ParrotMode {
    fn default() -> Self {
        ParrotMode::Noop
    }
}

#[derive(Component, Default)]
pub struct ParrotStream {
    mode: ParrotMode,
    buffer: Vec<Option<Diff>>,
    buffer_index: usize,
}

impl ParrotStream {
    fn listen(&mut self, input: Option<Diff>) {
        self.buffer.push(input);
    }

    pub fn cycle(&mut self) {
        self.mode = match self.mode {
            ParrotMode::Listening => {
                dbg!("Starting playback.");
                ParrotMode::Repeating
            }
            ParrotMode::Repeating => {
                dbg!("Entered direct control mode.");
                ParrotMode::Noop
            }
            ParrotMode::Noop => {
                dbg!("Starting recording.");
                self.buffer = vec![];
                self.buffer_index = 0;
                ParrotMode::Listening
            }
        }
    }
}

impl InputStream for ParrotStream {
    fn read(&mut self) -> Option<Diff> {
        if self.mode == ParrotMode::Repeating {
            self.buffer_index = (self.buffer_index + 1) % self.buffer.len();
            self.buffer[self.buffer_index].to_owned()
        } else {
            None
        }
    }
}

pub fn update_parrots<T: InputStream + Component>(mut readers: Query<(&mut ParrotStream, &mut T)>) {
    for (mut parrot, mut stream) in readers.iter_mut() {
        if parrot.mode == ParrotMode::Listening {
            parrot.listen(stream.read());
        } else if parrot.mode == ParrotMode::Repeating {
            // This is to prevent user input while parrot is parroting
            stream.read();
        }
    }
}
