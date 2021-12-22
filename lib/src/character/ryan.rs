use bevy::prelude::*;

use input_parsing::InputParser;
use player_state::FreedomLevel;
use types::MoveType;

use super::movement::{DASH_BACK, DASH_FORWARD};
use super::PlayerState;
use crate::{clock::Clock, frame_data_manager::FrameDataManager};

pub struct Ryan;

pub fn move_starter(
    clock: Res<Clock>,
    mut query: Query<(&mut InputParser, &PlayerState, &mut FrameDataManager), With<Ryan>>,
) {
    for (mut reader, state, mut frame_data) in query.iter_mut() {
        if state.freedom_level() >= FreedomLevel::LightBusy && state.is_grounded() {
            let events = reader.get_events();
            if events.is_empty() {
                continue;
            }

            let to_start = highest_priority_move(events);
            if to_start != DASH_FORWARD && to_start != DASH_BACK {
                reader.consume_event(&to_start);

                frame_data.start(to_start, clock.frame);
            }
        }
    }
}

fn highest_priority_move(options: Vec<MoveType>) -> MoveType {
    options.into_iter().min().unwrap()
}
