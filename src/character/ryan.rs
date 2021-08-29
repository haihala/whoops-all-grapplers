use bevy::prelude::*;

use super::super::input::{ActionButton, InputBuffer, SpecialMove, StickPosition};

pub struct Ryan;

pub fn ryan(mut query: Query<(&InputBuffer, &mut Transform), With<Ryan>>) {
    for (buffer, mut transform) in query.iter_mut() {
        for special in buffer.interpreted.iter() {
            match special {
                SpecialMove::QuarterCircle => todo!(),
                SpecialMove::BackwardQuarterCircle => todo!(),
            }
        }

        match buffer.stick_position {
            StickPosition::W => {
                transform.translation.x -= 1.0;
            }
            StickPosition::E => {
                transform.translation.x += 1.0;
            }
            _ => (),
        };

        if buffer.recently_pressed.contains(&ActionButton::Fast) {
            dbg!("recent fast");
        }
    }
}
