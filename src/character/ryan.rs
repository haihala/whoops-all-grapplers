use bevy::prelude::*;

use crate::input::{ActionButton, InputBuffer, SpecialMove, StickPosition};

pub struct Ryan;

pub fn ryan(
    mut query: Query<
        (
            &InputBuffer,
            &crate::player::PlayerState,
            &mut crate::physics::PhysicsObject,
        ),
        With<Ryan>,
    >,
    time: Res<Time>,
) {
    for (buffer, state, mut physics_object) in query.iter_mut() {
        for special in buffer.interpreted.iter() {
            match special {
                SpecialMove::QuarterCircle => todo!(),
                SpecialMove::BackwardQuarterCircle => todo!(),
            }
        }

        match buffer.stick_position {
            StickPosition::W => {
                physics_object.velocity.x -=
                    crate::constants::PLAYER_ACCELERATION * time.delta_seconds();
            }
            StickPosition::E => {
                physics_object.velocity.x +=
                    crate::constants::PLAYER_ACCELERATION * time.delta_seconds();
            }
            _ => (),
        };

        if state.grounded && buffer.stick_position == StickPosition::N {
            physics_object.velocity.y += crate::constants::PLAYER_JUMP_VELOCITY;
        }

        if buffer.recently_pressed.contains(&ActionButton::Fast) {
            dbg!("recent fast");
        }
    }
}
