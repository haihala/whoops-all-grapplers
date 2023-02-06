mod area;
pub use area::Area;

mod effects;
pub use effects::{SoundEffect, VisualEffect};

mod facing;
pub use facing::Facing;

mod gltf;
pub use gltf::*;

mod icons;
pub use icons::Icon;

mod inputs;
pub use inputs::{GameButton, StickPosition};

mod item_id;
pub use item_id::ItemId;

mod move_id;
pub use move_id::MoveId;

mod player;
pub use player::{Owner, Player, Players};

mod status;
pub use status::{Status, StatusCondition, StatusEffect};

mod time;
pub use time::*;

// This crate will be as small as possible so that types are where they are used
// It's meant for common universal types to circumvent circular dependencies.
pub const FPS: f32 = 60.0;
pub const INVENTORY_SIZE: usize = 7;
