use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::StickPosition;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug, Component)]
pub enum Facing {
    Right,
    Left,
}
impl Default for Facing {
    fn default() -> Self {
        Facing::Right
    }
}
impl Facing {
    #[must_use]
    pub fn opposite(self) -> Facing {
        match self {
            Facing::Right => Facing::Left,
            Facing::Left => Facing::Right,
        }
    }

    pub fn from_flipped(flipped: bool) -> Facing {
        if flipped {
            Facing::Left
        } else {
            Facing::Right
        }
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        *self = if flipped { Facing::Left } else { Facing::Right };
    }

    pub fn to_flipped(&self) -> bool {
        match self {
            Facing::Right => false,
            Facing::Left => true,
        }
    }

    pub fn to_signum(&self) -> f32 {
        match self {
            Facing::Right => 1.0,
            Facing::Left => -1.0,
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        match self {
            Facing::Right => Vec3::X,
            Facing::Left => -Vec3::X,
        }
    }

    pub fn mirror_vec(&self, vector: Vec3) -> Vec3 {
        match self {
            Facing::Right => vector,
            Facing::Left => Vec3::new(-vector.x, vector.y, vector.z),
        }
    }

    pub fn mirror_stick(&self, stick: StickPosition) -> StickPosition {
        let vector: IVec2 = stick.into();

        match self {
            Facing::Right => vector,
            Facing::Left => IVec2::new(-vector.x, vector.y),
        }
        .into()
    }

    pub fn mirror_f32(&self, number: f32) -> f32 {
        match self {
            Facing::Right => number,
            Facing::Left => -number,
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

        assert!(Facing::Right.mirror_vec(left) == left);
        assert!(Facing::Right.mirror_vec(right) == right);
        assert!(Facing::Left.mirror_vec(left) == right);
        assert!(Facing::Left.mirror_vec(right) == left);
    }

    #[test]
    fn test_mirroring_stick() {
        assert!(Facing::Right.mirror_stick(StickPosition::E) == StickPosition::E);
        assert!(Facing::Right.mirror_stick(StickPosition::Neutral) == StickPosition::Neutral);
        assert!(Facing::Right.mirror_stick(StickPosition::W) == StickPosition::W);

        assert!(Facing::Left.mirror_stick(StickPosition::E) == StickPosition::W);
        assert!(Facing::Left.mirror_stick(StickPosition::Neutral) == StickPosition::Neutral);
        assert!(Facing::Left.mirror_stick(StickPosition::W) == StickPosition::E);
    }
}
