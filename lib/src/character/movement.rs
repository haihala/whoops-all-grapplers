use bevy::prelude::*;

use input_parsing::InputParser;
use player_state::{FreedomLevel, PlayerState};
use types::{RelativeDirection, StickPosition};

pub use moves::universal::{DASH_BACK, DASH_FORWARD};

const PLAYER_CROUCHING_OFFSET: f32 = constants::PLAYER_SPRITE_STANDING_HEIGHT / 2.0;
const PLAYER_STANDING_OFFSET: f32 = constants::PLAYER_SPRITE_CROUCHING_HEIGHT / 2.0;
const PLAYER_CROUCHING_SHIFT: f32 = PLAYER_STANDING_OFFSET - PLAYER_CROUCHING_OFFSET;
const PLAYER_STANDING_SHIFT: f32 = -PLAYER_CROUCHING_SHIFT;

pub fn movement(
    mut query: Query<(
        &mut InputParser,
        &mut PlayerState,
        &mut Sprite,
        &mut Transform,
    )>,
) {
    for (mut reader, mut state, mut sprite, mut tf) in query.iter_mut() {
        if state.freedom_level() >= FreedomLevel::LightBusy && state.is_grounded() {
            let events = reader.get_events();
            // Dashing
            if events.contains(&DASH_FORWARD) {
                reader.consume_event(&DASH_FORWARD);
                state.start_dash(RelativeDirection::Forward);
                continue;
            } else if events.contains(&DASH_BACK) {
                reader.consume_event(&DASH_BACK);
                state.start_dash(RelativeDirection::Back);
                continue;
            }

            // Jumping and walking
            match reader.get_relative_stick_position() {
                StickPosition::N => state.register_jump(None),
                StickPosition::NW => state.register_jump(Some(RelativeDirection::Back)),
                StickPosition::NE => state.register_jump(Some(RelativeDirection::Forward)),
                StickPosition::W => state.walk(RelativeDirection::Back),
                StickPosition::E => state.walk(RelativeDirection::Forward),
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
