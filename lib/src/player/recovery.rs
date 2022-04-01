use bevy::prelude::*;
use player_state::PlayerState;
use time::Clock;

use crate::resources::Meter;

pub fn stun_recovery(mut players: Query<(&mut PlayerState, &mut Meter)>, clock: Res<Clock>) {
    let mut iter = players.iter_combinations_mut();
    while let Some([(mut state1, mut meter1), (mut state2, mut meter2)]) = iter.fetch_next() {
        handle_recovery(clock.frame, &mut state1, &mut meter2);
        handle_recovery(clock.frame, &mut state2, &mut meter1);
    }
}

fn handle_recovery(frame: usize, state: &mut PlayerState, meter: &mut Meter) {
    if let Some(unstun_frame) = state.unstun_frame() {
        if unstun_frame <= frame {
            state.recover();
            meter.flush_combo();
        }
    }
}

// TODO:
pub fn ground_recovery() {}
