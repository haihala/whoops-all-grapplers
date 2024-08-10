use bevy::prelude::*;

use characters::Character;
use player_state::PlayerState;
use wag_core::{Area, Facing};

use crate::movement::Pushbox;

pub fn size_adjustment(mut query: Query<(&mut PlayerState, &mut Pushbox, &Character, &Facing)>) {
    for (state, mut pushbox, character, facing) in &mut query {
        let pb = state.get_pushbox(character);
        **pushbox = Area::from_center_size(facing.mirror_vec2(pb.center()), pb.size());
    }
}
