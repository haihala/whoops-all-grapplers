use bevy::prelude::Resource;

// A resource that exists or doesn't containing info on the ongoing combo if one is ongoing.
#[derive(Debug, Resource)]
pub struct Combo;
