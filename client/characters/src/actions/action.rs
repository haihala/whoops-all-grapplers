use wag_core::Animation;

use crate::{ActionBlock, CancelCategory, CancelPolicy, Requirement, Situation};

#[derive(Clone)]
pub struct Action {
    pub input: Option<&'static str>,
    pub cancel_category: CancelCategory,
    pub script: Vec<ActionBlock>,
    pub requirement: fn(Situation) -> bool,
}
impl Action {
    pub fn new(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        script: Vec<ActionBlock>,
        requirement: fn(Situation) -> bool,
    ) -> Self {
        Self {
            input,
            cancel_category,
            script,
            requirement,
        }
    }

    pub fn grounded(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        script: Vec<ActionBlock>,
    ) -> Self {
        Self::new(input, cancel_category, script, |s: Situation| s.grounded())
    }

    pub fn airborne(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        script: Vec<ActionBlock>,
    ) -> Self {
        Self::new(input, cancel_category, script, |s: Situation| s.airborne())
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::grounded(
            None,
            CancelCategory::Any,
            vec![ActionBlock {
                events: vec![Animation::TPose.into()],
                exit_requirement: Requirement::Time(100),
                cancel_policy: CancelPolicy::never(),
                mutator: None,
            }],
        )
    }
}

impl std::fmt::Debug for Action {
    // Function pointers are not really debug friendly, trait is required higher up
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move")
            .field("input", &self.input)
            .field("cancel category", &self.cancel_category)
            .finish()
    }
}
