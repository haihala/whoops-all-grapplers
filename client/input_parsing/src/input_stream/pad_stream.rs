use bevy::prelude::*;
use wag_core::{Controllers, GameButton, Player, StickPosition, WagInputButton, WagInputEvent};

use crate::helper_types::{Diff, InputEvent};

use super::{InputStream, ParrotStream};

#[derive(Default, Component, Clone, Reflect)]
pub struct PadStream {
    next_read: Vec<InputEvent>,
    stick_position: IVec2,
    stick_position_last_read: StickPosition,
}

impl PadStream {
    fn update_next_diff_stick(&mut self) {
        let discrete_stick = self.stick_position.into();
        self.next_read.push(InputEvent::Point(discrete_stick));
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
        if self.next_read.is_empty() {
            return None;
        }

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
    }
}

pub fn update_pads(
    mut gamepad_events: EventReader<WagInputEvent>,
    mut readers: Query<(&mut PadStream, &mut ParrotStream, &Player)>,
    controllers: Res<Controllers>,
) {
    for event in gamepad_events.read() {
        for (mut pad, mut parrot, player) in &mut readers {
            let pad_id = controllers.get_handle(*player);

            if pad_id != event.player_handle {
                continue;
            }

            button_change(&mut pad, &mut parrot, event.button, event.pressed);
        }
    }
}

fn button_change(
    reader: &mut Mut<PadStream>,
    parrot: &mut Mut<ParrotStream>,
    button: WagInputButton,
    press: bool,
) {
    let handle_gamebutton = if press {
        move |reader: &mut Mut<PadStream>, button: GameButton| reader.press_button(button)
    } else {
        move |reader: &mut Mut<PadStream>, button: GameButton| reader.release_button(button)
    };

    match button {
        WagInputButton::Start => handle_gamebutton(reader, GameButton::Start),

        WagInputButton::South => handle_gamebutton(reader, GameButton::Fast),
        WagInputButton::East => handle_gamebutton(reader, GameButton::Strong),
        WagInputButton::North => handle_gamebutton(reader, GameButton::Wrestling),
        WagInputButton::West => handle_gamebutton(reader, GameButton::Gimmick),

        WagInputButton::Up => reader.update_dpad(press, None, Some(1)),
        WagInputButton::Down => reader.update_dpad(press, None, Some(-1)),
        WagInputButton::Left => reader.update_dpad(press, Some(-1), None),
        WagInputButton::Right => reader.update_dpad(press, Some(1), None),

        WagInputButton::Select => {
            if press {
                parrot.cycle()
            }
        }
        _ => {}
    }
}
