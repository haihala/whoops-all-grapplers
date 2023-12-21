use bevy::utils::HashSet;
use wag_core::{GameButton, Stats, StatusFlag};

use crate::{ActionTracker, Inventory, ResourceType, WAGResource};

#[derive(Debug, Clone, Default)]
pub struct Situation {
    pub grounded: bool,
    pub tracker: Option<ActionTracker>,
    pub inventory: Inventory,
    pub resources: Vec<(ResourceType, WAGResource)>,
    pub status_flags: HashSet<StatusFlag>,
    pub frame: usize,
    pub stats: Stats,
    pub held_buttons: HashSet<GameButton>,
    // Kept minimal so far, but will grow as needed
}
