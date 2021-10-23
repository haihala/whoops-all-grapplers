use bevy::math::Vec3;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum RelativeDirection {
    Forward,
    Back,
}

impl Default for RelativeDirection {
    fn default() -> Self {
        RelativeDirection::Forward
    }
}
impl RelativeDirection {
    pub fn as_absolute(&self, base: AbsoluteDirection) -> AbsoluteDirection {
        match self {
            RelativeDirection::Forward => base,
            RelativeDirection::Back => base.inverse(),
        }
    }
    pub fn handle_mirroring(&self, vector: Vec3) -> Vec3 {
        match self {
            RelativeDirection::Forward => vector,
            RelativeDirection::Back => Vec3::new(-vector.x, vector.y, vector.z),
        }
    }
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AbsoluteDirection {
    Right,
    Left,
}
impl Default for AbsoluteDirection {
    fn default() -> Self {
        AbsoluteDirection::Right
    }
}
impl AbsoluteDirection {
    pub fn inverse(&self) -> Self {
        match self {
            AbsoluteDirection::Left => AbsoluteDirection::Right,
            AbsoluteDirection::Right => AbsoluteDirection::Left,
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            match self {
                AbsoluteDirection::Right => 1.0,
                AbsoluteDirection::Left => -1.0,
            },
            0.0,
            0.0,
        )
    }

    pub fn as_relative(&self, pivot: AbsoluteDirection) -> RelativeDirection {
        if *self == pivot {
            RelativeDirection::Forward
        } else {
            RelativeDirection::Back
        }
    }

    pub fn handle_mirroring(&self, vector: Vec3) -> Vec3 {
        match self {
            AbsoluteDirection::Right => vector,
            AbsoluteDirection::Left => Vec3::new(-vector.x, vector.y, vector.z),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mirroring() {
        let left = Vec3::X;
        let right = -Vec3::X;

        assert!(RelativeDirection::Forward.handle_mirroring(left) == left);
        assert!(RelativeDirection::Forward.handle_mirroring(right) == right);
        assert!(RelativeDirection::Back.handle_mirroring(left) == right);
        assert!(RelativeDirection::Back.handle_mirroring(right) == left);
    }
}
