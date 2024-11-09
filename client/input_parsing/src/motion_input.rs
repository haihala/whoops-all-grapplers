use bevy::prelude::*;
use wag_core::StickPosition;

use crate::{
    helper_types::{InputRequirement, RequirementMode},
    input_parser::InputHistory,
};

#[derive(Default, Debug, Clone, Reflect, PartialEq)]
pub struct MotionInput {
    requirements: Vec<InputRequirement>,
    absolute: bool, // Does not care about which way the player is facing
    allowed_stick_positions: Vec<StickPosition>, // Circumvents buffer length
}
impl MotionInput {
    pub fn complexity(&self) -> usize {
        self.requirements.iter().fold(0, |acc, req| {
            acc + if let RequirementMode::All(parts) = &req.mode {
                parts.len()
            } else {
                1
            }
        }) + (!self.allowed_stick_positions.is_empty() as usize)
    }

    pub(crate) fn contained_in(&self, history: &[InputHistory]) -> bool {
        let mut past = history.iter().map(|ev| ev.handle_facing(self.absolute));

        let mut sticky = false;

        for requirement in self.requirements.clone() {
            let requirement_met = match requirement.mode.clone() {
                RequirementMode::All(mut to_fulfill) => loop {
                    let Some((event, state)) = past.next() else {
                        break false;
                    };

                    if !self.allowed_stick_positions.is_empty()
                        && !self.allowed_stick_positions.contains(&state.stick_position)
                    {
                        break false;
                    }

                    to_fulfill.retain(|ev| *ev != event);

                    if to_fulfill.is_empty() {
                        break true;
                    }

                    if sticky && requirement.mode.is_negated_by(event) {
                        break false;
                    }
                },
                RequirementMode::Any(options) => loop {
                    let Some((event, state)) = past.next() else {
                        break false;
                    };

                    if !self.allowed_stick_positions.is_empty()
                        && !self.allowed_stick_positions.contains(&state.stick_position)
                    {
                        break false;
                    }

                    if options.iter().any(|ev| *ev == event) {
                        break true;
                    }

                    if sticky && requirement.mode.is_negated_by(event) {
                        break false;
                    }
                },
                RequirementMode::None => panic!("How did we get here?"),
            };

            if !requirement_met {
                return false;
            }

            sticky = requirement.sticky;
        }

        true
    }
}

impl From<&str> for MotionInput {
    fn from(input: &str) -> Self {
        let mut split = input.split('|');
        let sequence = split.next().unwrap();
        let metadata = split.next().unwrap_or("");

        let mut incomplete = InputRequirement::default();
        let mut complete = vec![];

        for ch in sequence.chars() {
            match ch {
                '[' => {
                    incomplete.mode = RequirementMode::Any(vec![]);
                }
                '(' => {
                    incomplete.mode = RequirementMode::All(vec![]);
                }
                ']' => {
                    assert!(
                        matches!(incomplete.mode, RequirementMode::Any(_)),
                        "Using ] to close a ("
                    );
                    complete.push(incomplete);
                    incomplete = InputRequirement::default();
                }
                ')' => {
                    assert!(
                        matches!(incomplete.mode, RequirementMode::All(_)),
                        "Using ) to close a ["
                    );
                    complete.push(incomplete);
                    incomplete = InputRequirement::default();
                }
                '+' => {
                    assert!(
                        !complete.is_empty(),
                        "Sticky modifier can't be first symbol"
                    );

                    incomplete.sticky = true;
                }
                _ => {
                    let new_ev = ch.into();

                    match incomplete.mode {
                        RequirementMode::All(ref mut evs) | RequirementMode::Any(ref mut evs) => {
                            evs.push(new_ev);
                        }
                        RequirementMode::None => {
                            incomplete.mode = RequirementMode::Any(vec![new_ev]);
                            complete.push(incomplete);
                            incomplete = InputRequirement::default();
                        }
                    }
                }
            }
        }

        assert!(!complete.is_empty(), "No requirements");

        let mut out = Self {
            requirements: complete.into_iter().rev().collect(),
            ..default()
        };

        for ch in metadata.chars() {
            match ch {
                'A' => {
                    out.absolute = true;
                }
                '1'..='9' => {
                    out.allowed_stick_positions
                        .push((ch.to_digit(10).unwrap() as i32).into());
                }
                unknown => panic!("Unknown char ̈́'{}'", unknown),
            }
        }

        out
    }
}

