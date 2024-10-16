use bevy::prelude::*;

#[derive(Reflect, PartialEq, Eq, Clone, Copy, Debug, Component, Default, Hash)]
pub enum Facing {
    #[default]
    Right,
    Left,
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

    pub fn mirror_f32(&self, number: f32) -> f32 {
        match self {
            Facing::Right => number,
            Facing::Left => -number,
        }
    }

    pub fn mirror_vec3(&self, vector: Vec3) -> Vec3 {
        match self {
            Facing::Right => vector,
            Facing::Left => Vec3::new(-vector.x, vector.y, vector.z),
        }
    }

    pub fn mirror_vec2(&self, vector: Vec2) -> Vec2 {
        match self {
            Facing::Right => vector,
            Facing::Left => Vec2::new(-vector.x, vector.y),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_opposite() {
        assert!(Facing::Right.opposite() == Facing::Left);
        assert!(Facing::Left.opposite() == Facing::Right);
    }

    #[test]
    fn test_mirroring_f32() {
        assert!(Facing::Right.mirror_f32(10.0) == 10.0);
        assert!(Facing::Right.mirror_f32(-10.0) == -10.0);
        assert!(Facing::Left.mirror_f32(10.0) == -10.0);
        assert!(Facing::Left.mirror_f32(-10.0) == 10.0);
    }

    #[test]
    fn test_mirroring_vec() {
        let left = Vec3::X;
        let right = -Vec3::X;

        assert!(Facing::Right.mirror_vec3(left) == left);
        assert!(Facing::Right.mirror_vec3(right) == right);
        assert!(Facing::Left.mirror_vec3(left) == right);
        assert!(Facing::Left.mirror_vec3(right) == left);
    }
}
