use bevy::prelude::*;

use input_parsing::InputReader;
use types::MoveType;

use super::movement::{DASH_BACK, DASH_FORWARD};
use super::PlayerState;
use crate::{frame_data_manager::FrameDataManager, Clock};

pub struct Ryan;

pub fn move_starter(
    clock: Res<Clock>,
    mut query: Query<(&mut InputReader, &mut PlayerState, &mut FrameDataManager), With<Ryan>>,
) {
    for (mut reader, mut state, mut animation) in query.iter_mut() {
        if state.can_act() && state.is_grounded() {
            let events = reader.get_events();
            if events.is_empty() {
                continue;
            }

            let to_start = highest_priority_move(events);
            if to_start != DASH_FORWARD && to_start != DASH_BACK {
                reader.consume_event(&to_start);

                state.start_animation();
                animation.start(to_start, clock.frame);
            }
        }
    }
}

fn highest_priority_move(options: Vec<MoveType>) -> MoveType {
    options.into_iter().min().unwrap()
}
