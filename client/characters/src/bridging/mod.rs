// This module is called bridging, because it's for bridging the game state into the actions

mod action_tracker;
mod hit_data;
mod situation;

pub use action_tracker::ActionTracker;
pub use hit_data::{HitEffect, HitInfo};
pub use situation::Situation;
