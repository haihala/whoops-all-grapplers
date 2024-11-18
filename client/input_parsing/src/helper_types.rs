use bevy::{reflect::Reflect, utils::HashSet};

use wag_core::{GameButton, StickPosition};

#[derive(Clone, Eq, PartialEq, Debug, Default, Reflect)]
pub struct InputState {
    pub stick_position: StickPosition,
    pub pressed: HashSet<GameButton>,
}
impl InputState {
    pub fn apply(&mut self, event: InputEvent) {
        match event {
            InputEvent::Point(stick_position) => {
                self.stick_position = stick_position;
            }
            InputEvent::Press(game_button) => {
                self.pressed.insert(game_button);
            }
            InputEvent::Release(game_button) => {
                self.pressed.remove(&game_button);
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Copy)]
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

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Default)]
pub enum RequirementMode {
    All(Vec<InputEvent>),
    Any(Vec<InputEvent>),
    #[default]
    None,
}
impl RequirementMode {
    pub(crate) fn is_negated_by(&self, event: InputEvent) -> bool {
        match self {
            RequirementMode::All(vec) | RequirementMode::Any(vec) => {
                let sticks: Vec<StickPosition> = vec
                    .iter()
                    .filter_map(|ev| {
                        if let InputEvent::Point(dir) = ev {
                            Some(*dir)
                        } else {
                            None
                        }
                    })
                    .collect();
                let presses: Vec<GameButton> = vec
                    .iter()
                    .filter_map(|ev| {
                        if let InputEvent::Press(btn) = ev {
                            Some(*btn)
                        } else {
                            None
                        }
                    })
                    .collect();
                let releases: Vec<GameButton> = vec
                    .iter()
                    .filter_map(|ev| {
                        if let InputEvent::Release(btn) = ev {
                            Some(*btn)
                        } else {
                            None
                        }
                    })
                    .collect();

                match event {
                    InputEvent::Point(stick_position) => !sticks.contains(&stick_position),
                    InputEvent::Press(game_button) => releases.contains(&game_button),
                    InputEvent::Release(game_button) => presses.contains(&game_button),
                }
            }
            RequirementMode::None => panic!("How did we get here?"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Default)]
pub struct StateRequirement {
    pub stick: Vec<StickPosition>,
    pub buttons: Vec<(GameButton, bool)>,
}
impl StateRequirement {
    pub(crate) fn met_by(&self, state: InputState) -> bool {
        if !self.stick.is_empty() && !self.stick.contains(&state.stick_position) {
            return false;
        }

        for (btn, require_pressed) in &self.buttons {
            let is_pressed = state.pressed.contains(btn);
            if *require_pressed != is_pressed {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Default)]
pub struct InputRequirement {
    pub mode: RequirementMode,
    pub state_requirement: StateRequirement,
    pub sticky: bool,
}
