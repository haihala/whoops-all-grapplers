use bevy::prelude::*;

use crate::InputEvent;

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
    buffer: Vec<Vec<InputEvent>>,
    buffer_index: usize,
}

impl ParrotStream {
    fn listen(&mut self, input: Vec<InputEvent>) {
        self.buffer.push(input);
    }

    pub fn cycle(&mut self) {
        self.mode = match self.mode {
            ParrotMode::Listening => {
                info!("Starting playback.");
                ParrotMode::Repeating
            }
            ParrotMode::Repeating => {
                info!("Entered direct control mode.");
                ParrotMode::Noop
            }
            ParrotMode::Noop => {
                info!("Starting recording.");
                self.buffer = vec![];
                self.buffer_index = 0;
                ParrotMode::Listening
            }
        }
    }
}

impl InputStream for ParrotStream {
    fn read(&mut self) -> Vec<InputEvent> {
        if self.mode == ParrotMode::Repeating {
            self.buffer_index = (self.buffer_index + 1) % self.buffer.len();
            self.buffer[self.buffer_index].to_owned()
        } else if self.mode == ParrotMode::Listening {
            self.buffer
                .last()
                .map(|inner| inner.to_owned())
                .unwrap_or_default()
        } else {
            vec![]
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
