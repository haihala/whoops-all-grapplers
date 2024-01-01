mod action_id;
pub use action_id::{ActionId, DummyActionId, MizkuActionId};

mod area;
pub use area::Area;

mod args;
pub use args::WagArgs;

mod character_id;
pub use character_id::CharacterId;

mod color_palette;
pub use color_palette::*;

mod economy;
pub use economy::*;

mod effects;
pub use effects::{SoundEffect, VisualEffect};

mod facing;
pub use facing::Facing;

mod gltf;
pub use gltf::*;

mod icon;
pub use icon::Icon;

mod inputs;
pub use inputs::{GameButton, StickPosition};

mod item_id;
pub use item_id::ItemId;

mod joints;
pub use joints::{Joint, JointCollider, Joints};

mod player;
pub use player::{Owner, Player, Players};

mod status;
pub use status::{Stats, StatusCondition, StatusFlag};

mod time;
pub use time::*;

// This crate will be as small as possible so that types are where they are used
// It's meant for common universal types to circumvent circular dependencies.
pub const FPS: f32 = 60.0;
pub const INVENTORY_SIZE: usize = 7;
