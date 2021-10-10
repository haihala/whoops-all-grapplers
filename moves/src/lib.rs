mod inputs;
pub use inputs::{
    ryan_normals, ryan_specials, MotionDefinition, SpecialDefinition, StickTransition,
};
mod hitbox;
pub use hitbox::{ryan_hitboxes, Hitbox};
mod frame_data;
pub use frame_data::{ryan_frames, FrameData};
use types::MoveType;

/// Creates a module with a unique MoveType for each listed identifier
#[macro_export]
macro_rules! moves {
    ($module:ident, $offset:expr, ($move_name:ident, $($tail:ident),+)) => {    // Entry point
        pub mod $module {
            use super::*;
            moves!(0usize, $offset, ($move_name, $($tail),*));  // Calls the next one
        }
    };

    ($idx:expr, $offset:expr, ($move_name:ident, $($tail:ident),+)) => {  // Recursively unpacks the moves
        pub const $move_name: MoveType = ($idx+($offset*1000)) as MoveType;
        moves!($idx + 1usize, $offset, ($($tail),*));
    };

    ($idx:expr, $offset:expr, ($move_name:ident)) => {  // Last of recursion
        pub const $move_name: MoveType = ($idx+($offset*1000)) as MoveType;
    };
}

// Order matters, moves defined first have priority over later ones
moves!(universal, 1usize, (DASH_FORWARD, DASH_BACK));
moves!(ryan, 2usize, (HADOUKEN, COMMAND_PUNCH, PUNCH));
