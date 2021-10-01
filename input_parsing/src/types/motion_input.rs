use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct MotionInput {
    key_points: Vec<super::StickPosition>,
}
impl MotionInput {
    fn forward(&self) -> Box<dyn Iterator<Item = super::StickPosition>> {
        Box::new(self.key_points.clone().into_iter())
    }

    fn backward(&self) -> Box<dyn Iterator<Item = super::StickPosition>> {
        Box::new(
            self.key_points
                .clone()
                .into_iter()
                .map(super::StickPosition::into)
                .map(|v: IVec2| (-v.x, v.y)) // Invert X axis here
                .map(IVec2::from)
                .map(super::StickPosition::from),
        )
    }

    pub fn requirements(&self, flipped: bool) -> Box<dyn Iterator<Item = super::StickPosition>> {
        if flipped {
            self.backward()
        } else {
            self.forward()
        }
    }
}
impl From<Vec<i32>> for MotionInput {
    fn from(requirements: Vec<i32>) -> Self {
        MotionInput {
            key_points: requirements
                .into_iter()
                .map(super::StickPosition::from)
                .collect(),
        }
    }
}
