use bevy::math::Vec3;
use bevy_inspector_egui::Inspectable;

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
    pub fn to_vec3(&self) -> Vec3 {
        match self {
            AbsoluteDirection::Right => Vec3::X,
            AbsoluteDirection::Left => -Vec3::X,
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

        assert!(AbsoluteDirection::Right.handle_mirroring(left) == left);
        assert!(AbsoluteDirection::Right.handle_mirroring(right) == right);
        assert!(AbsoluteDirection::Left.handle_mirroring(left) == right);
        assert!(AbsoluteDirection::Left.handle_mirroring(right) == left);
    }
}
