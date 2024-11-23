use bevy::prelude::*;
use wag_core::{Controllers, GameButton, Player, WagArgs, WagInputButton, WagInputEventStream};

use crate::helper_types::InputEvent;

use super::{InputStream, ParrotStream};

#[derive(Default, Component, Clone, Reflect)]
pub struct PadStream {
    next_read: Vec<InputEvent>,
    stick_position: IVec2,
}

impl PadStream {
    fn press_button(&mut self, button: GameButton) {
        self.next_read.push(InputEvent::Press(button));
    }

    fn release_button(&mut self, button: GameButton) {
        self.next_read.push(InputEvent::Release(button));
    }

    fn update_dpad(&mut self, pressed: bool, direction: IVec2) {
        if pressed {
            self.stick_position += direction;
        } else {
            self.stick_position -= direction;
        }

        // TODO: Analog stick + dpad is broken, this is a temp fix
        // Stick parsing should be moved over to earlier in the parsing pipeline
        let sp = IVec2::new(
            self.stick_position.x.signum(),
            self.stick_position.y.signum(),
        );
        //assert!(self.stick_position.x.abs() < 2);
        //assert!(self.stick_position.y.abs() < 2);

        self.next_read.push(InputEvent::Point(sp.into()));
    }
}

impl InputStream for PadStream {
    fn read(&mut self) -> Vec<InputEvent> {
        let temp = self.next_read.clone();
        self.next_read.clear();
        temp
    }
}

pub fn update_pads(
    gamepad_events: Res<WagInputEventStream>,
    mut readers: Query<(&mut PadStream, &mut ParrotStream, &Player)>,
    controllers: Res<Controllers>,
    args: Res<WagArgs>,
) {
    for event in gamepad_events.events.iter() {
        for (mut pad, mut parrot, player) in &mut readers {
            let pad_id = controllers.get_handle(*player);

            if pad_id == event.player_handle {
                button_change(
                    &mut pad,
                    &mut parrot,
                    event.button,
                    event.pressed,
                    args.dev.is_some(),
                );
            }
        }
    }
}

fn button_change(
    reader: &mut Mut<PadStream>,
    parrot: &mut Mut<ParrotStream>,
    button: WagInputButton,
    press: bool,
    dev_mode: bool,
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

        WagInputButton::Up => reader.update_dpad(press, IVec2::Y),
        WagInputButton::Down => reader.update_dpad(press, -1 * IVec2::Y),
        WagInputButton::Left => reader.update_dpad(press, -1 * IVec2::X),
        WagInputButton::Right => reader.update_dpad(press, IVec2::X),

        WagInputButton::Select => {
            if press && dev_mode {
                parrot.cycle()
            }
        }
        _ => {}
    }
}
