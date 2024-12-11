use bevy::prelude::*;
use bevy::utils::HashMap;

use foundation::SoundEffect;

#[derive(Debug, Resource)]
pub struct Sounds {
    pub handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>,
}
