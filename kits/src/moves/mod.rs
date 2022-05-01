use bevy_inspector_egui::Inspectable;

mod situation;
pub use situation::MoveSituation;
mod move_id;
pub use move_id::MoveId;
mod move_data;
pub use move_data::{Branch, Move, Requirements};
mod move_properties;
pub use move_properties::*;

// Defined smallest to largest aka later ones can cancel earlier ones.
#[derive(PartialEq, PartialOrd, Debug, Inspectable, Clone, Copy, Eq)]
pub enum CancelLevel {
    Anything,
    LightNormal,
    Dash,
    Jump,
    HeavyNormal,
    LightSpecial,
    HeavySpecial,
    Grab,
    Uncancellable,
}
impl Default for CancelLevel {
    fn default() -> Self {
        CancelLevel::Anything
    }
}
