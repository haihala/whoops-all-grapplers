use bevy_inspector_egui::Inspectable;
use types::RelativeDirection;

mod dash;
pub use dash::{DashPhase, DashState};

#[derive(Inspectable, PartialEq, Clone, Copy, Debug)]
pub enum Movement {
    Walk((usize, RelativeDirection)),
    Dash(DashState),
    Null,
}
impl Movement {
    pub fn in_dash_startup(&self) -> bool {
        match *self {
            Movement::Dash(dash_state) => dash_state.get_phase() == Some(DashPhase::Start),
            _ => false,
        }
    }
    pub fn in_dash_recovery(&self) -> bool {
        match *self {
            Movement::Dash(dash_state) => dash_state.get_phase() == Some(DashPhase::Recovery),
            _ => false,
        }
    }
}

impl Default for Movement {
    fn default() -> Self {
        // Required by Inspectability, not actually used anywhere
        Movement::Null
    }
}
