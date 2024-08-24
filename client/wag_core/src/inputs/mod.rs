mod stick_position;
use bevy::prelude::*;
pub use stick_position::StickPosition;

use strum_macros::EnumIter;

use crate::Player;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, Reflect, Default)]
/// Buttons of the game
/// The name 'Button' is in prelude
pub enum GameButton {
    #[default]
    Default, // To satisfy Inspectable

    Start,

    Fast,
    Strong,
    Wrestling,
    Gimmick,
}

#[derive(Debug, Resource)]
pub struct Controllers {
    pub p1: Gamepad,
    pub p2: Gamepad,
}

impl Controllers {
    pub fn get_pad(&self, player: Player) -> Gamepad {
        match player {
            Player::One => self.p1,
            Player::Two => self.p2,
        }
    }

    pub fn get_player(&self, pad: Gamepad) -> Option<Player> {
        if pad == self.p1 {
            Some(Player::One)
        } else if pad == self.p2 {
            Some(Player::Two)
        } else {
            None
        }
    }
}
