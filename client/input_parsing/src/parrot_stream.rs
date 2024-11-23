use bevy::prelude::*;
use wag_core::{Controllers, InputEvent, InputStream, Player};

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
    pub next_read: Vec<InputEvent>,
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

pub fn update_parrots(
    mut readers: Query<(&mut ParrotStream, &Player)>,
    controllers: Res<Controllers>,
    stream: Res<InputStream>,
) {
    let evs = stream.events.clone();
    for (mut parrot, player) in &mut readers {
        let matching: Vec<_> = evs
            .clone()
            .into_iter()
            .filter(|ev| ev.player_handle == controllers.get_handle(*player))
            .map(|ev| ev.event)
            .collect();

        match parrot.mode {
            ParrotMode::Recording => {
                parrot.listen(matching.clone());
                parrot.next_read = matching;
            }
            ParrotMode::Repeating => {
                parrot.buffer_index = (parrot.buffer_index + 1) % parrot.buffer.len();
                parrot.next_read = parrot.buffer[parrot.buffer_index].to_owned();
            }
            ParrotMode::Passthrough => parrot.next_read = matching,
        };
    }
}
