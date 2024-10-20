use bevy::{audio::Volume, prelude::*};
use characters::{AnimationRequest, Character};
use player_state::PlayerState;
use rand::Rng;
use wag_core::{Facing, Players};

use crate::event_spreading::{PlaySound, StartAnimation};

use super::{announcer::AnnouncerMarker, AnimationHelper, Sounds};

pub fn start_animation(
    trigger: Trigger<StartAnimation>,
    mut query: Query<&mut AnimationHelper>,
    tfs: Query<&Transform>,
    players: Res<Players>,
) {
    let mut helper = query.get_mut(trigger.entity()).unwrap();
    let animation_request = trigger.event().0;

    helper.play(if animation_request.invert {
        // Meant for targets
        let [active, opponent] = tfs
            .get_many([trigger.entity(), players.get_other_entity(trigger.entity())])
            .unwrap();
        let position_offset = (opponent.translation - active.translation).truncate();
        AnimationRequest {
            animation: animation_request.animation,
            position_offset,
            invert: true,
            ..default()
        }
    } else {
        animation_request.to_owned()
    });
}

#[allow(clippy::type_complexity)]
pub fn update_generic_animation(
    mut query: Query<
        (&Character, &PlayerState, &Facing, &mut AnimationHelper),
        Or<(Changed<PlayerState>, Changed<Facing>)>,
    >,
) {
    for (character, state, facing, mut helper) in &mut query {
        if let Some(generic) = state.get_generic_animation(*facing) {
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

pub fn play_audio(trigger: Trigger<PlaySound>, mut commands: Commands, sounds: Res<Sounds>) {
    let effect = trigger.event().0;

    let clips = sounds.handles.get(&effect).unwrap();

    let source = clips[rand::thread_rng().gen_range(0..clips.len())].clone();
    let mut entity = commands.spawn(AudioBundle {
        source,
        settings: PlaybackSettings {
            // Shift speed (pitch) by up to about 10% either way
            speed: rand::thread_rng().gen_range(0.9..1.1),
            volume: Volume::new(effect.volume()),
            ..default()
        },
    });

    if effect.is_announcer() {
        entity.insert(AnnouncerMarker);
    }
}

pub fn clear_empty_audio_players(mut commands: Commands, spawned: Query<(Entity, &AudioSink)>) {
    for (entity, sink) in &spawned {
        if sink.empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
