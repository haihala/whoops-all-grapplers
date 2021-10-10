use std::time::Instant;

use bevy::utils::HashMap;

use moves::{MotionDefinition, StickTransition};
use types::StickPosition;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Motion {
    heads: HashMap<usize, Instant>,
    transitions: Vec<StickTransition>,
}
impl Default for Motion {
    fn default() -> Self {
        Self {
            heads: Default::default(),
            transitions: Default::default(),
        }
    }
}

impl Motion {
    pub fn clear(&mut self) {
        self.heads.clear();
    }

    pub fn is_done(&self) -> bool {
        self.furthest_head() == self.transitions.len()
    }

    pub fn is_halfway(&self) -> bool {
        self.furthest_head() as f32 >= (self.transitions.len() as f32 / 2.0)
    }

    fn furthest_head(&self) -> usize {
        *self.heads.iter().map(|(head, _)| head).max().unwrap_or(&0)
    }

    pub fn advance(&mut self, old_stick: StickPosition, new_stick: StickPosition) {
        let now = Instant::now();
        let first = self.transitions[0];

        self.heads = self
            .heads
            .clone()
            .into_iter()
            .filter_map(|(at, time)| {
                if at == self.transitions.len() {
                    // Motion is done
                    // Keep looping because the player may be going for another head
                    return Some((at, now));
                }

                let next = self.transitions[at];
                if Self::transition_matches(next, old_stick, new_stick) {
                    Some((at + 1, now))
                } else if time.elapsed().as_secs_f32()
                    > crate::MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS
                {
                    None
                } else {
                    Some((at, time))
                }
            })
            .collect();

        if Self::transition_matches(first, old_stick, new_stick) {
            self.heads.insert(1, now);
        }
    }

    fn transition_matches(
        transition: StickTransition,
        old_stick: StickPosition,
        new_stick: StickPosition,
    ) -> bool {
        if let Some(old_requirement) = transition.0 {
            if old_stick != old_requirement {
                return false;
            }
        }

        transition.1 == new_stick
    }
}
impl From<MotionDefinition> for Motion {
    fn from(definition: MotionDefinition) -> Self {
        Self {
            transitions: definition.transitions,
            ..Default::default()
        }
    }
}
