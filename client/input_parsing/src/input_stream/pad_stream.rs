use bevy::prelude::*;
use wag_core::{GameButton, StickPosition};

use crate::{
    helper_types::{Diff, InputEvent},
    PadReserve, STICK_DEAD_ZONE,
};

use super::{InputStream, ParrotStream};

#[derive(Default, Component)]
pub struct PadStream {
    pub pad_id: Option<Gamepad>,
    next_read: Vec<InputEvent>,
    stick_position: IVec2,
    stick_position_last_read: StickPosition,
}

impl PadStream {
    fn update_next_diff_stick(&mut self) {
        let discrete_stick = self.stick_position.into();
        self.next_read.push(InputEvent::Point(discrete_stick));
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

    fn press_button(&mut self, button: GameButton) {
        self.next_read.push(InputEvent::Press(button));
    }

    fn release_button(&mut self, button: GameButton) {
        self.next_read.push(InputEvent::Release(button));
    }

    fn update_dpad(&mut self, pressed: bool, new_x: Option<i32>, new_y: Option<i32>) {
        // Plan was for opposite presses to override, to make hitbox gaming easier
        // So on release we can't just reset to zero, because the other direction may be held
        // Hopefully this works.
        if pressed {
            if let Some(x) = new_x {
                self.stick_position.x = x;
            }
            if let Some(y) = new_y {
                self.stick_position.y = y;
            }
        } else {
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

        self.update_next_diff_stick();
    }
}

impl InputStream for PadStream {
    fn read(&mut self) -> Option<Diff> {
        let readable = self.pad_id.is_some() && !self.next_read.is_empty();
        if readable {
            let temp = self.next_read.clone();
            self.next_read.clear();
            let mut diff = temp
                .into_iter()
                .fold(Diff::default(), |acc, new| acc.apply(new));

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

    fn is_ready(&self) -> bool {
        self.pad_id.is_some()
    }
}

pub(crate) fn update_pads(
    mut gamepad_events: EventReader<GamepadEvent>,
    mut unused_pads: ResMut<PadReserve>,
    mut readers: Query<(&mut PadStream, &mut ParrotStream)>,
) {
    for GamepadEvent {
        gamepad,
        event_type,
    } in gamepad_events.iter()
    {
        for (mut pad, mut parrot) in &mut readers {
            let pad_in_question = pad.pad_id.is_some() && pad.pad_id.unwrap() == *gamepad;
            let unclaimed_pad = pad.pad_id.is_none();

            match event_type {
                GamepadEventType::Connected(_) => {
                    println!("Connected controller {}", gamepad.id);
                    unused_pads.push_back(*gamepad);
                }
                GamepadEventType::Disconnected => {
                    if pad_in_question {
                        pad.pad_id = None;
                    }
                    if unused_pads.contains(gamepad) {
                        println!("Unplugged controller {}", gamepad.id);
                        unused_pads.remove_pad(gamepad);
                    }
                }
                GamepadEventType::AxisChanged(axis, new_value) => {
                    if pad_in_question {
                        axis_change(&mut pad, *axis, *new_value);
                    }
                }
                GamepadEventType::ButtonChanged(button, new_value) => {
                    let pressed_start = *button == GamepadButtonType::Start && *new_value == 1.0;
                    if unused_pads.contains(gamepad) && unclaimed_pad && pressed_start {
                        // Pressed start, claim the pad
                        println!("Claimed controller {}", gamepad.id);
                        pad.pad_id = Some(*gamepad);
                        unused_pads.remove_pad(gamepad);
                    } else if pad_in_question {
                        button_change(&mut pad, &mut parrot, *button, *new_value);
                    }
                }
            }
        }
    }
}

fn axis_change(reader: &mut Mut<PadStream>, axis: GamepadAxisType, new_value: f32) {
    match axis {
        // Even though DPad axis are on the list, they don't fire
        GamepadAxisType::LeftStickX | GamepadAxisType::RightStickX => reader.update_stick(
            Some(if new_value.abs() > STICK_DEAD_ZONE {
                new_value.signum() as i32
            } else {
                0
            }),
            None,
        ),
        GamepadAxisType::LeftStickY | GamepadAxisType::RightStickY => reader.update_stick(
            None,
            Some(if new_value.abs() > STICK_DEAD_ZONE {
                new_value.signum() as i32
            } else {
                0
            }),
        ),
        _ => {}
    }
}

fn button_change(
    reader: &mut Mut<PadStream>,
    parrot: &mut Mut<ParrotStream>,
    button: GamepadButtonType,
    new_value: f32,
) {
    // TODO: real button mappings

    let press = new_value > 0.1;
    let handle_gamebutton = move |reader: &mut Mut<PadStream>, button: GameButton| {
        if press {
            reader.press_button(button)
        } else {
            reader.release_button(button)
        }
    };

    match button {
        GamepadButtonType::Start => handle_gamebutton(reader, GameButton::Start),

        GamepadButtonType::South => handle_gamebutton(reader, GameButton::Fast),
        GamepadButtonType::East => handle_gamebutton(reader, GameButton::Strong),
        GamepadButtonType::North => handle_gamebutton(reader, GameButton::Wrestling),
        GamepadButtonType::West => handle_gamebutton(reader, GameButton::Gimmick),

        GamepadButtonType::DPadUp => reader.update_dpad(press, None, Some(1)),
        GamepadButtonType::DPadDown => reader.update_dpad(press, None, Some(-1)),
        GamepadButtonType::DPadLeft => reader.update_dpad(press, Some(-1), None),
        GamepadButtonType::DPadRight => reader.update_dpad(press, Some(1), None),

        GamepadButtonType::Select => {
            if press {
                parrot.cycle()
            }
        }
        _ => {}
    }
}
