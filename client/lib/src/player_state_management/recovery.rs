use bevy::prelude::*;
use foundation::{CharacterClock, Combo, Player, Players, FPS};
use player_state::PlayerState;

pub fn stun_recovery(
    mut query: Query<(&mut PlayerState, &Player, &CharacterClock)>,
    mut combos: Query<&mut Combo>,
    players: Res<Players>,
) {
    for (mut state, player, clock) in &mut query {
        if let Some(unstun_frame) = state.unstun_frame() {
            if unstun_frame <= clock.frame {
                state.recover(clock.frame);
                combos.get_mut(players.get(player.other())).unwrap().reset();
            }
        }
    }
}

const QUICK_RISE_DURATION: usize = (FPS * 0.5) as usize;

pub fn ground_recovery(
    mut combos: Query<&mut Combo>,
    players: Res<Players>,
    mut query: Query<(&mut PlayerState, &Player, &CharacterClock)>,
) {
    for (mut state, player, clock) in &mut query {
        if let Some(landing_frame) = state.otg_since() {
            if landing_frame + QUICK_RISE_DURATION <= clock.frame {
                state.recover(clock.frame);
                combos.get_mut(players.get(player.other())).unwrap().reset();
            }
        }
    }
}
