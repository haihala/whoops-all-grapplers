use bevy::prelude::*;

use foundation::{Sound, SoundRequest};

#[derive(Debug, Resource, Default)]
pub struct Music {
    stack: Vec<SoundRequest>,
    action: Option<StackAction>,
}

#[derive(Debug)]
enum StackAction {
    Push(Sound),
    Pop,
}

impl Music {
    pub fn push(&mut self, song: Sound) {
        if self.action.is_some() {
            warn!("Pushing to active stack action");
        }
        self.action = Some(StackAction::Push(song));
    }

    pub fn pop(&mut self) {
        if self.action.is_some() {
            warn!("Popping active stack action");
        }
        self.action = Some(StackAction::Pop);
    }
}

#[derive(Debug, Component)]
pub struct MusicMarker(pub Sound);

pub fn setup_music(mut commands: Commands, check_query: Query<&MusicMarker>) {
    if check_query.is_empty() {
        // This should run successfully max once more than once
        commands.trigger(SoundRequest::from(Sound::AnimeBeginnings));
        commands.insert_resource(Music::default());
    }
}

pub fn update_music(
    mut commands: Commands,
    mut music: ResMut<Music>,
    audio_player: Single<(Entity, &MusicMarker)>,
) {
    if let Some(action) = music.action.take() {
        let (ent, marker) = audio_player.into_inner();
        let to_play = match action {
            StackAction::Push(sound) => {
                // Get playing song
                // Store the spot in the stack
                // Play new shit
                music.stack.push(SoundRequest::from(marker.0));
                Some(SoundRequest::from(sound))
            }
            StackAction::Pop => {
                // Get song from the stack
                music.stack.pop()
            }
        };

        if let Some(request) = to_play {
            commands.entity(ent).despawn();
            commands.trigger(request);
        }
    }
}
