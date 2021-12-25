use bevy::math::Vec3;
use bevy_inspector_egui::Inspectable;

use types::RelativeDirection;

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum DashPhase {
    Start,
    Recovery,
}
#[derive(Inspectable, PartialEq, Clone, Copy, Debug, Default)]
pub struct DashState {
    pub direction: RelativeDirection,
    midpoint_frame: usize,
    end_frame: usize,
    pub current_frame: usize,
}
impl DashState {
    pub fn new(direction: RelativeDirection, current_frame: usize) -> Self {
        Self {
            direction,
            current_frame,
            midpoint_frame: current_frame + constants::DASH_START_FRAMES,
            end_frame: current_frame + constants::DASH_WHOLE_FRAMES,
        }
    }

    pub fn get_phase(&self) -> Option<DashPhase> {
        if self.current_frame <= self.midpoint_frame {
            Some(DashPhase::Start)
        } else if self.current_frame <= self.end_frame {
            Some(DashPhase::Recovery)
        } else {
            None
        }
    }

    pub fn tick_check_expiration(&mut self, current_frame: usize) -> bool {
        self.current_frame = current_frame;

        current_frame > self.end_frame
    }

    pub fn get_vec(&self, forward: Vec3) -> Vec3 {
        let amplitude = match self.get_phase().unwrap() {
            DashPhase::Start => constants::DASH_START_SPEED,
            DashPhase::Recovery => constants::DASH_RECOVERY_SPEED,
        };
        let direction = self.direction.handle_mirroring(forward);

        direction * amplitude
    }
}
