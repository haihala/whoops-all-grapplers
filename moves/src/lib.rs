use bevy_inspector_egui::Inspectable;

mod ryan;
pub use ryan::*;
mod equipment;
pub use equipment::*;

mod move_bank;
pub use move_bank::*;
mod move_parts;
pub use move_parts::*;

#[derive(Inspectable, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveId {
    Default, // Some default value required by the default trait.

    // Universal
    DashForward,
    DashBack,
    NeutralSuperJump,
    ForwardSuperJump,
    BackSuperJump,
    NeutralJump,
    ForwardJump,
    BackJump,

    // Equipment
    HandMeDownKen,
    Gunshot,
    Shoot,

    // Test moves
    TestMove,
    SecondTestMove,

    // Ryan moves
    Grab,
    SonicBoom,
    BudgetBoom,
    HeavyHadouken,
    Hadouken,
    AirPunch,
    CommandPunch,
    Punch,
}

impl Default for MoveId {
    fn default() -> Self {
        Self::Default
    }
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
