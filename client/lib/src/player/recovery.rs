use bevy::prelude::*;
use player_state::PlayerState;
use time::Clock;

pub fn stun_recovery(mut players: Query<&mut PlayerState>, clock: Res<Clock>) {
    for mut state in &mut players {
        if let Some(unstun_frame) = state.unstun_frame() {
            if unstun_frame <= clock.frame {
                state.recover(clock.frame);
            }
        }
    }
}

// TODO:
pub fn ground_recovery() {}
