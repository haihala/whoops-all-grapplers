use bevy_inspector_egui::Inspectable;

use crate::RelativeDirection;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum DashPhase {
    Start,
    Recovery,
}
#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub struct DashState {
    pub direction: RelativeDirection,
    midpoint_frame: usize,
    end_frame: usize,
}
impl DashState {
    pub fn new(direction: RelativeDirection, current_frame: usize) -> Self {
        Self {
            direction,
            midpoint_frame: current_frame + constants::DASH_START_FRAMES,
            end_frame: current_frame + constants::DASH_WHOLE_FRAMES,
        }
    }

    pub fn get_phase(&self, current_frame: usize) -> Option<DashPhase> {
        if current_frame <= self.midpoint_frame {
            Some(DashPhase::Start)
        } else if current_frame <= self.end_frame {
            Some(DashPhase::Recovery)
        } else {
            None
        }
    }
}
