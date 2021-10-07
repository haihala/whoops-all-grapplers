use bevy::utils::HashMap;

use crate::ryan::*;
use crate::MoveType;

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
