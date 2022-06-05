use bevy::prelude::*;

use player_state::PlayerState;

pub fn size_adjustment(mut query: Query<(&mut PlayerState, &mut Sprite)>) {
    for (state, mut sprite) in query.iter_mut() {
        sprite.custom_size = Some(state.get_collider_size());
    }
}
