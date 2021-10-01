use bevy::prelude::*;
use uuid::Uuid;

use input_parsing::{GameButton, InputReader, SpecialMove};

pub struct Ryan;

const HADOUKEN: Uuid = Uuid::from_u128(1);

pub fn register_ryan_moves(mut reader: InputReader) -> InputReader {
    reader.register(
        HADOUKEN,
        SpecialMove {
            motion: vec![2, 3, 6].into(),
            button: GameButton::Fast,
        },
    );
    reader
}

pub fn ryan_executor(query: Query<&InputReader, With<Ryan>>) {
    for reader in query.iter() {
        for event in reader.events.iter() {
            match event.id {
                HADOUKEN => {
                    dbg!("Hadouken");
                }
                _ => todo!(),
            }
        }
    }
}
