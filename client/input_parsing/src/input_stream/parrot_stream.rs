use bevy::prelude::*;

use crate::helper_types::Diff;

use super::InputStream;

#[derive(PartialEq, Eq, Default, Clone, Copy, Reflect)]
enum ParrotMode {
    Listening,
    Repeating,
    #[default]
    Noop,
}

#[derive(Component, Default, Clone, Reflect)]
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
                println!("Starting playback.");
                ParrotMode::Repeating
            }
            ParrotMode::Repeating => {
                println!("Entered direct control mode.");
                ParrotMode::Noop
            }
            ParrotMode::Noop => {
                println!("Starting recording.");
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
        } else if self.mode == ParrotMode::Listening {
            self.buffer.last().and_then(|inner| inner.to_owned())
        } else {
            None
        }
    }
}

pub fn update_parrots<T: InputStream + Component>(mut readers: Query<(&mut ParrotStream, &mut T)>) {
    for (mut parrot, mut stream) in &mut readers {
        if parrot.mode == ParrotMode::Listening {
            parrot.listen(stream.read());
        } else if parrot.mode == ParrotMode::Repeating {
            // This is to prevent user input while parrot is parroting
            stream.read();
        }
    }
}
