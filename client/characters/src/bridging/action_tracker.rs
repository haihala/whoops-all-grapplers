use bevy::prelude::*;
use foundation::ActionId;

#[derive(Debug, Clone, Default, Reflect, Copy, Hash)]
pub struct ActionTracker {
    pub has_hit: bool,
    pub was_cancelled_into: bool,
    pub start_frame: usize,
    pub action_id: ActionId,
}
impl ActionTracker {
    pub fn new(start_frame: usize, was_cancelled_into: bool, action_id: ActionId) -> Self {
        Self {
            has_hit: false,
            was_cancelled_into,
            action_id,
            start_frame,
        }
    }
}
