use bevy::prelude::*;
use player_state::PlayerState;
use time::Clock;

use crate::meter::Meter;

pub fn stun_recovery(mut players: Query<(&mut PlayerState, &mut Meter)>, clock: Res<Clock>) {
    let mut iter = players.iter_combinations_mut();
    // TODO: May have a problem with item_combinations_mut not giving combinations both ways.
    while let Some([(mut state, _), (_, mut meter)]) = iter.fetch_next() {
        if let Some(unstun_frame) = state.unstun_frame() {
            if unstun_frame <= clock.frame {
                state.recover();
                meter.flush_combo();
            }
        }
    }
}

// TODO:
pub fn ground_recovery() {}