#[cfg(test)]
mod test {
    use wag_core::{GameButton, StickPosition};

    use crate::{helper_types::InputState, InputEvent};

    use super::*;

    #[test]
    fn hadouken() {
        let parsed: MotionInput = "236f".into();
        assert_eq!(
            parsed.requirements,
            vec![
                InputEvent::Press(GameButton::Fast),
                InputEvent::Point(StickPosition::E),
                InputEvent::Point(StickPosition::SE),
                InputEvent::Point(StickPosition::S),
            ]
            .into_iter()
            .map(|ev| InputRequirement {
                mode: RequirementMode::Any(vec![ev]),
                ..default()
            })
            .collect::<Vec<_>>()
        )
    }

    #[test]
    fn any_group_parsed() {
        let parsed: MotionInput = "[6f]".into();
        assert_eq!(
            parsed.requirements,
            vec![InputRequirement {
                mode: RequirementMode::Any(vec![
                    InputEvent::Point(StickPosition::E),
                    InputEvent::Press(GameButton::Fast)
                ]),
                ..default()
            },]
        )
    }

    #[test]
    fn all_group_parsed() {
        let parsed: MotionInput = "(6f)".into();
        assert_eq!(
            parsed.requirements,
            vec![InputRequirement {
                mode: RequirementMode::All(vec![
                    InputEvent::Point(StickPosition::E),
                    InputEvent::Press(GameButton::Fast)
                ]),
                ..default()
            },]
        )
    }

    #[test]
    fn any_group_contained() {
        let input: MotionInput = "[123]".into();

        assert!(input.contained_in(&[InputHistory {
            event: InputEvent::Point(StickPosition::SE),
            ..default()
        }]));

        assert!(input.contained_in(&[InputHistory {
            event: InputEvent::Point(StickPosition::S),
            ..default()
        }]));
    }

    #[test]
    fn all_group_contained() {
        let input: MotionInput = "(6f)".into();

        assert!(input.contained_in(&[
            InputHistory {
                event: InputEvent::Press(GameButton::Fast),
                ..default()
            },
            InputHistory {
                event: InputEvent::Point(StickPosition::E),
                ..default()
            },
        ]));

        assert!(!input.contained_in(&[InputHistory {
            event: InputEvent::Point(StickPosition::E),
            ..default()
        },]));

        assert!(!input.contained_in(&[InputHistory {
            event: InputEvent::Press(GameButton::Fast),
            ..default()
        },]));
    }

    #[test]
    fn metadata_parses() {
        let input: MotionInput = "f|A123".into();

        assert!(input.absolute);

        assert_eq!(
            input.allowed_stick_positions,
            vec![StickPosition::SW, StickPosition::S, StickPosition::SE]
        );
    }

    #[test]
    fn metadata_validates() {
        let input: MotionInput = "f|A123".into();

        let event = InputEvent::Press(GameButton::Fast);

        // Does not pass without correct state
        assert!(!input.contained_in(&[InputHistory { event, ..default() },]));

        // Passes with correct state
        assert!(input.contained_in(&[InputHistory {
            event,
            state: InputState {
                stick_position: StickPosition::S,
                ..default()
            },
            ..default()
        },]));
    }

    #[test]
    fn complexity() {
        for (input, complexity) in [
            ("f", 1),
            ("(f)", 1),
            ("(fs)", 2),
            ("236(fs)", 5),
            ("f|123", 2),
        ] {
            assert_eq!(Into::<MotionInput>::into(input).complexity(), complexity);
        }
    }
}
