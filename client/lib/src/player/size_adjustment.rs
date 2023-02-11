use bevy::prelude::*;

use characters::Character;
use player_state::PlayerState;

use crate::physics::Pushbox;

pub fn size_adjustment(mut query: Query<(&mut PlayerState, &mut Pushbox, &Character)>) {
    for (state, mut pushbox, character) in &mut query {
        **pushbox = character.get_pushbox(state.is_crouching());
    }
}
