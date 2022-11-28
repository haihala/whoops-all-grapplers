use bevy::prelude::*;
use characters::{Action, Character};
use player_state::PlayerState;
use wag_core::{Facing, Players};

use crate::assets::{AnimationHelper, AnimationRequest, Sounds};

#[allow(clippy::type_complexity)]
pub fn update_animation(
    mut query: Query<
        (
            &Character,
            &mut PlayerState,
            &Facing,
            &mut AnimationHelper,
            Entity,
        ),
        Or<(Changed<PlayerState>, Changed<Facing>)>,
    >,
    tfs: Query<&Transform>,
    players: Res<Players>,
) {
    // TODO: This is somewhat faulty as a concept, fix at some point.
    for (character, mut state, facing, mut helper, entity) in &mut query {
        let [active, opponent] = tfs
            .get_many([entity, players.get_other_entity(entity)])
            .unwrap();
        let position_offset = (opponent.translation - active.translation).truncate();

        if let Some(request) = state
            .drain_matching_actions(|action| match action {
                Action::Animation(animation) => Some(AnimationRequest {
                    animation: *animation,
                    ..default()
                }),
                Action::OffsetAnimation(animation) => Some(AnimationRequest {
                    animation: *animation,
                    position_offset,
                    ..default()
                }),
                Action::AnimationAtFrame(animation, frame) => Some(AnimationRequest {
                    animation: *animation,
                    time_offset: *frame,
                    ..default()
                }),
                Action::OffsetAnimationAtFrame(animation, frame) => Some(AnimationRequest {
                    animation: *animation,
                    time_offset: *frame,
                    position_offset,
                    ..default()
                }),
                _ => None,
            })
            .last()
        {
            helper.play(request.to_owned());
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
