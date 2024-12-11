use bevy::prelude::*;
use bevy::utils::HashSet;
use wag_core::{Combo, Facing, GameButton, Stats, StatusFlag, StickPosition};

use crate::{ActionEvent, ActionTracker, Gauge, GaugeType, Inventory};

#[derive(Debug, Clone, Default)]
pub struct Situation {
    pub grounded: bool,
    pub facing: Facing,
    pub tracker: Option<ActionTracker>,
    pub inventory: Inventory,
    pub resources: Vec<(GaugeType, Gauge)>,
    pub status_flags: HashSet<StatusFlag>,
    pub frame: usize,
    pub stats: Stats,
    pub stick_position: StickPosition,
    pub held_buttons: HashSet<GameButton>,
    pub position: Vec3,
    pub combo: Option<Combo>,
    pub stunned: bool,
}
impl Situation {
    pub fn get_resource(&self, resource_type: GaugeType) -> Option<&Gauge> {
        self.resources.iter().find_map(|(rt, res)| {
            if rt == &resource_type {
                Some(res)
            } else {
                None
            }
        })
    }

    pub fn end_at(&self, frame: usize) -> Vec<ActionEvent> {
        if self.after_frame(frame) {
            vec![ActionEvent::End]
        } else {
            vec![]
        }
    }

    pub fn elapsed(&self) -> usize {
        self.tracker.map_or(0, |t| self.frame - t.start_frame)
    }

    // TODO: Add a system where this also is only once true in the lifespan of the move
    pub fn on_frame(&self, frame: usize) -> bool {
        if self.elapsed() == 0 {
            return frame == 0;
        }

        let prev_frame = (self.elapsed() - 1) as f32 * self.stats.action_speed_multiplier;
        let next_frame = (self.elapsed() + 1) as f32 * self.stats.action_speed_multiplier;

        (prev_frame as usize) < frame && (next_frame as usize) > frame
    }

    pub fn after_frame(&self, frame: usize) -> bool {
        ((self.elapsed() as f32 * self.stats.action_speed_multiplier) as usize) > frame
    }
}
