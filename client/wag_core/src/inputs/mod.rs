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

impl GameButton {
    pub fn to_dsl(self) -> String {
        match self {
            GameButton::Default => panic!("Default can't be converted to dsl"),
            GameButton::Start => ".",
            GameButton::Fast => "f",
            GameButton::Strong => "s",
            GameButton::Wrestling => "w",
            GameButton::Gimmick => "g",
        }
        .into()
    }
}

// Game runs with strictly digital input, this is an abstraction
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, EnumIter)]
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

    pub fn from_key(value: KeyCode) -> Option<Self> {
        Some(match value {
            KeyCode::KeyJ => WagInputButton::South,
            KeyCode::KeyK => WagInputButton::East,
            KeyCode::KeyI => WagInputButton::North,
            KeyCode::KeyU => WagInputButton::West,
            KeyCode::KeyY => WagInputButton::L1,
            KeyCode::KeyH => WagInputButton::L2,
            KeyCode::KeyO => WagInputButton::R1,
            KeyCode::KeyL => WagInputButton::R2,
            KeyCode::KeyV => WagInputButton::Select,
            KeyCode::KeyB => WagInputButton::Start,
            KeyCode::KeyN => WagInputButton::L3,
            KeyCode::KeyM => WagInputButton::R3,
            KeyCode::KeyW => WagInputButton::Up,
            KeyCode::KeyS => WagInputButton::Down,
            KeyCode::KeyA => WagInputButton::Left,
            KeyCode::KeyD => WagInputButton::Right,
            _ => return None,
        })
    }

    pub fn to_gamepad_button_type(&self) -> GamepadButtonType {
        match self {
            WagInputButton::South => GamepadButtonType::South,
            WagInputButton::East => GamepadButtonType::East,
            WagInputButton::North => GamepadButtonType::North,
            WagInputButton::West => GamepadButtonType::West,
            WagInputButton::L1 => GamepadButtonType::LeftTrigger,
            WagInputButton::L2 => GamepadButtonType::LeftTrigger2,
            WagInputButton::R1 => GamepadButtonType::RightTrigger,
            WagInputButton::R2 => GamepadButtonType::RightTrigger2,
            WagInputButton::Select => GamepadButtonType::Select,
            WagInputButton::Start => GamepadButtonType::Start,
            WagInputButton::L3 => GamepadButtonType::LeftThumb,
            WagInputButton::R3 => GamepadButtonType::RightThumb,
            WagInputButton::Up => GamepadButtonType::DPadUp,
            WagInputButton::Down => GamepadButtonType::DPadDown,
            WagInputButton::Left => GamepadButtonType::DPadLeft,
            WagInputButton::Right => GamepadButtonType::DPadRight,
        }
    }

    pub fn to_keycode(&self) -> KeyCode {
        match self {
            WagInputButton::South => KeyCode::KeyJ,
            WagInputButton::East => KeyCode::KeyK,
            WagInputButton::North => KeyCode::KeyI,
            WagInputButton::West => KeyCode::KeyU,
            WagInputButton::L1 => KeyCode::KeyY,
            WagInputButton::L2 => KeyCode::KeyH,
            WagInputButton::R1 => KeyCode::KeyO,
            WagInputButton::R2 => KeyCode::KeyL,
            WagInputButton::Select => KeyCode::KeyV,
            WagInputButton::Start => KeyCode::KeyB,
            WagInputButton::L3 => KeyCode::KeyN,
            WagInputButton::R3 => KeyCode::KeyM,
            WagInputButton::Up => KeyCode::KeyW,
            WagInputButton::Down => KeyCode::KeyS,
            WagInputButton::Left => KeyCode::KeyA,
            WagInputButton::Right => KeyCode::KeyD,
        }
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

impl Default for Controllers {
    fn default() -> Self {
        Controllers { p1: 0, p2: 1 }
    }
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
