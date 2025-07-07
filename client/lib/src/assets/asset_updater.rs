use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use characters::{AnimationRequest, Character};
use foundation::{CharacterFacing, Players, SoundRequest, Sounds};
use player_state::PlayerState;
use rand::Rng;

use crate::event_spreading::ActivateVoiceline;

use super::{announcer::AnnouncerMarker, music::MusicMarker, AnimationHelper};

pub fn start_animation(
    trigger: Trigger<AnimationRequest>,
    mut query: Query<&mut AnimationHelper>,
    tfs: Query<&Transform>,
    players: Res<Players>,
) {
    let mut helper = query.get_mut(trigger.target()).unwrap();
    let animation_request = trigger.event();

    helper.play(if animation_request.invert {
        // Meant for targets
        let [active, opponent] = tfs
            .get_many([trigger.target(), players.get_other_entity(trigger.target())])
            .unwrap();
        let position_offset = (opponent.translation - active.translation).truncate();
        AnimationRequest {
            position_offset,
            invert: true,
            ..AnimationRequest::from(animation_request.animation)
        }
    } else {
        animation_request.to_owned()
    });
}

#[allow(clippy::type_complexity)]
pub fn update_generic_animation(
    mut query: Query<(
        &Character,
        &PlayerState,
        &CharacterFacing,
        &mut AnimationHelper,
    )>,
) {
    for (character, state, facing, mut helper) in &mut query {
        if let Some(generic) = state.get_generic_animation(facing.absolute) {
            let animation = character
                .generic_animations
                .get(&generic)
                .unwrap()
                .to_owned();

            helper.play_if_new(AnimationRequest {
                looping: true,
                ignore_action_speed: true,
                ..AnimationRequest::from(animation)
            });
        }
    }
}

pub fn play_voiceline(
    trigger: Trigger<ActivateVoiceline>,
    mut commands: Commands,
    chars: Query<&Character>,
) {
    commands.trigger(Into::<SoundRequest>::into(
        chars
            .get(trigger.target())
            .unwrap()
            .get_voiceline(trigger.event().0),
    ));
}

pub fn play_audio(trigger: Trigger<SoundRequest>, mut commands: Commands, sounds: Res<Sounds>) {
    let SoundRequest { sound } = trigger.event();

    let clips = sounds.handles.get(sound).unwrap();
    let source = clips[rand::thread_rng().gen_range(0..clips.len())].clone();

    let mut entity = commands.spawn((
        AudioPlayer(source),
        PlaybackSettings {
            // Shift speed (pitch) by up to about 10% either way
            speed: sound.speed_modifier(),
            volume: Volume::Linear(sound.volume()), // TODO: Supports decibels, may prefer that
            mode: if sound.is_music() {
                PlaybackMode::Loop
            } else {
                PlaybackMode::Despawn
            },
            ..default()
        },
    ));

    if sound.is_announcer() {
        entity.insert((AnnouncerMarker, Name::new("Announcer audio")));
    } else if sound.is_music() {
        entity.insert((MusicMarker(*sound), Name::new("Music")));
    } else {
        entity.insert(Name::new("VFX"));
    }
}

pub fn mirror_models(
    players: Query<(&CharacterFacing, &AnimationHelper)>,
    mut tfs: Query<&mut Transform>,
) {
    for (facing, anim_helper) in &players {
        let root = anim_helper.scene_root;
        let mut root_tf = tfs.get_mut(root).unwrap();
        root_tf.scale.x = facing.visual.mirror_f32(1.0);
    }
}
