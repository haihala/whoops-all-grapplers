use std::time::Instant;

use bevy::utils::HashMap;

use crate::StickPosition;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MotionInput {
    heads: HashMap<usize, Instant>,
    done: bool,

    key_points: Vec<StickPosition>,
    autoresets: Vec<StickPosition>,
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
    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn advance(&mut self, stick: StickPosition) {
        if self.done {
            // If we're done, don't bother looping
            return;
        }

        let now = Instant::now();
        let first = self.key_points[0];

        if stick == first {
            self.heads.insert(1, now);
        }

        self.heads = self
            .heads
            .clone()
            .iter()
            .filter_map(|(at, time)| {
                let next = self.key_points[*at];
                if next == stick {
                    if (at + 1) == self.key_points.len() {
                        // Motion is complete
                        self.done = true;
                        None
                    } else {
                        Some((at + 1, now))
                    }
                } else if self.autoresets.contains(&stick)
                    || time.elapsed().as_secs_f32() > crate::MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS
                {
                    None
                } else {
                    Some((*at, *time))
                }
            })
            .collect();
    }

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
