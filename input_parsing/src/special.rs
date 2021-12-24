use moves::SpecialDefinition;
use types::{GameButton, StickPosition};

use crate::{helper_types::Diff, motion_input::Motion};

/// I.E. Quarter circle forward press punch -> fireball
pub struct Special {
    motion: Motion,
    required_button: Option<GameButton>,

    button_received: bool,
}
impl Special {
    pub fn clear(&mut self) {
        self.motion.clear();
        self.button_received = self.required_button.is_none();
    }

    pub fn advance(&mut self, diff: &Diff, old_stick: StickPosition) {
        if let Some(new_stick) = diff.stick_move {
            assert!(
                old_stick != new_stick,
                "old={}, new={}",
                old_stick,
                new_stick,
            );
            self.motion.advance(old_stick, new_stick);
        }

        if !self.button_received {
            if let Some(button) = &self.required_button {
                if self.motion.is_halfway() && diff.pressed_contains(button) {
                    self.button_received = true;
                }
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.motion.is_done() && self.button_received
    }
}
impl From<SpecialDefinition> for Special {
    fn from(definition: SpecialDefinition) -> Self {
        Self {
            motion: definition.0.into(),
            required_button: definition.1,
            button_received: definition.1.is_none(),
        }
    }
}
