use bevy::prelude::*;
use player_state::PlayerState;
use wag_core::{Clock, Combo, Player, Players};

pub fn stun_recovery(
    mut commands: Commands,
    mut query: Query<(&mut PlayerState, &Player)>,
    players: Res<Players>,
    clock: Res<Clock>,
) {
    for (mut state, player) in &mut query {
        if let Some(unstun_frame) = state.unstun_frame() {
            if unstun_frame <= clock.frame {
                state.recover(clock.frame);
                commands
                    .entity(players.get(player.other()))
                    .remove::<Combo>();
            }
        }
    }
}

const QUICK_RISE_DURATION: usize = (wag_core::FPS * 0.5) as usize;

pub fn ground_recovery(
    mut commands: Commands,
    players: Res<Players>,
    mut query: Query<(&mut PlayerState, &Player)>,
    clock: Res<Clock>,
) {
    for (mut state, player) in &mut query {
        if let Some(landing_frame) = state.otg_since() {
            if landing_frame + QUICK_RISE_DURATION <= clock.frame {
                state.recover(clock.frame);
                commands
                    .entity(players.get(player.other()))
                    .remove::<Combo>();
            }
        }
    }
}
