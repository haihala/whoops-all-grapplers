use bevy::prelude::*;
use bevy::utils::HashSet;
use wag_core::{Facing, GameButton, Stats, StatusFlag};

use crate::{ActionTracker, Inventory, ResourceType, WAGResource};

#[derive(Debug, Clone, Default)]
pub struct Situation {
    pub grounded: bool,
    pub facing: Facing,
    pub tracker: Option<ActionTracker>,
    pub inventory: Inventory,
    pub resources: Vec<(ResourceType, WAGResource)>,
    pub status_flags: HashSet<StatusFlag>,
    pub frame: usize,
    pub stats: Stats,
    pub held_buttons: HashSet<GameButton>,
    pub position: Vec3,
    // Kept minimal so far, but will grow as needed
}
impl Situation {
    pub fn get_resource(&self, resource_type: ResourceType) -> Option<&WAGResource> {
        self.resources.iter().find_map(|(rt, res)| {
            if rt == &resource_type {
                Some(res)
            } else {
                None
            }
        })
    }
}
