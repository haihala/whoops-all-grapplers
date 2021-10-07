mod data;
mod frame_data;
pub use frame_data::FrameData;

pub type MoveType = u32;

/// Creates a module with a unique MoveType for each listed identifier
#[macro_export]
macro_rules! moves {
    ($module:ident, $offset:expr, ($move_name:ident, $($tail:ident),*)) => {    // Entry point
        pub mod $module {
            use super::*;
            moves!(0usize, $offset, $move_name, $($tail),*);  // Calls the next one
        }
    };

    ($idx:expr, $offset:expr, $move_name:ident, $($tail:ident),+ ) => {  // Recursively unpacks the moves
        pub const $move_name: MoveType = ($idx<<16+$offset) as MoveType;
        moves!($idx + 1usize, $offset, $($tail),*);
    };

    ($idx:expr, $offset:expr, $move_name:ident) => {  // Last of recursion
        pub const $move_name: MoveType = ($idx<<16+$offset) as MoveType;
    };
}

moves!(universal, 1, (DASH_FORWARD, DASH_BACK));
moves!(ryan, 2, (HADOUKEN, PUNCH, COMMAND_PUNCH));
