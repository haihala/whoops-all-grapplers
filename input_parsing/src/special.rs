use bevy::utils::{HashMap, Instant};

use moves::{SpecialDefinition, StickTransition};
use types::{GameButton, StickPosition};

use crate::helper_types::Diff;

#[derive(Default, Clone)]
pub struct Special {
    heads: HashMap<usize, Instant>,
    transitions: Vec<StickTransition>,
    required_button: Option<GameButton>,

    button_received: bool,
}
impl Special {
    pub fn clear(&mut self) {
        self.heads.clear();

        self.button_received = self.required_button.is_none();
    }

    pub fn is_done(&self) -> bool {
        self.furthest_head() == self.transitions.len() && self.button_received
    }

    pub fn advance(&mut self, diff: &Diff, old_stick: StickPosition) {
        if let Some(new_stick) = diff.stick_move {
            assert!(
                old_stick != new_stick,
                "old={}, new={}",
                old_stick,
                new_stick,
            );
            self.advance_motion(old_stick, new_stick);
        }

        if !self.button_received
            && self.required_button.is_some()
            && self.is_halfway()
            && diff.pressed_contains(&self.required_button.unwrap())
        {
            self.button_received = true;
        }
    }

    fn advance_motion(&mut self, old_stick: StickPosition, new_stick: StickPosition) {
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

                if time.elapsed().as_secs_f32() > constants::MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS
                {
                    None
                } else if Self::transition_matches(next, old_stick, new_stick) {
                    Some((at + 1, now))
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

    fn is_halfway(&self) -> bool {
        self.furthest_head() as f32 >= (self.transitions.len() as f32 / 2.0)
    }

    fn furthest_head(&self) -> usize {
        *self.heads.iter().map(|(head, _)| head).max().unwrap_or(&0)
    }
}
impl From<SpecialDefinition> for Special {
    fn from(definition: SpecialDefinition) -> Self {
        Self {
            transitions: definition.0.transitions,
            required_button: definition.1,
            button_received: definition.1.is_none(),
            ..Default::default()
        }
    }
}
