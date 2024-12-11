use bevy::{prelude::*, utils::HashSet};

use super::{GameButton, InputEvent, StickPosition};

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

    pub fn changes_to(&self, new_state: &InputState) -> Vec<InputEvent> {
        let mut out = vec![];

        if self.stick_position != new_state.stick_position {
            out.push(InputEvent::Point(new_state.stick_position));
        }

        // Pressed
        for btn in new_state.pressed.iter() {
            if !self.pressed.contains(btn) {
                out.push(InputEvent::Press(*btn));
            }
        }

        // Released
        for btn in self.pressed.iter() {
            if !new_state.pressed.contains(btn) {
                out.push(InputEvent::Release(*btn));
            }
        }

        out
    }
}
