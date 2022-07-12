use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveId {
    Default, // To satisfy Inspectable.

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

    // Dummy moves
    Grab,
    SonicBoom,
    BudgetBoom,
    HeavyHadouken,
    Hadouken,
    AirPunch,
    Low,
    CommandPunch,
    Punch,
}

impl Default for MoveId {
    fn default() -> Self {
        Self::Default
    }
}
