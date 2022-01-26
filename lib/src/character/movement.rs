use bevy::prelude::*;

use input_parsing::InputParser;
use player_state::{PlayerState, PLAYER_CROUCHING_SHIFT, PLAYER_STANDING_SHIFT};
use types::{LRDirection, StickPosition};

pub use moves::universal::{DASH_BACK, DASH_FORWARD};

pub fn movement(mut query: Query<(&InputParser, &mut PlayerState, &mut Sprite, &mut Transform)>) {
    for (reader, mut state, mut sprite, mut tf) in query.iter_mut() {
        if state.is_grounded() {
            match reader.get_absolute_stick_position() {
                StickPosition::N => state.jump(None),
                StickPosition::NW => state.jump(Some(LRDirection::Left)),
                StickPosition::NE => state.jump(Some(LRDirection::Right)),
                StickPosition::W => state.walk(LRDirection::Left),
                StickPosition::E => state.walk(LRDirection::Right),
                StickPosition::SW | StickPosition::S | StickPosition::SE => state.crouch(),
                StickPosition::Neutral => state.stand(),
            }

            let new_size = state.get_collider_size();
            if sprite.size != new_size {
                if sprite.size.y > new_size.y {
                    // Crouching
                    tf.translation.y += PLAYER_CROUCHING_SHIFT;
                } else {
                    // Standing up
                    tf.translation.y += PLAYER_STANDING_SHIFT;
                }
                sprite.size = new_size;
            }
        }
    }
}
