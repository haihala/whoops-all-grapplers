use bevy::prelude::*;

use characters::{Character, Hurtbox};
use player_state::PlayerState;

use crate::physics::Pushbox;

pub fn size_adjustment(
    mut query: Query<(&mut PlayerState, &mut Pushbox, &mut Hurtbox, &Character)>,
) {
    for (state, mut pushbox, mut hurtbox, character) in query.iter_mut() {
        **hurtbox = character.get_hurtbox(state.is_crouching());
        **pushbox = character.get_pushbox(state.is_crouching());
    }
}
