use crate::{ActionEvent, ActionRequirement, CancelPolicy, Situation};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionBlock {
    pub events: Vec<ActionEvent>,
    pub exit_requirement: BlockerRequirement, // To pass naturally
    pub cancel_policy: CancelPolicy,          // To be cancelled out of
    pub mutator: Option<fn(&mut ActionBlock, &Situation) -> ActionBlock>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum BlockerRequirement {
    #[default]
    None,
    Conditions(Vec<ActionRequirement>),
    Time(usize),
}
impl BlockerRequirement {
    pub fn fulfilled(&self, situation: Situation) -> bool {
        match self {
            Self::None => true,
            Self::Conditions(conditions) => ActionRequirement::check(conditions, &situation),
            Self::Time(duration) => {
                (situation.frame - situation.tracker.unwrap().current_block_start_frame)
                    >= ((*duration as f32 / situation.stats.action_speed_multiplier) as usize)
            }
        }
    }
}
impl From<usize> for BlockerRequirement {
    fn from(value: usize) -> Self {
        Self::Time(value)
    }
}

impl From<ActionRequirement> for BlockerRequirement {
    fn from(value: ActionRequirement) -> Self {
        Self::Conditions(vec![value])
    }
}

impl From<Vec<ActionRequirement>> for BlockerRequirement {
    fn from(value: Vec<ActionRequirement>) -> Self {
        Self::Conditions(value)
    }
}
