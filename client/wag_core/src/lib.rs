mod action_id;
pub use action_id::{ActionId, SamuraiAction, SpecialVersion};

mod area;
pub use area::Area;

mod args;
pub use args::{Dev, WagArgs};

mod cancels;
pub use cancels::*;

mod character_id;
pub use character_id::{CharacterId, Characters, LocalCharacter};

mod color_palette;
pub use color_palette::*;

mod combo;
pub use combo::Combo;

mod economy;
pub use economy::*;

mod facing;
pub use facing::Facing;

mod gltf;
pub use gltf::*;

mod icon;
pub use icon::{Icon, Icons};

mod inputs;
pub use inputs::{
    Controllers, GameButton, InputEvent, InputState, InputStream, LocalController,
    NetworkInputButton, OwnedInput, StickPosition, KEYBOARD_PAD_ID, STICK_DEAD_ZONE,
};

mod item_id;
pub use item_id::ItemId;

mod player;
pub use player::{Owner, Player, Players};

mod sound;
pub use sound::{SoundEffect, VoiceLine, BIG_HIT_THRESHOLD, SMALL_HIT_THRESHOLD};

mod status;
pub use status::{Stats, StatusCondition, StatusFlag};

mod time;
pub use time::*;

mod visual_effects;
pub use visual_effects::{VfxRequest, VisualEffect};

// This crate will be as small as possible so that types are where they are used
// It's meant for common universal types to circumvent circular dependencies.
pub const FPS: f32 = 60.0;

// How many frames can you kara cancel to metered versions of moves
pub const METERED_KARA_WINDOW: usize = 3;
