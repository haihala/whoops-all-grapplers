use bevy::prelude::*;

use input_parsing::InputParser;
use player_state::PlayerState;
use wag_core::{ActionId, Clock, Facing, StatusFlag, StickPosition};

use crate::player_state_management::MoveBuffer;

pub fn movement_input(
    mut query: Query<(&InputParser, &mut PlayerState, &mut MoveBuffer, &Facing)>,
    clock: Res<Clock>,
) {
    for (reader, mut state, mut buffer, facing) in &mut query {
        if state.has_flag(StatusFlag::MovementLock) {
            continue;
        }

        if state.is_grounded() && !state.action_in_progress() && !state.stunned() {
            match dbg!(reader.get_stick_pos()) {
                StickPosition::W => state.walk(Facing::Left),
                StickPosition::E => state.walk(Facing::Right),
                StickPosition::SW | StickPosition::S | StickPosition::SE => state.crouch(),
                StickPosition::Neutral => state.stand(),
                _ => {
                    // Jumps are relative, the rest are absolute
                    let mirrored_stick = facing.mirror_stick_pos(reader.get_stick_pos());

                    match mirrored_stick {
                        StickPosition::N => {
                            buffer.add_events(vec![ActionId::NeutralJump], clock.frame)
                        }
                        StickPosition::NW => {
                            buffer.add_events(vec![ActionId::BackJump], clock.frame)
                        }
                        StickPosition::NE => {
                            buffer.add_events(vec![ActionId::ForwardJump], clock.frame)
                        }
                        _ => panic!("How did this stick position not get handled earlier?"),
                    }
                }
            }
        }
    }
}
