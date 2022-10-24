use bevy::prelude::*;
use player_state::PlayerState;
use time::Clock;

use crate::damage::Combo;

pub fn stun_recovery(
    mut commands: Commands,
    combo: Option<Res<Combo>>,
    mut players: Query<&mut PlayerState>,
    clock: Res<Clock>,
) {
    let mut stunned_player = false;

    for mut state in &mut players {
        if let Some(unstun_frame) = state.unstun_frame() {
            if unstun_frame <= clock.frame {
                state.recover(clock.frame);
            }
        }

        if state.stunned() {
            stunned_player = true;
        }
    }

    if combo.is_some() && !stunned_player {
        commands.remove_resource::<Combo>();
    }
}

const QUICK_RISE_DURATION: usize = (wag_core::FPS * 0.5) as usize;

pub fn ground_recovery(mut players: Query<&mut PlayerState>, clock: Res<Clock>) {
    for mut state in &mut players {
        if let Some(landing_frame) = state.otg_since() {
            if landing_frame + QUICK_RISE_DURATION <= clock.frame {
                state.recover(clock.frame);
            }
        }
    }
}
