use bevy::prelude::*;

#[derive(
    Reflect, FromReflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub enum ItemId {
    Gi,
    Gun,
    HandMeDownKen,
    Drugs,

    #[default]
    Default,
}
