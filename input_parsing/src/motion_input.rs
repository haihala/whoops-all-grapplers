use bevy::prelude::*;
use bevy::utils::Instant;

use types::GameButton;

use crate::{
    helper_types::{Diff, Frame, InputEvent},
    MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS,
};

#[derive(Debug, Clone, Eq, PartialEq)]
struct ParserHead {
    index: usize,
    last_update: Instant,
    /// None if complete
    requirement: Option<InputEvent>,
    multipresses_received: Vec<GameButton>,
}

impl Default for ParserHead {
    fn default() -> Self {
        Self {
            index: default(),
            last_update: Instant::now(),
            requirement: default(),
            multipresses_received: default(),
        }
    }
}
impl ParserHead {
    fn new_from_diff(requirements: &[InputEvent], diff: &Diff) -> ParserHead {
        let mut new = ParserHead::new(requirements.get(0).cloned());
        new.advance(requirements, diff);
        new
    }

    fn new(requirement: Option<InputEvent>) -> ParserHead {
        ParserHead {
            requirement,
            ..default()
        }
    }

    fn is_done(&self) -> bool {
        self.requirement.is_none()
    }

    fn expired(&self) -> bool {
        let time_from_previous_event = Instant::now()
            .duration_since(self.last_update)
            .as_secs_f32();

        time_from_previous_event > MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS
    }

    fn bump(&mut self, requirement: Option<InputEvent>) {
        *self = ParserHead {
            requirement,
            index: self.index + 1,
            ..default()
        }
    }

    fn advance(&mut self, requirements: &[InputEvent], diff: &Diff) {
        while !self.is_done() && self.requirement_met(diff) {
            self.bump(self.get_next_requirement(requirements));
        }
    }

    fn get_next_requirement(&self, requirements: &[InputEvent]) -> Option<InputEvent> {
        requirements.get(self.index + 1).cloned()
    }

    fn requirement_met(&mut self, diff: &Diff) -> bool {
        if let Some(requirement) = self.requirement.clone() {
            match requirement {
                InputEvent::Point(required_stick) => {
                    diff.stick_move.is_some() && diff.stick_move.unwrap() == required_stick
                }
                InputEvent::Range(required_sticks) => {
                    diff.stick_move.is_some() && required_sticks.contains(&diff.stick_move.unwrap())
                }
                InputEvent::Press(required_button) => diff.pressed_contains(&required_button),
                InputEvent::MultiPress(required_buttons) => {
                    if let Some(pressed) = diff.pressed.clone() {
                        let mut new_buttons = pressed.into_iter().collect();
                        self.multipresses_received.append(&mut new_buttons);

                        if required_buttons
                            .into_iter()
                            .filter(|button| !self.multipresses_received.contains(button))
                            .peekable()
                            .peek()
                            .is_none()
                        {
                            return true;
                        }
                    }
                    false
                }
                InputEvent::Release(required_button) => diff.released_contains(&required_button),
            }
        } else {
            false
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct MotionInput {
    heads: Vec<ParserHead>,
    requirements: Vec<InputEvent>,
}
impl MotionInput {
    pub fn clear(&mut self) {
        self.heads.clear();
    }

    pub fn is_done(&self) -> bool {
        self.heads.iter().any(|head| head.requirement.is_none())
    }

    pub fn advance(&mut self, diff: &Diff, frame: &Frame) {
        if self.is_done() {
            return;
        }

        let new_head = ParserHead::new_from_diff(&self.requirements, &frame.diff_from_neutral());

        if let Some(ref mut existing_head) = self
            .heads
            .iter_mut()
            .find(|head| head.index == new_head.index)
        {
            // There is an existing head with the same index
            existing_head.last_update = Instant::now();
        } else {
            // No existing head
            self.heads.push(new_head);
        }

        self.heads = self
            .heads
            .clone()
            .into_iter()
            .filter_map(|mut head| {
                if head.expired() {
                    None
                } else {
                    head.advance(&self.requirements, diff);
                    Some(head)
                }
            })
            .collect();
    }
}

impl From<&str> for MotionInput {
    fn from(input: &str) -> Self {
        let mut tokens = vec![];
        let mut multichar = None;

        for ch in input.chars() {
            match ch {
                '[' => {
                    assert!(multichar.is_none(), "Nested '['");
                    multichar = Some(String::default());
                }
                ']' => {
                    assert!(multichar.is_some(), "Closing ']' before opener");
                    tokens.push(multichar.unwrap());
                    multichar = None;
                }
                _ => {
                    if let Some(mut temp) = multichar {
                        temp.push(ch);
                        multichar = Some(temp);
                    } else {
                        tokens.push(ch.to_string());
                    }
                }
            }
        }

        assert!(!tokens.is_empty(), "No tokens");

        let requirements: Vec<InputEvent> = tokens
            .into_iter()
            .map(|token| {
                let gts: Vec<InputEvent> = token.chars().map(|char| char.into()).collect();
                if gts.len() == 1 {
                    gts[0].clone()
                } else {
                    match gts[0] {
                        InputEvent::Point(_) => InputEvent::Range(
                            gts.into_iter()
                                .map(|requirement| {
                                    if let InputEvent::Point(stick) = requirement {
                                        stick
                                    } else {
                                        panic!("Mismatched requirements")
                                    }
                                })
                                .collect(),
                        ),
                        InputEvent::Press(_) => InputEvent::MultiPress(
                            gts.into_iter()
                                .map(|requirement| {
                                    if let InputEvent::Press(button) = requirement {
                                        button
                                    } else {
                                        panic!("Mismatched requirements")
                                    }
                                })
                                .collect(),
                        ),
                        _ => panic!("Multiple non multipleable requirements"),
                    }
                }
            })
            .collect();

        Self {
            requirements,
            ..default()
        }
    }
}

#[cfg(test)]
mod test {
    use types::StickPosition;

    use super::*;

    #[test]
    fn hadouken() {
        let parsed: MotionInput = "236f".into();
        assert_eq!(
            parsed.requirements,
            vec![
                InputEvent::Point(StickPosition::S),
                InputEvent::Point(StickPosition::SE),
                InputEvent::Point(StickPosition::E),
                InputEvent::Press(GameButton::Fast),
            ]
        )
    }

    #[test]
    fn head_advancement() {
        let motion: MotionInput = "6f".into();
        let frame = Frame {
            stick_position: StickPosition::E,
            ..default()
        };

        let diff = Diff {
            pressed: Some(vec![GameButton::Fast].into_iter().collect()),
            ..default()
        };

        let mut ph = ParserHead::new_from_diff(&motion.requirements, &frame.diff_from_neutral());
        assert!(ph.index == 1);

        ph.advance(&motion.requirements, &diff);
        assert!(ph.is_done());
    }
}
