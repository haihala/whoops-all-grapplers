use bevy::prelude::*;

use crate::InputEvent;

use super::InputStream;

#[derive(PartialEq, Eq, Default, Clone, Copy, Reflect)]
enum ParrotMode {
    Recording,
    Repeating,
    #[default]
    Passthrough,
}

#[derive(Component, Default, Clone, Reflect)]
pub struct ParrotStream {
    mode: ParrotMode,
    buffer: Vec<Vec<InputEvent>>,
    buffer_index: usize,
    next_read: Vec<InputEvent>,
}

impl ParrotStream {
    fn listen(&mut self, input: Vec<InputEvent>) {
        self.buffer.push(input);
    }

    pub fn cycle(&mut self) {
        self.mode = match self.mode {
            ParrotMode::Recording => {
                info!("Starting playback.");
                ParrotMode::Repeating
            }
            ParrotMode::Repeating => {
                info!("Entered direct control mode.");
                ParrotMode::Passthrough
            }
            ParrotMode::Passthrough => {
                info!("Starting recording.");
                self.buffer = vec![];
                self.buffer_index = 0;
                ParrotMode::Recording
            }
        }
    }
}

impl InputStream for ParrotStream {
    fn read(&mut self) -> Vec<InputEvent> {
        let temp = self.next_read.clone();
        self.next_read.clear();
        temp
    }
}

pub fn update_parrots<T: InputStream + Component>(mut readers: Query<(&mut ParrotStream, &mut T)>) {
    for (mut parrot, mut stream) in &mut readers {
        let evs = stream.read();

        match parrot.mode {
            ParrotMode::Recording => {
                parrot.listen(evs.clone());
                parrot.next_read = evs;
            }
            ParrotMode::Repeating => {
                parrot.buffer_index = (parrot.buffer_index + 1) % parrot.buffer.len();
                parrot.next_read = parrot.buffer[parrot.buffer_index].to_owned();
            }
            ParrotMode::Passthrough => parrot.next_read = evs,
        };
    }
}
