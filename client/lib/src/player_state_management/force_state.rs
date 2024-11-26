use bevy::prelude::*;
use player_state::PlayerState;

use crate::event_spreading::ForceState;

pub fn force_state(trigger: Trigger<ForceState>, mut players: Query<&mut PlayerState>) {
    let mut state = players.get_mut(trigger.entity()).unwrap();
    state.force_state(trigger.event().0);
}
