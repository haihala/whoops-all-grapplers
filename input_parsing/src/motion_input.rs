use bevy::utils::Instant;

use types::{GameButton, StickPosition};

use crate::{
    helper_types::{Diff, Frame},
    CHARGE_TIME, MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS,
};

/// Enum used to define move inputs.
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// Prefix. Next requirement must be held for some time
    Charge,
    /// Stick must visit a point
    Point(StickPosition),
    /// Stick must visit one of the following points
    Range(Vec<StickPosition>),
    /// Press a button
    Press(GameButton),
    /// Press all of the following buttons
    MultiPress(Vec<GameButton>),
    /// Release a button
    Release(GameButton),
}
impl From<char> for InputEvent {
    fn from(ch: char) -> InputEvent {
        if let Ok(number_token) = ch.to_string().parse::<i32>() {
            InputEvent::Point(number_token.into())
        } else {
            match ch {
                'c' => InputEvent::Charge,
                'f' => InputEvent::Press(GameButton::Fast),
                'F' => InputEvent::Release(GameButton::Fast),
                's' => InputEvent::Press(GameButton::Strong),
                'S' => InputEvent::Release(GameButton::Strong),
                'g' => InputEvent::Press(GameButton::Grab),
                'G' => InputEvent::Release(GameButton::Grab),
                'e' => InputEvent::Press(GameButton::Equipment),
                'E' => InputEvent::Release(GameButton::Equipment),
                't' => InputEvent::Press(GameButton::Taunt),
                'T' => InputEvent::Release(GameButton::Taunt),
                _ => panic!("Invalid character {}", ch),
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct ParserHead {
    index: usize,
    last_update: Option<Instant>,
    /// None if complete
    requirement: Option<InputEvent>,
    charge_started: Option<Instant>,
    multipresses_received: Vec<GameButton>,
}
impl ParserHead {
    fn new_from_diff(requirements: Vec<InputEvent>, diff: &Diff) -> ParserHead {
        let mut new = ParserHead::new(requirements.get(0).cloned());
        new.advance(requirements, diff);
        new
    }

    fn new(requirement: Option<InputEvent>) -> ParserHead {
        ParserHead {
            requirement,
            ..Default::default()
        }
    }

    fn is_done(&self) -> bool {
        self.requirement.is_none()
    }

    fn expired(&self) -> bool {
        let now = Instant::now();
        now.duration_since(self.last_update.unwrap_or(now))
            .as_secs_f32()
            > MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS
            && self.charge_started.is_none()
    }

    fn bump(&mut self, requirement: Option<InputEvent>) {
        *self = ParserHead {
            requirement,
            index: self.index + 1,
            last_update: Some(Instant::now()),
            ..Default::default()
        }
    }

    fn double_bump(&mut self, requirement: Option<InputEvent>) {
        *self = ParserHead {
            requirement,
            index: self.index + 2,
            last_update: Some(Instant::now()),
            ..Default::default()
        }
    }

    fn advance(&mut self, requirements: Vec<InputEvent>, diff: &Diff) {
        if self.is_done() {
            return;
        }

        let starting_index = self.index;
        let current_requirement = self.requirement.clone().unwrap();
        let next_requirement = self.get_next_requirement(&requirements);

        match current_requirement {
            InputEvent::Charge => {
                let now = Instant::now();
                let requirement_met = self.requirement_met(next_requirement.unwrap(), diff);

                if let Some(charge_start) = self.charge_started {
                    if now.duration_since(charge_start).as_secs_f32() > CHARGE_TIME {
                        // Charge is done
                        let post_charge_requirement =
                            self.get_requirement_with_offset(&requirements, 2);
                        self.double_bump(post_charge_requirement);
                    } else if !requirement_met {
                        self.charge_started = None;
                    }
                } else if requirement_met {
                    // Start charge
                    self.charge_started = Some(now);
                }
            }
            _ => {
                if self.requirement_met(current_requirement, diff) {
                    self.bump(next_requirement);
                }
            }
        }

        if self.index != starting_index {
            // A bump has happened, maybe multiple announcements can happen on the same tick.
            self.advance(requirements, diff);
        }
    }

    fn get_next_requirement(&self, requirements: &[InputEvent]) -> Option<InputEvent> {
        self.get_requirement_with_offset(requirements, 1)
    }

    fn get_requirement_with_offset(
        &self,
        requirements: &[InputEvent],
        offset: usize,
    ) -> Option<InputEvent> {
        requirements.get(self.index + offset).cloned()
    }

    fn requirement_met(&mut self, requirement: InputEvent, diff: &Diff) -> bool {
        match requirement {
            InputEvent::Charge => {
                panic!(
                    "Charge getting here means there were two consecutive charges in the definition"
                );
            }
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

        let new_head =
            ParserHead::new_from_diff(self.requirements.clone(), &frame.diff_from_neutral());

        if !self.heads.iter().any(|head| head.index == new_head.index) {
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
                    head.advance(self.requirements.clone(), diff);
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

        assert!(
            !matches!(requirements.last(), Some(InputEvent::Charge)),
            "Last requirement can't be a prefix"
        );

        Self {
            requirements,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
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
    fn simple_sonic_boom() {
        let parsed: MotionInput = "c46f".into();
        assert_eq!(
            parsed.requirements,
            vec![
                InputEvent::Charge,
                InputEvent::Point(StickPosition::W),
                InputEvent::Point(StickPosition::E),
                InputEvent::Press(GameButton::Fast),
            ]
        )
    }

    #[test]
    fn real_sonic_boom() {
        let parsed: MotionInput = "c[741][63]f".into();
        assert_eq!(
            parsed.requirements,
            vec![
                InputEvent::Charge,
                InputEvent::Range(vec![StickPosition::NW, StickPosition::W, StickPosition::SW,]),
                InputEvent::Range(vec![StickPosition::E, StickPosition::SE]),
                InputEvent::Press(GameButton::Fast),
            ]
        )
    }

    #[test]
    fn zonk() {
        let parsed: MotionInput = "cfF".into();
        assert_eq!(
            parsed.requirements,
            vec![
                InputEvent::Charge,
                InputEvent::Press(GameButton::Fast),
                InputEvent::Release(GameButton::Fast),
            ]
        )
    }
}
