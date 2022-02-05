use bevy::prelude::*;

use player_state::PlayerState;

pub fn size_adjustment(mut query: Query<(&mut PlayerState, &mut Sprite, &mut Transform)>) {
    for (state, mut sprite, mut tf) in query.iter_mut() {
        let new_size = state.get_collider_size();
        let old_size = sprite.custom_size.unwrap();

        if old_size != new_size {
            // When size changes, bottom middle is the pivot point
            // Only touch y coordinate
            tf.translation.y += (new_size.y - old_size.y) / 2.0;
            sprite.custom_size = Some(new_size);
        }
    }
}
