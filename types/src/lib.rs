use bevy_inspector_egui::Inspectable;
use std::fmt::{Debug, Display};

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Player {
    One,
    Two,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
