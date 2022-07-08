use bevy::prelude::*;

use kits::{Hurtbox, Kit};
use player_state::PlayerState;

use crate::physics::Pushbox;

pub fn size_adjustment(mut query: Query<(&mut PlayerState, &mut Pushbox, &mut Hurtbox, &Kit)>) {
    for (state, mut pushbox, mut hurtbox, kit) in query.iter_mut() {
        **hurtbox = kit.get_hurtbox(state.is_crouching());
        **pushbox = kit.get_pushbox(state.is_crouching());
    }
}
