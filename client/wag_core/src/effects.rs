use bevy::prelude::*;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect, FromReflect,
)]
pub enum SoundEffect {
    Whoosh,
    Clash,
    Block,
    Hit,
    #[default]
    Silence,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisualEffect {
    Clash,
    Block,
    Hit,
}
