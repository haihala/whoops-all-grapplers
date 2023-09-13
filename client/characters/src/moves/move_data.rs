use wag_core::Animation;

use crate::Situation;

use super::{airborne, grounded, CancelCategory, CancelPolicy, FlowControl};

#[derive(Clone)]
pub struct Move {
    pub input: Option<&'static str>,
    pub cancel_category: CancelCategory,
    pub phases: Vec<FlowControl>,
    pub requirement: fn(Situation) -> bool,
}
impl Move {
    pub fn new(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        phases: Vec<FlowControl>,
        requirement: fn(Situation) -> bool,
    ) -> Self {
        Self {
            input,
            cancel_category,
            phases,
            requirement,
        }
    }

    pub fn grounded(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        phases: Vec<FlowControl>,
    ) -> Self {
        Self::new(input, cancel_category, phases, grounded)
    }

    pub fn airborne(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        phases: Vec<FlowControl>,
    ) -> Self {
        Self::new(input, cancel_category, phases, airborne)
    }
}

impl Default for Move {
    fn default() -> Self {
        Self::grounded(
            None,
            CancelCategory::Any,
            vec![
                Animation::TPose.into(),
                // The wait is here to indicate if this default is actually being executed
                FlowControl::Wait(100, CancelPolicy::never()),
            ],
        )
    }
}

impl std::fmt::Debug for Move {
    // Function pointers are not really debug friendly, trait is required higher up
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move")
            .field("input", &self.input)
            .field("cancel category", &self.cancel_category)
            .finish()
    }
}
