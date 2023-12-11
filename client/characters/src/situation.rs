use wag_core::Stats;

use crate::{ActionTracker, Inventory, ResourceType, WAGResource};

#[derive(Debug, Clone, Default)]
pub struct Situation {
    pub grounded: bool,
    pub tracker: Option<ActionTracker>,
    pub inventory: Inventory,
    pub resources: Vec<(ResourceType, WAGResource)>,
    pub frame: usize,
    pub stats: Stats,
    // Kept minimal so far, but will grow as needed
}
