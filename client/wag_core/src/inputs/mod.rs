mod stick_position;
use bevy::prelude::*;
pub use stick_position::StickPosition;

use strum_macros::EnumIter;

use crate::Player;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, Reflect, Default)]
/// Buttons of the game
/// The name 'Button' is in prelude
/// This is for in match inputs
pub enum GameButton {
    #[default]
    Default, // To satisfy Inspectable

    Start,

    Fast,
    Strong,
    Wrestling,
    Gimmick,
}

// Game runs with strictly digital input, this is an abstraction
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WagInputButton {
    Up,
    Down,
    Left,
    Right,
    South,
    West,
    North,
    East,
    Start,
    Select,
    R1,
    R2,
    R3,
    L1,
    L2,
    L3,
}

impl WagInputButton {
    pub fn from_gamepad_button_type(value: GamepadButtonType) -> Option<Self> {
        Some(match value {
            GamepadButtonType::South => WagInputButton::South,
            GamepadButtonType::East => WagInputButton::East,
            GamepadButtonType::North => WagInputButton::North,
            GamepadButtonType::West => WagInputButton::West,
            GamepadButtonType::LeftTrigger => WagInputButton::L1,
            GamepadButtonType::LeftTrigger2 => WagInputButton::L2,
            GamepadButtonType::RightTrigger => WagInputButton::R1,
            GamepadButtonType::RightTrigger2 => WagInputButton::R2,
            GamepadButtonType::Select => WagInputButton::Select,
            GamepadButtonType::Start => WagInputButton::Start,
            GamepadButtonType::LeftThumb => WagInputButton::L3,
            GamepadButtonType::RightThumb => WagInputButton::R3,
            GamepadButtonType::DPadUp => WagInputButton::Up,
            GamepadButtonType::DPadDown => WagInputButton::Down,
            GamepadButtonType::DPadLeft => WagInputButton::Left,
            GamepadButtonType::DPadRight => WagInputButton::Right,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct WagInputEvent {
    pub button: WagInputButton,
    pub pressed: bool,
    pub player_handle: usize,
}

#[derive(Debug, Resource, Clone, Copy)]
pub struct Controllers {
    pub p1: usize,
    pub p2: usize,
}

impl Controllers {
    pub fn get_handle(&self, player: Player) -> usize {
        match player {
            Player::One => self.p1,
            Player::Two => self.p2,
        }
    }

    pub fn get_player(&self, handle: usize) -> Option<Player> {
        if handle == self.p1 {
            Some(Player::One)
        } else if handle == self.p2 {
            Some(Player::Two)
        } else {
            None
        }
    }
}

#[derive(Debug, Resource, Clone, Copy)]
pub struct LocalController(pub usize);
