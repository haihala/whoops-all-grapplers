use bevy::prelude::*;
use bevy::utils::HashSet;
use types::{GameButton, StickPosition};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
/// Frame is a situation, diff is a change
pub struct Frame {
    pub stick_position: StickPosition,
    pub pressed: HashSet<GameButton>,
}
impl Frame {
    pub fn apply(&mut self, diff: Diff) {
        if let Some(stick) = diff.stick_move {
            self.stick_position = stick;
        }

        if let Some(pressed) = diff.pressed {
            self.pressed = self.pressed.union(&pressed).into_iter().cloned().collect();
        }

        if let Some(released) = diff.released {
            self.pressed.retain(|button| !released.contains(button));
        }
    }

    pub fn diff_from_neutral(&self) -> Diff {
        let stick_move = if self.stick_position == StickPosition::Neutral {
            None
        } else {
            Some(self.stick_position)
        };

        let pressed = if self.pressed.is_empty() {
            None
        } else {
            Some(self.pressed.clone())
        };

        Diff {
            stick_move,
            pressed,
            ..default()
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
/// A single update in input state
pub struct Diff {
    pub stick_move: Option<StickPosition>,
    pub pressed: Option<HashSet<GameButton>>,
    pub released: Option<HashSet<GameButton>>,
}
impl Diff {
    pub fn apply(mut self, event: InputEvent) -> Self {
        match event {
            InputEvent::Point(stick) => self.stick_move = Some(stick),
            InputEvent::Press(button) => self.pressed = Some(add_or_init(self.pressed, button)),
            InputEvent::Release(button) => self.released = Some(add_or_init(self.released, button)),
            InputEvent::MultiPress(_) => panic!("Applying multipress to diff"),
            InputEvent::Range(_) => panic!("Applying range to diff"),
        }

        self
    }
    pub fn pressed_contains(&self, button: &GameButton) -> bool {
        if let Some(pressed) = &self.pressed {
            pressed.contains(button)
        } else {
            false
        }
    }

    pub fn released_contains(&self, button: &GameButton) -> bool {
        if let Some(released) = &self.released {
            released.contains(button)
        } else {
            false
        }
    }
}
fn add_or_init(base: Option<HashSet<GameButton>>, button: GameButton) -> HashSet<GameButton> {
    if let Some(mut pressed) = base {
        pressed.insert(button);
        pressed
    } else {
        vec![button].into_iter().collect()
    }
}

/// Enum used to define move inputs.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InputEvent {
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
