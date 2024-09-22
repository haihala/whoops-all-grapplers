use bevy::prelude::*;
use player_state::PlayerState;

use crate::event_spreading::ForceStand;

pub fn force_stand(trigger: Trigger<ForceStand>, mut players: Query<&mut PlayerState>) {
    let mut state = players.get_mut(trigger.entity()).unwrap();
    state.force_stand()
}
