use bevy_inspector_egui::Inspectable;

mod situation_shorthands;
pub use situation_shorthands::*;
mod move_history;
pub use move_history::MoveHistory;
mod move_situation;
pub use move_situation::Situation;
mod move_data;
pub use move_data::Move;
mod move_phases;
pub use move_phases::{Action, FlowControl, Movement};
mod move_properties;
pub use move_properties::*;

#[derive(PartialEq, PartialOrd, Debug, Inspectable, Clone, Copy, Eq, Default)]
pub enum MoveType {
    #[default]
    Normal,
    Special,
}
