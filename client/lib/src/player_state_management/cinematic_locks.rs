use bevy::prelude::*;
use player_state::PlayerState;
use wag_core::Clock;

use crate::event_spreading::LockPlayer;

pub fn handle_cinematics(mut players: Query<&mut PlayerState>, clock: Res<Clock>) {
    for mut state in &mut players {
        if let Some(unlock_frame) = state.active_cinematic() {
            if unlock_frame <= clock.frame {
                state.end_cinematic();
            }
        }
    }
}

pub fn start_lock(
    trigger: Trigger<LockPlayer>,
    clock: Res<Clock>,
    mut players: Query<&mut PlayerState>,
) {
    let mut state = players.get_mut(trigger.entity()).unwrap();
    state.start_cinematic(trigger.event().0 + clock.frame);
}
