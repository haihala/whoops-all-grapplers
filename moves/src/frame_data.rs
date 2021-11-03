use bevy::utils::HashMap;
use bevy_inspector_egui::Inspectable;

use crate::ryan::*;
use crate::MoveType;

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub struct FrameData {
    pub active_start: usize,
    pub recovery_start: usize,
    pub recovered: usize,
}
impl FrameData {
    pub fn new(startup: usize, active: usize, recovery: usize) -> Self {
        Self {
            active_start: startup,
            recovery_start: startup + active,
            recovered: startup + active + recovery,
        }
    }
}

pub fn ryan_frames() -> HashMap<MoveType, FrameData> {
    vec![
        (HADOUKEN, FrameData::new(10, 10, 10)),
        (PUNCH, FrameData::new(10, 10, 10)),
        (COMMAND_PUNCH, FrameData::new(10, 10, 10)),
    ]
    .into_iter()
    .collect()
}
