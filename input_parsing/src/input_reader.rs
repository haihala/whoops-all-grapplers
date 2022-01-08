use std::collections::VecDeque;

use bevy::prelude::*;
use types::{GameButton, StickPosition};

use crate::{
    helper_types::{ButtonUpdate, Diff, InputChange},
    STICK_DEAD_ZONE,
};

#[derive(Default)]
pub struct InputReader {
    pub pad_id: Option<Gamepad>,
    next_read: Vec<InputChange>,
    stick_position: IVec2,
    stick_position_last_read: StickPosition,
}

impl InputReader {
    fn update_next_diff_stick(&mut self) {
        let discrete_stick = self.stick_position.into();
        self.next_read.push(InputChange::Stick(discrete_stick));
    }

    fn update_stick(&mut self, new_x: Option<i32>, new_y: Option<i32>) {
        if let Some(x) = new_x {
            self.stick_position.x = x;
        }
        if let Some(y) = new_y {
            self.stick_position.y = y;
        }

        self.update_next_diff_stick();
    }

    fn update_button(&mut self, button: GameButton, update: ButtonUpdate) {
        self.next_read.push(InputChange::Button(button, update));
    }

    fn update_dpad(&mut self, update: ButtonUpdate, new_x: Option<i32>, new_y: Option<i32>) {
        // Plan was for opposite presses to override, to make hitbox gaming easier
        // So on release we can't just reset to zero, because the other direction may be held
        // Hopefully this works.
        match update {
            ButtonUpdate::Pressed => {
                if let Some(x) = new_x {
                    self.stick_position.x = x;
                }
                if let Some(y) = new_y {
                    self.stick_position.y = y;
                }
            }
            ButtonUpdate::Released => {
                if let Some(x) = new_x {
                    if self.stick_position.x == x {
                        self.stick_position.x = 0;
                    }
                }
                if let Some(y) = new_y {
                    if self.stick_position.y == y {
                        self.stick_position.y = 0;
                    }
                }
            }
        }

        self.update_next_diff_stick();
    }

    pub fn readable(&self) -> bool {
        self.pad_id.is_some() && !self.next_read.is_empty()
    }
    pub fn read(&mut self) -> Option<Diff> {
        if self.readable() {
            let temp = self.next_read.clone();
            self.next_read.clear();
            let mut diff = temp
                .into_iter()
                .fold(Diff::default(), |acc, new| acc.apply(&new));

            if let Some(new_stick) = diff.stick_move {
                if new_stick == self.stick_position_last_read {
                    diff.stick_move = None
                }
                self.stick_position_last_read = new_stick;
            }
            Some(diff)
        } else {
            None
        }
    }

    #[cfg(test)]
    pub fn push(&mut self, change: InputChange) {
        self.next_read.push(change);
    }

    #[cfg(test)]
    pub fn with_pad(pad_id: Gamepad) -> InputReader {
        InputReader {
            pad_id: Some(pad_id),
            ..Default::default()
        }
    }
}

pub fn update_readers(
    mut gamepad_events: EventReader<GamepadEvent>,
    mut unused_pads: ResMut<VecDeque<Gamepad>>,
    mut readers: Query<&mut InputReader>,
) {
    for GamepadEvent(pad_id, event_type) in gamepad_events.iter() {
        let matching_reader = readers
            .iter_mut()
            .find(|reader| reader.pad_id.is_some() && reader.pad_id.unwrap() == *pad_id);

        match event_type {
            GamepadEventType::Connected => {
                pad_connection(pad_id, &mut unused_pads, &mut readers);
            }
            GamepadEventType::Disconnected => {
                pad_disconnection(&mut matching_reader.unwrap(), &mut unused_pads);
            }
            GamepadEventType::AxisChanged(axis, new_value) => {
                axis_change(&mut matching_reader.unwrap(), *axis, *new_value);
            }
            GamepadEventType::ButtonChanged(button, new_value) => {
                button_change(&mut matching_reader.unwrap(), *button, *new_value);
            }
        };
    }
}

fn pad_connection(
    pad_id: &Gamepad,
    unused_pads: &mut ResMut<VecDeque<Gamepad>>,
    readers: &mut Query<&mut InputReader>,
) {
    println!("New gamepad connected with ID: {:?}", pad_id);
    let unused_reader = readers.iter_mut().find(|reader| reader.pad_id.is_none());

    if let Some(mut reader) = unused_reader {
        // There is a free character
        reader.pad_id = Some(*pad_id);
    } else {
        unused_pads.push_back(*pad_id);
    }
}

fn pad_disconnection(reader: &mut Mut<InputReader>, unused_pads: &mut ResMut<VecDeque<Gamepad>>) {
    println!("Gamepad disconnected with ID: {:?}", reader.pad_id);

    reader.pad_id = unused_pads.pop_front();
}

fn axis_change(reader: &mut Mut<InputReader>, axis: GamepadAxisType, new_value: f32) {
    match axis {
        // Even though DPad axis are on the list, they don't fire
        GamepadAxisType::LeftStickX | GamepadAxisType::RightStickX | GamepadAxisType::DPadX => {
            reader.update_stick(
                Some(if new_value.abs() > STICK_DEAD_ZONE {
                    new_value.signum() as i32
                } else {
                    0
                }),
                None,
            )
        }
        GamepadAxisType::LeftStickY | GamepadAxisType::RightStickY | GamepadAxisType::DPadY => {
            reader.update_stick(
                None,
                Some(if new_value.abs() > STICK_DEAD_ZONE {
                    new_value.signum() as i32
                } else {
                    0
                }),
            )
        }
        // No clue what these are
        GamepadAxisType::LeftZ => todo!(),
        GamepadAxisType::RightZ => todo!(),
    }
}

fn button_change(reader: &mut Mut<InputReader>, button: GamepadButtonType, new_value: f32) {
    // TODO: real button mappings

    let update = if new_value > 0.1 {
        ButtonUpdate::Pressed
    } else {
        ButtonUpdate::Released
    };

    match button {
        GamepadButtonType::South => reader.update_button(GameButton::Light, update),
        GamepadButtonType::East => reader.update_button(GameButton::Heavy, update),

        GamepadButtonType::DPadUp => reader.update_dpad(update, None, Some(1)),
        GamepadButtonType::DPadDown => reader.update_dpad(update, None, Some(-1)),
        GamepadButtonType::DPadLeft => reader.update_dpad(update, Some(-1), None),
        GamepadButtonType::DPadRight => reader.update_dpad(update, Some(1), None),
        _ => {}
    }
}
