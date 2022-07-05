use bevy::prelude::*;

use kits::{Hurtbox, Kit};
use player_state::PlayerState;
use types::Area;

pub fn size_adjustment(mut query: Query<(&mut PlayerState, &mut Sprite, &mut Hurtbox, &Kit)>) {
    for (state, mut sprite, mut hurtbox, kit) in query.iter_mut() {
        let size = kit.get_size(state.is_crouching());
        sprite.custom_size = Some(size);
        **hurtbox = Area::from_center_size(Vec2::Y * size.y / 2.0, size);
    }
}
