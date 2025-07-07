use bevy::prelude::*;
use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, EnumIter, Component)]
pub enum CharacterId {
    #[default]
    Ronin,
    CPO,
}
impl FromStr for CharacterId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ronin" => Ok(Self::Ronin),
            "cpo" => Ok(Self::CPO),
            _ => Err(format!("Unknown character: {s}")),
        }
    }
}
impl std::fmt::Display for CharacterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterId::Ronin => write!(f, "ronin"),
            CharacterId::CPO => write!(f, "CPO"),
        }
    }
}
// Used for network
impl From<CharacterId> for u8 {
    fn from(val: CharacterId) -> Self {
        CharacterId::iter()
            .enumerate()
            .find_map(|(i, ch)| if ch == val { Some(i + 1) } else { None })
            .unwrap() as u8
    }
}

impl From<u8> for CharacterId {
    fn from(value: u8) -> Self {
        Self::iter()
            .enumerate()
            .find_map(|(i, ch)| if i as u8 + 1 == value { Some(ch) } else { None })
            .unwrap()
    }
}
#[derive(Debug, Resource)]
pub struct Characters {
    pub p1: CharacterId,
    pub p2: CharacterId,
}

#[derive(Debug, Resource)]
pub struct LocalCharacter(pub CharacterId);
