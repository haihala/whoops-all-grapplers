use bevy::prelude::*;
use characters::Character;

use crate::assets::AnimationHelper;

// TODO: Add state dependency here
pub fn update_animation(mut query: Query<(&Character, &mut AnimationHelper)>) {
    for (character, mut helper) in query.iter_mut() {
        helper.play(character.idle_animation);
    }
}
