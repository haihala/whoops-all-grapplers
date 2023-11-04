use crate::{ActionEvent, CancelPolicy, Situation};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionBlock {
    pub events: Vec<ActionEvent>,
    pub exit_requirement: Requirement, // To pass naturally
    pub cancel_policy: CancelPolicy,   // To be cancelled out of
    pub mutator: Option<fn(&mut ActionBlock, &Situation) -> ActionBlock>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Requirement {
    #[default]
    None,
    Condition(fn(Situation) -> bool),
    Time(usize),
}
impl Requirement {
    pub fn fulfilled(&self, situation: Situation) -> bool {
        match self {
            Self::None => true,
            Self::Condition(condition) => condition(situation),
            Self::Time(duration) => {
                (situation.frame - situation.tracker.unwrap().current_block_start_frame)
                    >= ((*duration as f32 / situation.stats.action_speed_multiplier) as usize)
            }
        }
    }
}
impl From<usize> for Requirement {
    fn from(value: usize) -> Self {
        Self::Time(value)
    }
}

impl From<fn(Situation) -> bool> for Requirement {
    fn from(value: fn(Situation) -> bool) -> Self {
        Self::Condition(value)
    }
}
