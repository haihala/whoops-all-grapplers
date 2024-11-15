use crate::{ActionEvent, ActionRequirement, Situation};

pub type Script = Box<dyn Fn(&Situation) -> Vec<ActionEvent> + Send + Sync>;

pub struct Action {
    pub input: Option<&'static str>,
    pub requirement: ActionRequirement,
    pub script: Script,
}

impl std::fmt::Debug for Action {
    // Function pointers are not really debug friendly, trait is required higher up
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move").field("input", &self.input).finish()
    }
}
