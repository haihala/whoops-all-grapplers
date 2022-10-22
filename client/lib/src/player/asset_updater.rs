use bevy::prelude::*;
use characters::{Action, Character};
use core::Facing;
use player_state::PlayerState;

use crate::assets::{AnimationHelper, AnimationRequest, Sounds};

#[allow(clippy::type_complexity)]
pub fn update_animation(
    mut query: Query<
        (&Character, &mut PlayerState, &Facing, &mut AnimationHelper),
        Or<(Changed<PlayerState>, Changed<Facing>)>,
    >,
) {
    for (character, mut state, facing, mut helper) in &mut query {
        if let Some(&(move_animation, frame_skip)) = state
            .drain_matching_actions(|animation| {
                if let Action::AnimationAtFrame(ani, frame) = animation {
                    Some((*ani, *frame))
                } else if let Action::Animation(ani) = animation {
                    Some((*ani, 0))
                } else {
                    None
                }
            })
            .last()
        {
            helper.play(AnimationRequest {
                animation: move_animation,
                offset: frame_skip,
                ..default()
            });
        } else if let Some(generic) = state.get_generic_animation(*facing) {
            let generic_animation = character
                .generic_animations
                .get(&generic)
                .unwrap()
                .to_owned();

            if helper.current != generic_animation && helper.generic_overrideable {
                helper.play(AnimationRequest {
                    animation: generic_animation,
                    generic_overrideable: true,
                    ..default()
                });
            }
        }
    }
}

pub(super) fn update_audio(mut query: Query<&mut PlayerState>, mut sounds: ResMut<Sounds>) {
    for mut state in &mut query {
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
