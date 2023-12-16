use crate::{ActionEvent, ActionRequirement, CancelPolicy, Situation};

#[derive(Clone, Debug, PartialEq)]
pub struct ActionBlock {
    pub events: Vec<ActionEvent>,
    pub exit_requirement: ContinuationRequirement, // To pass naturally
    pub cancel_policy: CancelPolicy,               // To be cancelled out of
    pub mutator: Option<fn(ActionBlock, &Situation) -> ActionBlock>, // This gets passed the original block itself
}
impl Default for ActionBlock {
    fn default() -> Self {
        Self {
            events: Default::default(),
            exit_requirement: Default::default(),
            cancel_policy: CancelPolicy::never(),
            mutator: None,
        }
    }
}
impl ActionBlock {
    pub fn apply_mutator(&self, situation: &Situation) -> Self {
        if let Some(mutator) = self.mutator {
            mutator(self.clone(), situation)
        } else {
            self.clone()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ContinuationRequirement {
    #[default]
    None,
    Conditions(Vec<ActionRequirement>),
    Time(usize),
}
impl ContinuationRequirement {
    pub fn fulfilled(&self, situation: &Situation) -> bool {
        match self {
            Self::None => true,
            Self::Conditions(conditions) => ActionRequirement::check(conditions, situation),
            Self::Time(duration) => {
                let current_block_start_frame =
                    situation.tracker.clone().unwrap().current_block_start_frame;
                (situation.frame - current_block_start_frame)
                    >= ((*duration as f32 / situation.stats.action_speed_multiplier) as usize)
            }
        }
    }
}
impl From<usize> for ContinuationRequirement {
    fn from(value: usize) -> Self {
        Self::Time(value)
    }
}

impl From<ActionRequirement> for ContinuationRequirement {
    fn from(value: ActionRequirement) -> Self {
        Self::Conditions(vec![value])
    }
}

impl From<Vec<ActionRequirement>> for ContinuationRequirement {
    fn from(value: Vec<ActionRequirement>) -> Self {
        Self::Conditions(value)
    }
}
