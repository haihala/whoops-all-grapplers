use bevy::prelude::*;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect, FromReflect,
)]
pub enum Icon {
    #[default]
    Default,
}
