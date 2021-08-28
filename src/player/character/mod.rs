use bevy::prelude::*;

use super::{ActionButton, InputBuffer, SpecialMove};
pub struct Ryan;

pub fn ryan(query: Query<&InputBuffer, With<Ryan>>) {
    for buffer in query.iter() {
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
    }
}
