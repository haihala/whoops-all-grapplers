mod ryan;

pub use ryan::{ryan_executor, ryan_parser, Ryan, RyanMoveBuffer};

use crate::{input::InputStore, input::StickPosition, physics::PhysicsObject};

#[derive(Debug)]
pub enum CharacterAction {
    Left,
    Right,
    Jump,
    LeftJump,
    RightJump,
}

fn parse_character_action(input_buffer: &InputStore, grounded: bool) -> Option<CharacterAction> {
    match input_buffer.stick_position {
        StickPosition::W => Some(CharacterAction::Left),
        StickPosition::E => Some(CharacterAction::Right),
        StickPosition::N => {
            if grounded {
                Some(CharacterAction::Jump)
            } else {
                None
            }
        }
        StickPosition::NW => {
            if grounded {
                Some(CharacterAction::LeftJump)
            } else {
                None
            }
        }
        StickPosition::NE => {
            if grounded {
                Some(CharacterAction::RightJump)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn handle_character_action(
    character_action: &Option<CharacterAction>,
    state: &crate::player::PlayerState,
    physics_object: &mut PhysicsObject,
    delta_time: f32,
) {
    let run_speed =
        crate::constants::PLAYER_INITIAL_RUN_SPEED.max(crate::constants::PLAYER_TOP_SPEED.min(
            physics_object.ground_speed.abs() + crate::constants::PLAYER_ACCELERATION * delta_time,
        ));

    physics_object.ground_speed = 0.0;
    if let Some(action) = character_action {
        match action {
            CharacterAction::Right => {
                physics_object.ground_speed = if state.flipped {
                    crate::constants::PLAYER_WALK_SPEED
                } else {
                    run_speed
                };
            }
            CharacterAction::Left => {
                physics_object.ground_speed = if state.flipped {
                    -run_speed
                } else {
                    -crate::constants::PLAYER_WALK_SPEED
                };
            }
            CharacterAction::Jump => {
                if state.grounded {
                    physics_object.velocity = crate::constants::PLAYER_JUMP_VECTOR.into();
                }
            }
            CharacterAction::RightJump => {
                if state.grounded {
                    physics_object.velocity = crate::constants::PLAYER_RIGHT_JUMP_VECTOR.into();
                }
            }
            CharacterAction::LeftJump => {
                if state.grounded {
                    physics_object.velocity = crate::constants::PLAYER_LEFT_JUMP_VECTOR.into();
                }
            }
        }
    }
}
