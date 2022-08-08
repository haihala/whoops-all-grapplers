use crate::Situation;

use super::{grounded, FlowControl, MoveType};

#[derive(Clone)]
pub struct Move {
    pub input: Option<&'static str>,
    pub move_type: MoveType,
    pub phases: Vec<FlowControl>,
    pub requirement: fn(Situation) -> bool,
}

impl Default for Move {
    fn default() -> Self {
        Self {
            input: Default::default(),
            move_type: Default::default(),
            phases: Default::default(),
            requirement: grounded,
        }
    }
}

impl std::fmt::Debug for Move {
    // Function pointers are not really debug friendly, trait is required higher up
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move")
            .field("input", &self.input)
            .field("move_type", &self.move_type)
            .finish()
    }
}
