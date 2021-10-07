use bevy::prelude::*;

use super::movement::{DASH_BACK, DASH_FORWARD};
use super::PlayerState;
use crate::{frame_data_manager::FrameDataManager, Clock};
use input_parsing::{GameButton, InputReader, Normal, Special};
use moves::{ryan::*, MoveType};

pub struct Ryan;

pub fn inputs() -> InputReader {
    let mut reader = InputReader::default();

    reader.register_special(
        HADOUKEN,
        Special {
            motion: vec![2, 3, 6].into(),
            button: Some(GameButton::Fast),
        },
    );

    reader.register_normal(
        PUNCH,
        Normal {
            button: GameButton::Fast,
            stick: None,
        },
    );

    reader.register_special(
        DASH_FORWARD,
        Special {
            motion: (vec![6, 5, 6], vec![7, 4, 1]).into(),
            button: None,
        },
    );

    reader.register_special(
        DASH_BACK,
        Special {
            motion: (vec![4, 5, 4], vec![9, 6, 3]).into(),
            button: None,
        },
    );
    reader
}

pub fn move_starter(
    clock: Res<Clock>,
    mut query: Query<(&mut InputReader, &mut PlayerState, &mut FrameDataManager), With<Ryan>>,
) {
    for (mut reader, mut state, mut animation) in query.iter_mut() {
        if *state == PlayerState::Standing {
            let events = reader.get_events();
            if events.is_empty() {
                continue;
            }

            let to_start = highest_priority_move(events);
            if to_start != DASH_FORWARD && to_start != DASH_BACK {
                *state = PlayerState::Startup;
                reader.consume_event(&to_start);
                animation.start(to_start, clock.frame);
            }
        }
    }
}

fn highest_priority_move(options: Vec<MoveType>) -> MoveType {
    if options.contains(&HADOUKEN) {
        HADOUKEN
    } else if options.contains(&PUNCH) {
        PUNCH
    } else if options.len() == 1 {
        options[0]
    } else {
        todo!()
    }
}
