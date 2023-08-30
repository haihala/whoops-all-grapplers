use bevy::prelude::*;
use bevy::utils::Instant;

use wag_core::GameButton;

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
    fn from_frame(requirements: &[InputEvent], prev_state: Frame) -> ParserHead {
        let mut new = ParserHead {
            requirement: requirements.get(0).cloned(),
            ..default()
        };

        new.advance(
            requirements,
            &Diff {
                stick_move: Some(prev_state.stick_position),
                pressed: if !prev_state.pressed.is_empty() {
                    Some(prev_state.pressed)
                } else {
                    None
                },
                ..default()
            },
        );
        new
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

    pub fn advance(&mut self, diff: &Diff, prev_state: Frame) {
        if self.is_done() {
            return;
        }

        let new_head = ParserHead::from_frame(&self.requirements, prev_state);

        if !new_head.is_done() {
            // The previous input state has triggered this event.
            // It ought to have been reacted to already, so don't bother adding it into the pool.

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
    use map_macro::hash_set;
    use wag_core::StickPosition;

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

        let diff = Diff {
            pressed: Some(hash_set! {GameButton::Fast}),
            ..default()
        };

        let mut ph = ParserHead::from_frame(
            &motion.requirements,
            Frame {
                stick_position: StickPosition::E,
                ..default()
            },
        );
        assert!(ph.index == 1);

        ph.advance(&motion.requirements, &diff);
        assert!(ph.is_done());
    }
}
