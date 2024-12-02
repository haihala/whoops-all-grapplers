use bevy::{reflect::Reflect, utils::HashSet};

use wag_core::{GameButton, InputEvent, StickPosition};

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

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Default)]
pub enum RequirementMode {
    All(Vec<InputEvent>),
    Any(Vec<InputEvent>),
    Anything,
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
            RequirementMode::Anything => false,
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

    pub fn is_empty(&self) -> bool {
        self.stick.is_empty() && self.buttons.is_empty()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Default)]
pub struct InputRequirement {
    pub mode: RequirementMode,
    pub state_requirement: StateRequirement,
    pub sticky: bool,
}
