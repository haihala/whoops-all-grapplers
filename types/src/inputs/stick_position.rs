use bevy::prelude::*;

use std::fmt::Debug;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StickPosition {
    NW,
    N,
    NE,
    W,
    Neutral,
    E,
    SW,
    S,
    SE,
}

impl std::fmt::Display for StickPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Default for StickPosition {
    fn default() -> Self {
        StickPosition::Neutral
    }
}
impl From<i32> for StickPosition {
    fn from(numpad: i32) -> Self {
        match numpad {
            1 => StickPosition::SW,
            2 => StickPosition::S,
            3 => StickPosition::SE,
            4 => StickPosition::W,
            5 => StickPosition::Neutral,
            6 => StickPosition::E,
            7 => StickPosition::NW,
            8 => StickPosition::N,
            9 => StickPosition::NE,
            _ => panic!("Invalid numpad to StickPosition conversion"),
        }
    }
}
impl From<IVec2> for StickPosition {
    fn from(item: IVec2) -> Self {
        let matrix = vec![
            vec![StickPosition::SW, StickPosition::S, StickPosition::SE],
            vec![StickPosition::W, StickPosition::Neutral, StickPosition::E],
            vec![StickPosition::NW, StickPosition::N, StickPosition::NE],
        ];

        matrix[(item.y + 1) as usize][(item.x + 1) as usize]
    }
}
// Can't implement traits for bevy types
#[allow(clippy::from_over_into)]
impl Into<IVec2> for StickPosition {
    fn into(self) -> IVec2 {
        match self {
            StickPosition::NW => (-1, 1).into(),
            StickPosition::N => (0, 1).into(),
            StickPosition::NE => (1, 1).into(),
            StickPosition::W => (-1, 0).into(),
            StickPosition::Neutral => (0, 0).into(),
            StickPosition::E => (1, 0).into(),
            StickPosition::SW => (-1, -1).into(),
            StickPosition::S => (0, -1).into(),
            StickPosition::SE => (1, -1).into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_ivec_stickposition_conversions() {
        for sp1 in StickPosition::iter() {
            let ivec: IVec2 = sp1.into();
            let sp2: StickPosition = ivec.into();
            assert!(sp1 == sp2)
        }
    }
}
