use bevy::prelude::*;
use characters::{Action, Character};
use player_state::PlayerState;
use types::Facing;

use crate::assets::{AnimationHelper, Sounds};

#[allow(clippy::type_complexity)]
pub fn update_animation(
    mut query: Query<
        (&Character, &mut PlayerState, &Facing, &mut AnimationHelper),
        Or<(Changed<PlayerState>, Changed<Facing>)>,
    >,
) {
    for (character, mut state, facing, mut helper) in query.iter_mut() {
        if let Some(generic) = state.get_generic_animation(*facing) {
            helper.play(
                character
                    .generic_animations
                    .get(&generic)
                    .unwrap()
                    .to_owned(),
            );
        } else if let Some(move_animation) = state
            .drain_matching_actions(|animation| {
                if let Action::Animation(ani) = animation {
                    Some(*ani)
                } else {
                    None
                }
            })
            .last()
        {
            helper.play(move_animation.to_owned());
        } else {
            dbg!("No generic animation nor is a move ongoing?");
        }
    }
}

pub(super) fn update_audio(mut query: Query<&mut PlayerState>, mut sounds: ResMut<Sounds>) {
    for mut state in query.iter_mut() {
        for clip in state.drain_matching_actions(|animation| {
            if let Action::Sound(clip) = animation {
                Some(*clip)
            } else {
                None
            }
        }) {
            sounds.play(clip);
        }
    }
}