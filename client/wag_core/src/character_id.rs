use bevy::prelude::*;
use std::str::FromStr;

use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, EnumIter, Component)]
pub enum CharacterId {
    Mizku,
    #[default]
    Dummy,
}
impl FromStr for CharacterId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dummy" => Ok(Self::Dummy),
            "mizku" => Ok(Self::Mizku),
            _ => Err(format!("Unknown character: {}", s)),
        }
    }
}
impl std::fmt::Display for CharacterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterId::Dummy => write!(f, "dummy"),
            CharacterId::Mizku => write!(f, "mizku"),
        }
    }
}

#[derive(Debug, Resource)]
pub struct Characters {
    pub p1: CharacterId,
    pub p2: CharacterId,
}
