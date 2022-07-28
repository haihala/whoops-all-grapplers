use bevy_inspector_egui::Inspectable;

mod situation;
pub use situation::MoveSituation;
mod move_id;
pub use move_id::MoveId;
mod move_data;
pub use move_data::{Branch, Move, Requirements};
mod move_properties;
pub use move_properties::*;

#[derive(PartialEq, PartialOrd, Debug, Inspectable, Clone, Copy, Eq, Default)]
pub enum MoveType {
    #[default]
    Normal,
    Special,
}
