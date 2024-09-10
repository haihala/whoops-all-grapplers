use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;

use wag_core::SoundEffect;

#[derive(Debug, Resource)]
pub struct Sounds {
    handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>,
    queue: Vec<SoundEffect>,
}
impl Sounds {
    pub fn new(handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>) -> Sounds {
        Sounds {
            handles,
            queue: vec![],
        }
    }

    pub fn play(&mut self, key: SoundEffect) {
        self.queue.push(key);
    }
}

pub fn play_queued(
    mut commands: Commands,
    mut sounds: ResMut<Sounds>,
    spawned: Query<(Entity, &AudioSink)>,
) {
    for effect in sounds.queue.drain(..).collect::<Vec<_>>().into_iter() {
        let Some(clips) = sounds.handles.get(&effect) else {
            continue;
        };
        let source = clips[rand::thread_rng().gen_range(0..clips.len())].clone();
        commands.spawn(AudioBundle {
            source,
            settings: PlaybackSettings {
                // Shift speed (pitch) by up to about 10% either way
                speed: rand::thread_rng().gen_range(0.9..1.1),
                volume: Volume::new(effect.volume()),
                ..default()
            },
        });
    }
    for (entity, sink) in &spawned {
        if sink.empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
