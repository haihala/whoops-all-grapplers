use bevy::utils::HashSet;

use wag_core::{GameButton, StickPosition};

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
            self.pressed = self.pressed.union(&pressed).cloned().collect();
        }

        if let Some(released) = diff.released {
            self.pressed.retain(|button| !released.contains(button));
        }
    }

    pub fn diff_from_neutral(self) -> Diff {
        Diff {
            stick_move: Some(self.stick_position),
            pressed: if self.pressed.is_empty() {
                None
            } else {
                Some(self.pressed)
            },
            released: None,
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

    pub fn mirrored(mut self) -> Self {
        self.stick_move = self.stick_move.map(StickPosition::mirror);
        self
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InputEvent {
    Point(StickPosition),
    Press(GameButton),
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
                'w' => InputEvent::Press(GameButton::Wrestling),
                'W' => InputEvent::Release(GameButton::Wrestling),
                'g' => InputEvent::Press(GameButton::Gimmick),
                'G' => InputEvent::Release(GameButton::Gimmick),
                // There is no need for negative edge on start, this whole thing is mighty sus so let's not get caught up on that shall we
                '.' => InputEvent::Press(GameButton::Start),
                _ => panic!("Invalid character {ch}"),
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InputRequirement {
    pub sticky: bool,
    pub events: Vec<InputEvent>,
}
impl From<InputEvent> for InputRequirement {
    fn from(event: InputEvent) -> InputRequirement {
        InputRequirement {
            sticky: false,
            events: vec![event],
        }
    }
}
