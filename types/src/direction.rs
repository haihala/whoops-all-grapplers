use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::StickPosition;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug, Component)]
pub enum LRDirection {
    Right,
    Left,
}
impl Default for LRDirection {
    fn default() -> Self {
        LRDirection::Right
    }
}
impl LRDirection {
    pub fn opposite(&self) -> LRDirection {
        match self {
            LRDirection::Right => LRDirection::Left,
            LRDirection::Left => LRDirection::Right,
        }
    }

    pub fn from_flipped(flipped: bool) -> LRDirection {
        if flipped {
            LRDirection::Left
        } else {
            LRDirection::Right
        }
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        *self = if flipped {
            LRDirection::Left
        } else {
            LRDirection::Right
        };
    }

    pub fn to_flipped(&self) -> bool {
        match self {
            LRDirection::Right => false,
            LRDirection::Left => true,
        }
    }

    pub fn to_signum(&self) -> f32 {
        match self {
            LRDirection::Right => 1.0,
            LRDirection::Left => -1.0,
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        match self {
            LRDirection::Right => Vec3::X,
            LRDirection::Left => -Vec3::X,
        }
    }

    pub fn mirror_vec(&self, vector: Vec3) -> Vec3 {
        match self {
            LRDirection::Right => vector,
            LRDirection::Left => Vec3::new(-vector.x, vector.y, vector.z),
        }
    }

    pub fn mirror_stick(&self, stick: StickPosition) -> StickPosition {
        let vector: IVec2 = stick.into();

        match self {
            LRDirection::Right => vector,
            LRDirection::Left => IVec2::new(-vector.x, vector.y),
        }
        .into()
    }

    pub fn mirror_f32(&self, number: f32) -> f32 {
        match self {
            LRDirection::Right => number,
            LRDirection::Left => -number,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mirroring_vec() {
        let left = Vec3::X;
        let right = -Vec3::X;

        assert!(LRDirection::Right.mirror_vec(left) == left);
        assert!(LRDirection::Right.mirror_vec(right) == right);
        assert!(LRDirection::Left.mirror_vec(left) == right);
        assert!(LRDirection::Left.mirror_vec(right) == left);
    }

    #[test]
    fn test_mirroring_stick() {
        assert!(LRDirection::Right.mirror_stick(StickPosition::E) == StickPosition::E);
        assert!(LRDirection::Right.mirror_stick(StickPosition::Neutral) == StickPosition::Neutral);
        assert!(LRDirection::Right.mirror_stick(StickPosition::W) == StickPosition::W);

        assert!(LRDirection::Left.mirror_stick(StickPosition::E) == StickPosition::W);
        assert!(LRDirection::Left.mirror_stick(StickPosition::Neutral) == StickPosition::Neutral);
        assert!(LRDirection::Left.mirror_stick(StickPosition::W) == StickPosition::E);
    }
}
