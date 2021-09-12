use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpecialMoveInput {
    key_points: Vec<super::StickPosition>,
}

impl SpecialMoveInput {
    fn from_numpad(requirements: Vec<i32>) -> SpecialMoveInput {
        SpecialMoveInput {
            key_points: requirements
                .into_iter()
                .map(super::StickPosition::from)
                .collect(),
        }
    }

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
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SpecialMoveName {
    QuarterCircleForward,
    QuarterCircleBack,
}

pub type MotionMapping = HashMap<SpecialMoveName, SpecialMoveInput>;

pub fn get_special_move_name_mappings() -> MotionMapping {
    let mappings: MotionMapping = [
        (
            SpecialMoveName::QuarterCircleForward,
            SpecialMoveInput::from_numpad(vec![2, 3, 6]),
        ),
        (
            SpecialMoveName::QuarterCircleBack,
            SpecialMoveInput::from_numpad(vec![2, 1, 4]),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    mappings
}
