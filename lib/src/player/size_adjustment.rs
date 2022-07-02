use bevy::prelude::*;

use kits::Kit;
use player_state::PlayerState;

pub fn size_adjustment(mut query: Query<(&mut PlayerState, &mut Sprite, &Kit)>) {
    for (state, mut sprite, kit) in query.iter_mut() {
        sprite.custom_size = Some(kit.get_size(state.is_crouching()));
    }
}
