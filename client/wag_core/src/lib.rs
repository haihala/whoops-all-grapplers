mod area;
pub use area::Area;

mod facing;
pub use facing::Facing;

mod gltf;
pub use gltf::*;

mod effects;
pub use effects::{SoundEffect, VisualEffect};

mod inputs;
pub use inputs::{GameButton, StickPosition};

mod player;
pub use player::{Owner, Player, Players};

mod move_id;
pub use move_id::MoveId;

mod item_id;
pub use item_id::ItemId;

mod status;
pub use status::{Status, StatusCondition, StatusEffect};

mod time;
pub use time::*;

// This crate will be as small as possible so that types are where they are used
// It's meant for common universal types to circumvent circular dependencies.
pub const FPS: f32 = 60.0;
