use bevy::prelude::*;
use characters::{ActionEvent, ActionEvents, AnimationRequest, Character};
use player_state::PlayerState;
use wag_core::{Facing, Players};

use super::{AnimationHelper, Sounds, Vfx};

#[allow(clippy::type_complexity)]
pub fn update_animation(
    mut query: Query<
        (
            &Character,
            &PlayerState,
            &ActionEvents,
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
    for (character, state, events, facing, mut helper, entity) in &mut query {
        let [active, opponent] = tfs
            .get_many([entity, players.get_other_entity(entity)])
            .unwrap();
        let position_offset = (opponent.translation - active.translation).truncate();
        if let Some(req) = events
            .get_matching_events(|action| match action {
                ActionEvent::Animation(animation_request) => {
                    Some(if animation_request.invert {
                        // Meant for targets
                        AnimationRequest {
                            animation: animation_request.animation,
                            position_offset,
                            invert: true,
                            ..default()
                        }
                    } else {
                        animation_request.to_owned()
                    })
                }
                _ => None,
            })
            .last()
        {
            helper.play(req.to_owned());
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

pub fn update_audio(mut query: Query<&ActionEvents>, mut sounds: ResMut<Sounds>) {
    for events in &mut query {
        for clip in events.get_matching_events(|animation| {
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

pub fn update_vfx(mut query: Query<(&ActionEvents, &Transform)>, mut effects: ResMut<Vfx>) {
    for (events, tf) in &mut query {
        for mut request in events.get_matching_events(|animation| {
            if let ActionEvent::VisualEffect(vfx) = animation {
                Some(vfx.clone())
            } else {
                None
            }
        }) {
            request.position += tf.translation;
            effects.spawn(request);
        }
    }
}
