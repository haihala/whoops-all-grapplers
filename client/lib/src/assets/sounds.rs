use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;

use wag_core::SoundEffect;

#[derive(Debug, Resource)]
pub struct Sounds {
    handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>,
    queue: Vec<Handle<AudioSource>>,
}
impl Sounds {
    pub fn new(handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>) -> Sounds {
        Sounds {
            handles,
            queue: vec![],
        }
    }

    pub fn play(&mut self, key: SoundEffect) {
        if let Some(clips) = self.handles.get(&key) {
            let clip = clips[rand::thread_rng().gen_range(0..clips.len())].clone();
            self.queue.push(clip);
        }
    }
}

pub fn play_queued(
    mut commands: Commands,
    mut sounds: ResMut<Sounds>,
    spawned: Query<(Entity, &AudioSink)>,
) {
    for source in sounds.queue.drain(..) {
        commands.spawn(AudioBundle {
            source,
            ..default()
        });
    }
    for (entity, sink) in &spawned {
        if sink.empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
