use bevy::prelude::*;
use characters::{ActionEvent, Character};
use player_state::PlayerState;
use wag_core::{Clock, Facing, Players};

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
    clock: Res<Clock>,
) {
    // TODO: This is somewhat faulty as a concept, fix at some point.
    for (character, mut state, facing, mut helper, entity) in &mut query {
        let base_offset = state
            .last_breakpoint_frame()
            .map(|frame| clock.frame - frame)
            .unwrap_or_default();

        let [active, opponent] = tfs
            .get_many([entity, players.get_other_entity(entity)])
            .unwrap();
        let position_offset = (opponent.translation - active.translation).truncate();
        if let Some(req) = state
            .drain_matching_actions(|action| match action {
                ActionEvent::Animation(animation) => Some(AnimationRequest {
                    animation: *animation,
                    ..default()
                }),
                ActionEvent::RecipientAnimation(animation) => Some(AnimationRequest {
                    animation: *animation,
                    position_offset,
                    invert: true,
                    ..default()
                }),
                _ => None,
            })
            .last()
        {
            let mut request = req.to_owned();
            request.time_offset += base_offset;
            helper.play(request);
        } else if let Some(generic) = state.get_generic_animation(*facing) {
            let animation = character
                .generic_animations
                .get(&generic)
                .unwrap()
                .to_owned();

            helper.play_if_new(AnimationRequest {
                animation,
                looping: true,
                ignore_action_speed: true,
                ..default()
            });
        }
    }
}

pub(super) fn update_audio(mut query: Query<&mut PlayerState>, mut sounds: ResMut<Sounds>) {
    for mut state in &mut query {
        for clip in state.drain_matching_actions(|animation| {
            if let ActionEvent::Sound(clip) = animation {
                Some(*clip)
            } else {
                None
            }
        }) {
            sounds.play(clip);
        }
    }
}
