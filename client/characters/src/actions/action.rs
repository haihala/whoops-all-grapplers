use wag_core::Animation;

use crate::{ActionBlock, ActionRequirement, BlockerRequirement, CancelCategory, CancelPolicy};

#[derive(Clone)]
pub struct Action {
    pub input: Option<&'static str>,
    pub cancel_category: CancelCategory,
    pub script: Vec<ActionBlock>,
    pub requirements: Vec<ActionRequirement>,
}
impl Action {
    pub fn new(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        script: Vec<ActionBlock>,
        requirements: Vec<ActionRequirement>,
    ) -> Self {
        Self {
            input,
            cancel_category,
            script,
            requirements,
        }
    }

    pub fn grounded(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        script: Vec<ActionBlock>,
    ) -> Self {
        Self::new(
            input,
            cancel_category,
            script,
            vec![ActionRequirement::Grounded],
        )
    }

    pub fn airborne(
        input: Option<&'static str>,
        cancel_category: CancelCategory,
        script: Vec<ActionBlock>,
    ) -> Self {
        Self::new(
            input,
            cancel_category,
            script,
            vec![ActionRequirement::Airborne],
        )
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::grounded(
            None,
            CancelCategory::Any,
            vec![ActionBlock {
                events: vec![Animation::default().into()],
                exit_requirement: BlockerRequirement::Time(100),
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
