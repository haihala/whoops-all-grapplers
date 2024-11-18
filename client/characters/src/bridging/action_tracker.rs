use bevy::prelude::*;
use wag_core::ActionId;

#[derive(Debug, Clone, Default, Reflect, Copy, Hash)]
pub struct ActionTracker {
    pub has_hit: bool,
    pub start_frame: usize,
    pub action_id: ActionId,
}
impl ActionTracker {
    pub fn new(start_frame: usize, action_id: ActionId) -> Self {
        Self {
            has_hit: false,
            action_id,
            start_frame,
        }
    }
}
