use bevy::{math::u16, platform::collections::HashMap, prelude::*};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::Player;

mod stick_position;
pub use stick_position::StickPosition;

mod input_state;
pub use input_state::InputState;

// How many frames can you kara cancel to metered versions of moves
pub const KARA_WINDOW: usize = 3;
pub const STICK_DEAD_ZONE: f32 = 0.3;

pub const KEYBOARD_MAGIC_CONSTANT: usize = 69;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, Reflect, Default)]
/// Buttons of the game
/// The name 'Button' is in prelude
/// This is for in match inputs
pub enum GameButton {
    #[default]
    Default, // To satisfy Inspectable

    Start,
    Select,

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
            GameButton::Select => ",",
            GameButton::Fast => "f",
            GameButton::Strong => "s",
            GameButton::Wrestling => "w",
            GameButton::Gimmick => "g",
        }
        .into()
    }
}

impl TryFrom<NetworkInputButton> for GameButton {
    type Error = (); // This could be improved

    fn try_from(value: NetworkInputButton) -> Result<Self, ()> {
        Ok(match value {
            // This is where keybindings are sort of defined
            NetworkInputButton::South => GameButton::Fast,
            NetworkInputButton::West => GameButton::Gimmick,
            NetworkInputButton::North => GameButton::Wrestling,
            NetworkInputButton::East => GameButton::Strong,
            NetworkInputButton::Start => GameButton::Start,
            NetworkInputButton::Select => GameButton::Select,
            _ => return Err(()),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum MenuInput {
    Up,
    Down,
    Left,
    Right,
    Accept,
    Cancel,
    Secondary,
}

impl TryFrom<NetworkInputButton> for MenuInput {
    type Error = ();

    fn try_from(value: NetworkInputButton) -> Result<Self, ()> {
        Ok(match value {
            // This is where keybindings are sort of defined
            NetworkInputButton::South => MenuInput::Accept,
            NetworkInputButton::West => MenuInput::Secondary,
            NetworkInputButton::East => MenuInput::Cancel,

            NetworkInputButton::Up => MenuInput::Up,
            NetworkInputButton::Down => MenuInput::Down,
            NetworkInputButton::Left => MenuInput::Left,
            NetworkInputButton::Right => MenuInput::Right,
            _ => return Err(()),
        })
    }
}

// Game runs with strictly digital input, this is an abstraction
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, EnumIter)]
pub enum NetworkInputButton {
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

impl NetworkInputButton {
    pub fn to_gamepad_button_type(&self) -> GamepadButton {
        match self {
            NetworkInputButton::South => GamepadButton::South,
            NetworkInputButton::East => GamepadButton::East,
            NetworkInputButton::North => GamepadButton::North,
            NetworkInputButton::West => GamepadButton::West,
            NetworkInputButton::L1 => GamepadButton::LeftTrigger,
            NetworkInputButton::L2 => GamepadButton::LeftTrigger2,
            NetworkInputButton::R1 => GamepadButton::RightTrigger,
            NetworkInputButton::R2 => GamepadButton::RightTrigger2,
            NetworkInputButton::Select => GamepadButton::Select,
            NetworkInputButton::Start => GamepadButton::Start,
            NetworkInputButton::L3 => GamepadButton::LeftThumb,
            NetworkInputButton::R3 => GamepadButton::RightThumb,
            NetworkInputButton::Up => GamepadButton::DPadUp,
            NetworkInputButton::Down => GamepadButton::DPadDown,
            NetworkInputButton::Left => GamepadButton::DPadLeft,
            NetworkInputButton::Right => GamepadButton::DPadRight,
        }
    }

    pub fn to_keycode(&self) -> KeyCode {
        match self {
            NetworkInputButton::South => KeyCode::KeyJ,
            NetworkInputButton::East => KeyCode::KeyK,
            NetworkInputButton::North => KeyCode::KeyI,
            NetworkInputButton::West => KeyCode::KeyU,
            NetworkInputButton::L1 => KeyCode::KeyY,
            NetworkInputButton::L2 => KeyCode::KeyH,
            NetworkInputButton::R1 => KeyCode::KeyO,
            NetworkInputButton::R2 => KeyCode::KeyL,
            NetworkInputButton::Select => KeyCode::KeyV,
            NetworkInputButton::Start => KeyCode::KeyB,
            NetworkInputButton::L3 => KeyCode::KeyN,
            NetworkInputButton::R3 => KeyCode::KeyM,
            NetworkInputButton::Up => KeyCode::KeyW,
            NetworkInputButton::Down => KeyCode::KeyS,
            NetworkInputButton::Left => KeyCode::KeyA,
            NetworkInputButton::Right => KeyCode::KeyD,
        }
    }

    pub fn serialize(active: impl Fn(NetworkInputButton) -> bool) -> u16 {
        let mut out = 0;
        for (shift, nw_btn) in NetworkInputButton::iter().enumerate() {
            if active(nw_btn) {
                out |= 1 << shift;
            }
        }
        out
    }

    pub fn from_stick(stick: IVec2) -> Vec<NetworkInputButton> {
        let mut out = vec![];
        if stick.x == 1 {
            out.push(NetworkInputButton::Right);
        } else if stick.x == -1 {
            out.push(NetworkInputButton::Left);
        }

        if stick.y == 1 {
            out.push(NetworkInputButton::Up);
        } else if stick.y == -1 {
            out.push(NetworkInputButton::Down);
        }

        out
    }

    pub fn deserialize(input: u16) -> Vec<NetworkInputButton> {
        let mut out = vec![];
        for (shift, nw_btn) in NetworkInputButton::iter().enumerate() {
            if input & 1 << shift != 0 {
                out.push(nw_btn);
            }
        }
        out
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Copy, Hash)]
pub enum InputEvent {
    Point(StickPosition),
    Press(GameButton),
    Release(GameButton),
}

impl From<char> for InputEvent {
    fn from(ch: char) -> InputEvent {
        if let Ok(number_token) = ch.to_string().parse::<i32>() {
            InputEvent::Point(number_token.into())
        } else {
            match ch {
                'f' => InputEvent::Press(GameButton::Fast),
                'F' => InputEvent::Release(GameButton::Fast),
                's' => InputEvent::Press(GameButton::Strong),
                'S' => InputEvent::Release(GameButton::Strong),
                'w' => InputEvent::Press(GameButton::Wrestling),
                'W' => InputEvent::Release(GameButton::Wrestling),
                'g' => InputEvent::Press(GameButton::Gimmick),
                'G' => InputEvent::Release(GameButton::Gimmick),
                // There is no need for negative edge on start, this whole thing is mighty sus so let's not get caught up on that shall we
                '.' => InputEvent::Press(GameButton::Start),
                ',' => InputEvent::Press(GameButton::Select),
                _ => panic!("Invalid character {ch}"),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnedInput {
    pub event: InputEvent,
    pub player_handle: InputDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnedMenuInput {
    pub event: MenuInput,
    pub player_handle: InputDevice,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Resource)]
pub struct InputStream {
    pub events: Vec<OwnedInput>,
    pub menu_events: Vec<OwnedMenuInput>,
    pub input_states: HashMap<InputDevice, u16>,
}

fn stream_folder(mut stick: IVec2, new: &NetworkInputButton) -> IVec2 {
    match new {
        NetworkInputButton::Up => {
            stick.y += 1;
        }
        NetworkInputButton::Down => {
            stick.y -= 1;
        }
        NetworkInputButton::Right => {
            stick.x += 1;
        }
        NetworkInputButton::Left => {
            stick.x -= 1;
        }
        _ => {}
    };
    stick
}

impl InputStream {
    pub fn update_pad(&mut self, input_device: InputDevice, state: u16) {
        let old_state =
            NetworkInputButton::deserialize(*self.input_states.entry(input_device).or_default());
        let new_state = NetworkInputButton::deserialize(state);
        self.input_states.insert(input_device, state);

        let old_stick = old_state.iter().fold(IVec2::default(), stream_folder);
        let new_stick = new_state.iter().fold(IVec2::default(), stream_folder);

        if new_stick != old_stick {
            self.events.push(OwnedInput {
                event: InputEvent::Point(new_stick.into()),
                player_handle: input_device,
            })
        }

        for nw_btn in NetworkInputButton::iter() {
            let old = old_state.contains(&nw_btn);
            let new = new_state.contains(&nw_btn);

            if new && !old {
                // Press
                if let Ok(game_btn) = GameButton::try_from(nw_btn) {
                    self.events.push(OwnedInput {
                        event: InputEvent::Press(game_btn),
                        player_handle: input_device,
                    })
                }

                if let Ok(event) = MenuInput::try_from(nw_btn) {
                    self.menu_events.push(OwnedMenuInput {
                        event,
                        player_handle: input_device,
                    })
                }
            }

            if old && !new {
                // Release
                if let Ok(game_btn) = GameButton::try_from(nw_btn) {
                    self.events.push(OwnedInput {
                        event: InputEvent::Release(game_btn),
                        player_handle: input_device,
                    })
                }
            }
        }
    }
}

#[derive(Debug, Resource, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDevice {
    Controller(Entity),
    Keyboard,
    Online(usize),
}

#[derive(Debug, Resource, Clone, Copy)]
pub struct Controllers {
    pub p1: InputDevice,
    pub p2: InputDevice,
}

impl Controllers {
    pub fn get_handle(&self, player: Player) -> InputDevice {
        match player {
            Player::One => self.p1,
            Player::Two => self.p2,
        }
    }

    pub fn get_player(&self, handle: InputDevice) -> Option<Player> {
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
pub struct LocalController(pub InputDevice);
