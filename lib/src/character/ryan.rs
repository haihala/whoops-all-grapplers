use bevy::prelude::*;
use uuid::Uuid;

use input_parsing::{GameButton, InputReader, Special};

use crate::player::MainState;

pub struct Ryan;

const HADOUKEN: Uuid = Uuid::from_u128(1);

pub fn register_ryan_moves(mut reader: InputReader) -> InputReader {
    reader.register_special(
        HADOUKEN,
        Special {
            motion: vec![2, 3, 6].into(),
            button: GameButton::Fast,
        },
    );
    reader
}

pub fn ryan_executor(mut query: Query<(&mut InputReader, &MainState), With<Ryan>>) {
    for (mut reader, state) in query.iter_mut() {
        if *state == MainState::Standing {
            for event in reader.get_events() {
                match event {
                    HADOUKEN => {
                        dbg!("Hadouken");
                    }
                    _ => todo!(),
                }
            }
        }
    }
}
