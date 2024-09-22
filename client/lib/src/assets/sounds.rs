use bevy::prelude::*;
use bevy::utils::HashMap;

use wag_core::SoundEffect;

#[derive(Debug, Resource)]
pub struct Sounds {
    pub handles: HashMap<SoundEffect, Vec<Handle<AudioSource>>>,
}
