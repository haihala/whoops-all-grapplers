use bevy_inspector_egui::Inspectable;

mod ryan;
pub use ryan::*;
mod move_bank;
pub use move_bank::*;

/// Creates a unique MoveId for each listed identifier provided the offset is unique
#[macro_export]
macro_rules! moves {
    ($offset:expr, ($move_name:ident, $($tail:ident),+)) => {    // Entry point
        use types::MoveId;

        moves!(0usize, $offset, ($move_name, $($tail),*));  // Calls the next one
    };

    ($idx:expr, $offset:expr, ($move_name:ident, $($tail:ident),+)) => {
        pub const $move_name: MoveId = ($idx+($offset*1000)) as MoveId;
        moves!($idx + 1usize, $offset, ($($tail),*));   // Recursively calls itself
    };

    ($idx:expr, $offset:expr, ($move_name:ident)) => {  // Last of recursion
        pub const $move_name: MoveId = ($idx+($offset*1000)) as MoveId;
    };
}

// Order matters, moves defined first have priority over later ones
pub mod test {
    moves!(99usize, (TEST_MOVE, SECOND_TEST_MOVE));
}

pub mod universal {
    moves!(
        0usize,
        (
            DASH_FORWARD,
            DASH_BACK,
            NEUTRAL_SUPER_JUMP,
            FORWARD_SUPER_JUMP,
            BACK_SUPER_JUMP,
            NEUTRAL_JUMP,
            FORWARD_JUMP,
            BACK_JUMP
        )
    );
}

pub mod equipment {
    moves!(1usize, (HANDMEDOWNKEN, GUNSHOT, SHOOT));
}

// Defined smallest to largest aka later ones can cancel earlier ones.
#[derive(PartialEq, PartialOrd, Debug, Inspectable, Clone, Copy, Eq)]
pub enum CancelLevel {
    Anything,
    LightNormal,
    Dash,
    Jump,
    HeavyNormal,
    LightSpecial,
    HeavySpecial,
    Grab,
    Uncancellable,
}
impl Default for CancelLevel {
    fn default() -> Self {
        CancelLevel::Anything
    }
}
