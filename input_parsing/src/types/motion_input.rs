use std::time::Instant;

use bevy::prelude::*;

use crate::StickPosition;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct MotionInput {
    index: usize,
    previous_event_time: Option<Instant>,

    key_points: Vec<super::StickPosition>,
}
impl MotionInput {
    pub fn is_done(&self) -> bool {
        self.index == self.key_points.len() - 1
    }

    pub fn is_started(&self) -> bool {
        self.index != 0
    }

    pub fn bump(&mut self) {
        self.index += 1;
        self.previous_event_time = Some(Instant::now());
    }

    pub fn next_requirement(&self, flipped: bool) -> StickPosition {
        let nth = self.key_points.get(self.index).unwrap().clone();

        if flipped {
            let as_vec: IVec2 = nth.into();
            super::StickPosition::from(IVec2::new(-as_vec.x, as_vec.y))
        } else {
            nth
        }
    }

    pub fn handle_expiration(&mut self) {
        if self.previous_event_time.is_some()
            && self.previous_event_time.unwrap().elapsed().as_secs_f32()
                > crate::MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS
        {
            self.clear();
        }
    }

    pub fn clear(&mut self) {
        self.index = 0;
        self.previous_event_time = None;
    }
}
impl From<Vec<i32>> for MotionInput {
    fn from(requirements: Vec<i32>) -> Self {
        MotionInput {
            key_points: requirements
                .into_iter()
                .map(super::StickPosition::from)
                .collect(),
            index: 0,
            previous_event_time: None,
        }
    }
}
