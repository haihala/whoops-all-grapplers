use bevy::utils::HashMap;

use crate::{ActionTracker, Inventory, ResourceType, WAGResource};

#[derive(Debug, Clone, Default)]
pub struct Situation {
    pub grounded: bool,
    pub tracker: Option<ActionTracker>,
    pub inventory: Inventory,
    pub resources: HashMap<ResourceType, WAGResource>,
    pub frame: usize,
    // Kept minimal so far, but will grow as needed
}
impl Situation {
    pub fn grounded(&self) -> bool {
        self.grounded
    }
    pub fn airborne(&self) -> bool {
        !self.grounded
    }
}
