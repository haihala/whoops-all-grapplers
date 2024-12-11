use bevy::prelude::*;

use crate::ActionId;

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Reflect, Default, Hash)]
pub enum ActionCategory {
    Dash,
    Jump,
    Other, // For gi parry and fast fall
    #[default]
    Normal,
    Special,
    Super,
    MegaInterrupt,
    Forced, // For throw recipients
}

#[derive(Debug, Reflect, PartialEq, Eq, Clone, Default, Hash)]
pub enum CancelType {
    #[default]
    Special,
    Super,
    Specific(Vec<ActionId>),
    Anything,
}
