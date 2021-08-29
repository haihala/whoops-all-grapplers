use bevy::prelude::*;

use super::super::input::{ActionButton, Controller, InputBuffer, SpecialMove, StickPosition};

pub struct Ryan;

pub fn ryan(mut query: Query<(&InputBuffer, &Controller, &mut Transform), With<Ryan>>) {
    for (buffer, controller, mut transform) in query.iter_mut() {
        for special in buffer.interpreted.iter() {
            match special {
                SpecialMove::QuarterCircle => {
                    for frame in buffer.frames.iter() {
                        for btn in frame.pressed.iter() {
                            match btn {
                                ActionButton::Vicious => todo!(),
                                ActionButton::Fast => todo!(),
                            }
                        }
                    }
                }
                SpecialMove::BackwardQuarterCircle => todo!(),
            }
        }

        match controller.1 {
            StickPosition::W => {
                transform.translation.x -= 1.0;
            }
            StickPosition::E => {
                transform.translation.x += 1.0;
            }
            _ => (),
        };
    }
}
