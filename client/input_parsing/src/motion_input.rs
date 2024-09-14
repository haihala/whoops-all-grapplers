use bevy::prelude::*;
use bevy::utils::Instant;

use crate::{
    helper_types::{Diff, Frame, InputEvent, InputRequirement},
    MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS,
};

#[derive(Debug, Clone, Eq, PartialEq, Reflect)]
struct ParserHead {
    index: usize,
    last_update: Instant,
    /// None if complete
    requirement: Option<InputRequirement>,
    prev_requirement: Option<InputRequirement>,
}

impl Default for ParserHead {
    fn default() -> Self {
        Self {
            index: default(),
            last_update: Instant::now(),
            requirement: default(),
            prev_requirement: default(),
        }
    }
}
impl ParserHead {
    fn from_frame(requirements: &[InputRequirement], prev_state: Frame) -> ParserHead {
        let mut new = ParserHead {
            requirement: requirements.first().cloned(),
            ..default()
        };

        new.advance(
            requirements,
            &Frame::default(),
            &prev_state.clone().diff_from_neutral(),
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

    fn bump(&mut self, requirements: &[InputRequirement]) {
        *self = ParserHead {
            requirement: requirements.get(self.index + 1).cloned(),
            index: self.index + 1,
            prev_requirement: self.requirement.clone(),
            ..default()
        };
    }

    fn advance(&mut self, requirements: &[InputRequirement], base: &Frame, diff: &Diff) {
        while !self.is_done() && self.requirements_met(base, diff) {
            self.bump(requirements);
        }
    }

    fn requirements_met(&mut self, base: &Frame, diff: &Diff) -> bool {
        let Some(requirement) = self.requirement.clone() else {
            return false;
        };

        if requirement.sticky
            && !self.requirement_passes(
                self.prev_requirement.clone().unwrap(),
                &base.clone().diff_from_neutral(),
            )
        {
            return false;
        }

        self.requirement_passes(requirement, diff)
    }

    fn requirement_passes(&mut self, requirement: InputRequirement, diff: &Diff) -> bool {
        requirement.events.iter().any(|event| match event {
            InputEvent::Point(required_stick) => {
                diff.stick_move.is_some() && diff.stick_move.unwrap() == *required_stick
            }
            InputEvent::Press(required_button) => diff.pressed_contains(required_button),
            InputEvent::Release(required_button) => diff.released_contains(required_button),
        })
    }
}

#[derive(Default, Debug, Clone, Reflect)]
pub struct MotionInput {
    heads: Vec<ParserHead>,
    requirements: Vec<InputRequirement>,
}
impl MotionInput {
    pub fn clear(&mut self) {
        self.heads.clear();
    }

    pub fn is_done(&self) -> bool {
        self.heads.iter().any(|head| head.requirement.is_none())
    }

    pub fn complexity(&self) -> usize {
        self.requirements.len()
    }

    pub fn advance(&mut self, diff: &Diff, prev_state: Frame) {
        if self.is_done() {
            return;
        }

        let new_head = ParserHead::from_frame(&self.requirements, prev_state.clone());

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
                    head.advance(&self.requirements, &prev_state, diff);
                    Some(head)
                }
            })
            .collect();
    }
}

impl From<&str> for MotionInput {
    fn from(input: &str) -> Self {
        // Vec of tuples, first part is the input to be parsed, second is if it's sticky
        let mut tokens = vec![];
        let mut multichar = None;
        let mut sticky = false;

        for ch in input.chars() {
            match ch {
                '[' => {
                    assert!(multichar.is_none(), "Nested '['");
                    multichar = Some(String::default());
                }
                ']' => {
                    assert!(multichar.is_some(), "Closing ']' before opener");
                    tokens.push((multichar.unwrap(), sticky));
                    sticky = false;
                    multichar = None;
                }
                '+' => {
                    sticky = true;
                }
                _ => {
                    if let Some(mut temp) = multichar {
                        temp.push(ch);
                        multichar = Some(temp);
                    } else {
                        tokens.push((ch.to_string(), sticky));
                        sticky = false;
                    }
                }
            }
        }

        assert!(!tokens.is_empty(), "No tokens");

        let requirements: Vec<InputRequirement> = tokens
            .into_iter()
            .map(|(symbols, sticky)| {
                let events: Vec<InputEvent> = symbols.chars().map(|char| char.into()).collect();

                InputRequirement { sticky, events }
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
    use wag_core::{GameButton, StickPosition};

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
            .into_iter()
            .map(InputRequirement::from)
            .collect::<Vec<_>>()
        )
    }

    #[test]
    fn sticky() {
        let parsed: MotionInput = "6+f".into();
        assert_eq!(
            parsed.requirements,
            vec![
                InputRequirement {
                    sticky: false,
                    events: vec![InputEvent::Point(StickPosition::E)]
                },
                InputRequirement {
                    sticky: true,
                    events: vec![InputEvent::Press(GameButton::Fast)]
                },
            ]
        )
    }

    #[test]
    fn head_advancement() {
        let motion: MotionInput = "6f".into();

        let state = Frame {
            stick_position: StickPosition::E,
            ..default()
        };
        let diff = Diff {
            pressed: Some(vec![GameButton::Fast].into_iter().collect()),
            ..default()
        };

        let mut ph = ParserHead::from_frame(&motion.requirements, state.clone());
        assert!(ph.index == 1);

        ph.advance(&motion.requirements, &state, &diff);
        assert!(ph.is_done());
    }

    #[test]
    fn sticky_head_advancement() {
        let mut sticky: MotionInput = "6+f".into();

        let mut base = Frame::default();
        let forward = Diff {
            stick_move: Some(StickPosition::E),
            ..default()
        };

        sticky.advance(&forward, base.clone());
        base.apply(forward);

        sticky.advance(
            &Diff {
                pressed: Some(vec![GameButton::Fast].into_iter().collect()),
                ..default()
            },
            base.clone(),
        );
        assert!(sticky.is_done());
    }

    #[test]
    fn sticky_head_advancement_limits_correctly() {
        let mut non_sticky: MotionInput = "6f".into();
        let mut sticky: MotionInput = "6+f".into();

        let mut base = Frame::default();
        let forward = Diff {
            stick_move: Some(StickPosition::E),
            ..default()
        };

        non_sticky.advance(&forward, base.clone());
        sticky.advance(&forward, base.clone());
        base.apply(forward.clone());

        let neutral = Diff {
            stick_move: Some(StickPosition::Neutral),
            ..default()
        };

        non_sticky.advance(&neutral, base.clone());
        sticky.advance(&neutral, base.clone());
        base.apply(neutral);

        let button = Diff {
            pressed: Some(vec![GameButton::Fast].into_iter().collect()),
            ..default()
        };

        non_sticky.advance(&button, base.clone());
        sticky.advance(&button, base.clone());

        assert!(non_sticky.is_done());
        assert!(!sticky.is_done());
    }

    #[test]
    fn range() {
        let mut input: MotionInput = "[123]".into();

        let down = Diff {
            stick_move: Some(StickPosition::S),
            ..default()
        };

        input.advance(&down, Frame::default());
        assert!(input.is_done());
    }
}
