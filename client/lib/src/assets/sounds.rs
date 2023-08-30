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

pub fn play_queued(mut commands: Commands, mut sounds: ResMut<Sounds>) {
    for source in sounds.queue.drain(..) {
        commands.spawn(AudioBundle {
            source,
            ..default()
        });
    }
}

pub fn get_sound_paths() -> HashMap<SoundEffect, Vec<&'static str>> {
    vec![
        (SoundEffect::Whoosh, vec!["sound_effects/whoosh.ogg"]),
        (SoundEffect::Block, vec!["sound_effects/block.ogg"]),
        (
            SoundEffect::Hit,
            vec![
                "sound_effects/hit1.ogg",
                "sound_effects/hit2.ogg",
                "sound_effects/hit3.ogg",
            ],
        ),
        (
            SoundEffect::Clash,
            vec!["sound_effects/clink1.ogg", "sound_effects/clink2.ogg"],
        ),
    ]
    .into_iter()
    .collect()
}
