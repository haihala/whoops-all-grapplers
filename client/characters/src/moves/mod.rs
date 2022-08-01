use bevy_inspector_egui::Inspectable;

mod move_history;
pub use move_history::{MoveHistory, Situation};
mod moves;
pub use moves::Move;
mod move_phases;
pub use move_phases::{Action, FlowControl};
mod move_properties;
pub use move_properties::*;

#[derive(PartialEq, PartialOrd, Debug, Inspectable, Clone, Copy, Eq, Default)]
pub enum MoveType {
    #[default]
    Normal,
    Special,
}
