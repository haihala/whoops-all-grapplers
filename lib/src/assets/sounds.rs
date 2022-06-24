use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;

use types::SoundEffect;

pub struct Sounds {
    handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>,
}
impl Sounds {
    pub fn new(handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>) -> Sounds {
        Sounds { handles }
    }

    pub fn get(&self, key: SoundEffect) -> Handle<AudioSource> {
        let clips = self.handles.get(&key).unwrap();
        clips[rand::thread_rng().gen_range(0..clips.len())].clone()
    }
}

pub fn get_sound_paths() -> HashMap<SoundEffect, Vec<&'static str>> {
    return vec![(SoundEffect::Whoosh, vec!["sound_effects/whoosh.ogg"])]
        .into_iter()
        .collect();
}
