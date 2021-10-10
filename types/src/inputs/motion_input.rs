use std::time::Instant;

use bevy::utils::HashMap;

use crate::StickPosition;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MotionInput {
    pub heads: HashMap<usize, Instant>,
    pub done: bool,

    pub key_points: Vec<StickPosition>,
    pub autoresets: Vec<StickPosition>,
}
impl Default for MotionInput {
    fn default() -> Self {
        Self {
            heads: Default::default(),
            done: false,
            key_points: Default::default(),
            autoresets: Default::default(),
        }
    }
}

impl MotionInput {
    pub fn clear(&mut self) {
        self.heads.clear();
        self.done = false;
    }
}
impl From<Vec<i32>> for MotionInput {
    fn from(requirements: Vec<i32>) -> Self {
        Self {
            key_points: requirements.into_iter().map(StickPosition::from).collect(),
            ..Default::default()
        }
    }
}
impl From<(Vec<i32>, Vec<i32>)> for MotionInput {
    fn from(specs: (Vec<i32>, Vec<i32>)) -> Self {
        Self {
            key_points: specs.0.into_iter().map(StickPosition::from).collect(),
            autoresets: specs.1.into_iter().map(StickPosition::from).collect(),
            ..Default::default()
        }
    }
}
