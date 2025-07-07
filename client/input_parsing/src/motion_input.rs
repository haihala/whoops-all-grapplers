use bevy::prelude::*;
use foundation::InputEvent;

use crate::{
    helper_types::{InputRequirement, RequirementMode, StateRequirement},
    input_parser::InputHistory,
};

#[derive(Default, Debug, Clone, Reflect, PartialEq)]
pub struct MotionInput {
    requirements: Vec<InputRequirement>,
    slow: bool,     // More time per requirement
    absolute: bool, // Does not care about which way the player is facing
}
impl MotionInput {
    pub fn steps(&self) -> usize {
        self.requirements.iter().fold(0, |total, req| {
            total
                + if let RequirementMode::All(parts) = &req.mode {
                    parts.len()
                } else {
                    1
                }
        })
    }

    pub fn complexity(&self) -> usize {
        self.steps()
            + self.requirements.iter().fold(0, |mut total, req| {
                if !req.state_requirement.stick.is_empty() {
                    total += 1;
                }

                total += req.state_requirement.buttons.len();
                total
            })
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

                    if !requirement.state_requirement.met_by(state) {
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

                    if !requirement.state_requirement.met_by(state) {
                        break false;
                    }

                    if options.contains(&event) {
                        break true;
                    }

                    if sticky && requirement.mode.is_negated_by(event) {
                        break false;
                    }
                },
                RequirementMode::Anything => loop {
                    let Some((_, state)) = past.next() else {
                        break false;
                    };

                    if requirement.state_requirement.met_by(state) {
                        break true;
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

    pub fn buffer_window_size(&self) -> usize {
        (self.steps() - 1) * if self.slow { 10 } else { 5 }
    }
}

impl From<&str> for MotionInput {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for MotionInput {
    fn from(input: String) -> Self {
        let mut split = input.split('|');
        let sequence = split.next().unwrap();
        let metadata = split.next().unwrap_or("");

        let mut incomplete = InputRequirement::default();
        let mut complete = vec![];

        let mut chars = sequence.chars();

        while let Some(ch) = chars.next() {
            match ch {
                // Modifiers
                '+' => {
                    debug_assert!(
                        !complete.is_empty(),
                        "Sticky modifier can't be first symbol"
                    );

                    incomplete.sticky = true;
                }
                '{' => {
                    incomplete.state_requirement = chars
                        .by_ref()
                        .take_while(|nxt| *nxt != '}')
                        .fold(StateRequirement::default(), |mut acc, nxt| {
                            let ev: InputEvent = nxt.into();

                            match ev {
                                InputEvent::Point(stick_position) => acc.stick.push(stick_position),
                                InputEvent::Press(game_button) => {
                                    acc.buttons.push((game_button, true))
                                }
                                InputEvent::Release(game_button) => {
                                    acc.buttons.push((game_button, false))
                                }
                            };

                            acc
                        });
                }
                // Steps
                '[' => {
                    incomplete.mode = RequirementMode::Any(
                        chars
                            .by_ref()
                            .take_while(|nxt| *nxt != ']')
                            .map(|nxt| nxt.into())
                            .collect(),
                    );
                    complete.push(incomplete);
                    incomplete = InputRequirement::default();
                }
                '(' => {
                    incomplete.mode = RequirementMode::All(
                        chars
                            .by_ref()
                            .take_while(|nxt| *nxt != ')')
                            .map(|nxt| nxt.into())
                            .collect(),
                    );
                    complete.push(incomplete);
                    incomplete = InputRequirement::default();
                }
                '*' => {
                    incomplete.mode = RequirementMode::Anything;
                    debug_assert!(!incomplete.state_requirement.is_empty());
                    complete.push(incomplete);
                    incomplete = InputRequirement::default();
                }
                _ => {
                    incomplete.mode = RequirementMode::Any(vec![ch.into()]);
                    complete.push(incomplete);
                    incomplete = InputRequirement::default();
                }
            }
        }

        debug_assert!(!complete.is_empty(), "No requirements");

        let mut out = Self {
            requirements: complete.into_iter().rev().collect(),
            ..default()
        };

        for ch in metadata.chars() {
            match ch {
                'A' => {
                    out.absolute = true;
                }
                'S' => {
                    out.slow = true;
                }
                unknown => panic!("Unknown char ̈́'{unknown}'"),
            }
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::platform::collections::HashSet;
    use foundation::{Facing::*, GameButton::*, InputEvent::*, InputState, StickPosition::*};

    #[test]
    fn hadouken() {
        let parsed: MotionInput = "236f".into();
        assert_eq!(
            parsed.requirements,
            vec![Press(Fast), Point(E), Point(SE), Point(S)]
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
                mode: RequirementMode::Any(vec![Point(E), Press(Fast)]),
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
                mode: RequirementMode::All(vec![Point(E), Press(Fast)]),
                ..default()
            },]
        )
    }

    #[test]
    fn any_group_contained() {
        let input: MotionInput = "[123]".into();

        assert!(input.contained_in(&[InputHistory {
            event: Point(SE),
            ..default()
        }]));

        assert!(input.contained_in(&[InputHistory {
            event: Point(S),
            ..default()
        }]));
    }

    #[test]
    fn all_group_contained() {
        let input: MotionInput = "(6f)".into();

        assert!(input.contained_in(&[
            InputHistory {
                event: Press(Fast),
                ..default()
            },
            InputHistory {
                event: Point(E),
                ..default()
            },
        ]));

        assert!(!input.contained_in(&[InputHistory {
            event: Point(E),
            ..default()
        },]));

        assert!(!input.contained_in(&[InputHistory {
            event: Press(Fast),
            ..default()
        },]));
    }

    #[test]
    fn input_state_check_parses() {
        let input: MotionInput = "{123sG}f".into();

        assert_eq!(input.complexity(), 4);
        assert_eq!(input.steps(), 1);
        assert_eq!(
            input.requirements[0].state_requirement,
            StateRequirement {
                stick: vec![SW, S, SE],
                buttons: vec![(Strong, true), (Gimmick, false)]
            }
        );
    }

    #[test]
    fn input_state_check_validates() {
        let input: MotionInput = "{123}f|A".into();

        let event = Press(Fast);

        // Does not pass without correct state
        assert!(!input.contained_in(&[InputHistory { event, ..default() },]));

        // Passes with correct state
        assert!(input.contained_in(&[InputHistory {
            event,
            state: InputState {
                stick_position: S,
                ..default()
            },
            ..default()
        }]));
    }

    #[test]
    fn metadata_parses() {
        let input: MotionInput = "f|A".into();

        assert!(input.absolute);
    }

    #[test]
    fn complexity() {
        for (input, complexity) in [
            ("f", 1),
            ("(f)", 1),
            ("(fs)", 2),
            ("236(fs)", 5),
            ("{123}f", 2),
        ] {
            assert_eq!(Into::<MotionInput>::into(input).complexity(), complexity);
        }
    }

    #[test]
    fn buffers_failing() {
        // This is real input data
        // IMO it should contain "{2}*4f", but didn't because of a bug
        let hist = vec![
            InputHistory {
                event: Press(Fast),
                state: InputState {
                    stick_position: E,
                    pressed: HashSet::new(),
                },
                facing: Left,
                frame: 783,
            },
            InputHistory {
                event: Point(E),
                state: InputState {
                    stick_position: E,
                    pressed: HashSet::new(),
                },
                facing: Left,
                frame: 782,
            },
            InputHistory {
                event: Point(E),
                state: InputState {
                    stick_position: E,
                    pressed: HashSet::new(),
                },
                facing: Left,
                frame: 781,
            },
            InputHistory {
                event: Point(E),
                state: InputState {
                    stick_position: E,
                    pressed: HashSet::new(),
                },
                facing: Left,
                frame: 781,
            },
            InputHistory {
                event: Point(E),
                state: InputState {
                    stick_position: E,
                    pressed: HashSet::new(),
                },
                facing: Left,
                frame: 781,
            },
            InputHistory {
                event: Point(E),
                state: InputState {
                    stick_position: E,
                    pressed: HashSet::new(),
                },
                facing: Left,
                frame: 781,
            },
            InputHistory {
                event: Point(E),
                state: InputState {
                    stick_position: E,
                    pressed: HashSet::new(),
                },
                facing: Left,
                frame: 781,
            },
            InputHistory {
                event: Point(E),
                state: InputState {
                    stick_position: SE,
                    pressed: HashSet::new(),
                },
                facing: Left,
                frame: 781,
            },
            InputHistory {
                event: Release(Fast),
                state: InputState {
                    stick_position: SE,
                    pressed: vec![Fast].into_iter().collect(),
                },
                facing: Left,
                frame: 781,
            },
            InputHistory {
                event: Point(SE),
                state: InputState {
                    stick_position: S,
                    pressed: vec![Fast].into_iter().collect(),
                },
                facing: Left,
                frame: 779,
            },
        ];

        let the_move: MotionInput = "{2}*4f".into();

        assert!(the_move.contained_in(&hist));
    }
}
