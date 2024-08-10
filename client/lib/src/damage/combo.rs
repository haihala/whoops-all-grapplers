use bevy::prelude::Component;

// A resource that exists or doesn't containing info on the ongoing combo if one is ongoing.
#[derive(Debug, Component)]
pub struct Combo {
    pub hits: usize,
}
