use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect)]
pub enum SoundEffect {
    Whoosh,
    Clash,
    Block,
    Hit,
    #[default]
    Silence,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum VisualEffect {
    Clash,
    Block,
    Hit,
}
