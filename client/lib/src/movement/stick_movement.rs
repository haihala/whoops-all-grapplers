use bevy::prelude::*;

use foundation::{ActionId, CharacterClock, CharacterFacing, Facing, StatusFlag, StickPosition};
use input_parsing::InputParser;
use player_state::PlayerState;

use crate::player_state_management::MoveBuffer;

pub fn movement_input(
    mut query: Query<(
        &InputParser,
        &mut PlayerState,
        &mut MoveBuffer,
        &CharacterFacing,
        &CharacterClock,
    )>,
) {
    for (reader, mut state, mut buffer, facing, rel_clock) in &mut query {
        if state.has_flag(StatusFlag::MovementLock) {
            continue;
        }

        if state.is_grounded() && !state.action_in_progress() && !state.stunned() {
            match reader.get_stick_pos() {
                StickPosition::W => state.walk(Facing::Left),
                StickPosition::E => state.walk(Facing::Right),
                StickPosition::SW | StickPosition::S | StickPosition::SE => state.crouch(),
                StickPosition::Neutral => state.stand(),
                _ => {
                    // Jumps are relative, the rest are absolute
                    let mirrored_stick = facing.absolute.mirror_stick_pos(reader.get_stick_pos());

                    match mirrored_stick {
                        StickPosition::N => {
                            buffer.add_events([ActionId::NeutralJump].into_iter(), rel_clock.frame)
                        }
                        StickPosition::NW => {
                            buffer.add_events([ActionId::BackJump].into_iter(), rel_clock.frame)
                        }
                        StickPosition::NE => {
                            buffer.add_events([ActionId::ForwardJump].into_iter(), rel_clock.frame)
                        }
                        _ => panic!("How did this stick position not get handled earlier?"),
                    }
                }
            }
        }
    }
}
