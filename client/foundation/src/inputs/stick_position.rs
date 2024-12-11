use bevy::prelude::*;

use std::fmt::Debug;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Reflect)]
pub enum StickPosition {
    NW,
    N,
    NE,
    W,
    #[default]
    Neutral,
    E,
    SW,
    S,
    SE,
}
impl StickPosition {
    pub fn mirror(self) -> Self {
        let vector: IVec2 = self.into();
        IVec2::new(-vector.x, vector.y).into()
    }

    pub fn as_vec2(self) -> Vec2 {
        let vector: IVec2 = self.into();
        vector.as_vec2()
    }
}
impl std::fmt::Display for StickPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
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
impl From<StickPosition> for i32 {
    fn from(sp: StickPosition) -> Self {
        match sp {
            StickPosition::SW => 1,
            StickPosition::S => 2,
            StickPosition::SE => 3,
            StickPosition::W => 4,
            StickPosition::Neutral => 5,
            StickPosition::E => 6,
            StickPosition::NW => 7,
            StickPosition::N => 8,
            StickPosition::NE => 9,
        }
    }
}
impl From<IVec2> for StickPosition {
    fn from(item: IVec2) -> Self {
        let matrix = [
            vec![StickPosition::SW, StickPosition::S, StickPosition::SE],
            vec![StickPosition::W, StickPosition::Neutral, StickPosition::E],
            vec![StickPosition::NW, StickPosition::N, StickPosition::NE],
        ];

        matrix[(item.y + 1) as usize][(item.x + 1) as usize]
    }
}
impl From<StickPosition> for IVec2 {
    fn from(sp: StickPosition) -> Self {
        match sp {
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

    #[test]
    fn test_i32_stickposition_conversions() {
        for sp1 in StickPosition::iter() {
            let int: i32 = sp1.into();
            let sp2: StickPosition = int.into();
            assert!(sp1 == sp2)
        }
    }

    #[test]
    fn test_mirroring() {
        for horizontally_neutral in [StickPosition::N, StickPosition::Neutral, StickPosition::S] {
            assert_eq!(horizontally_neutral.clone(), horizontally_neutral.mirror());
        }

        for (left, right) in [
            (StickPosition::NW, StickPosition::NE),
            (StickPosition::W, StickPosition::E),
            (StickPosition::SW, StickPosition::SE),
        ] {
            assert_eq!(left.mirror(), right);
            assert_eq!(left, right.mirror());

            assert_eq!(left.mirror().mirror(), left);
            assert_eq!(right.mirror().mirror(), right);
        }
    }
}
