use crate::ryan::*;
use crate::universal::*;
use bevy::utils::HashMap;
use types::{GameButton, MoveType, Normal, StickPosition};

pub type StickTransition = (Option<StickPosition>, StickPosition);
pub struct MotionDefinition {
    // Definitions are done this way so that input parsing logic can stay in input parsing
    // Moves still need to be defined somehow
    pub transitions: Vec<StickTransition>,
}
impl From<Vec<i32>> for MotionDefinition {
    fn from(requirements: Vec<i32>) -> Self {
        Self {
            transitions: requirements
                .into_iter()
                .map(StickPosition::from)
                .map(|x| (None, x))
                .collect(),
        }
    }
}
impl From<Vec<(Option<i32>, i32)>> for MotionDefinition {
    fn from(requirements: Vec<(Option<i32>, i32)>) -> Self {
        Self {
            transitions: requirements
                .into_iter()
                .map(|(pre, post)| {
                    let target = StickPosition::from(post);
                    if let Some(req) = pre {
                        (Some(StickPosition::from(req)), target)
                    } else {
                        (None, target)
                    }
                })
                .collect(),
        }
    }
}

pub type SpecialDefinition = (MotionDefinition, Option<GameButton>);

pub fn ryan_normals() -> HashMap<MoveType, Normal> {
    vec![
        (
            PUNCH,
            Normal {
                button: GameButton::Fast,
                stick: None,
            },
        ),
        (
            COMMAND_PUNCH,
            Normal {
                button: GameButton::Fast,
                stick: Some(StickPosition::E),
            },
        ),
    ]
    .into_iter()
    .collect()
}

pub fn ryan_specials() -> HashMap<MoveType, SpecialDefinition> {
    vec![(HADOUKEN, (vec![2, 3, 6].into(), Some(GameButton::Fast)))]
        .into_iter()
        .chain(universal_specials())
        .collect()
}

fn universal_specials() -> std::vec::IntoIter<(MoveType, SpecialDefinition)> {
    vec![
        (DASH_FORWARD, (vec![6, 5, 6].into(), None)),
        (DASH_BACK, (vec![4, 5, 4].into(), None)),
    ]
    .into_iter()
}
