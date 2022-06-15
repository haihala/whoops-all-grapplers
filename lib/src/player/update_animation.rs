use bevy::prelude::*;
use kits::Kit;

use crate::assets::AnimationHelper;

// TODO: Add state dependency here
pub fn update_animation(mut query: Query<(&Kit, &mut AnimationHelper)>) {
    for (kit, mut helper) in query.iter_mut() {
        helper.play(kit.idle_animation);
    }
}
