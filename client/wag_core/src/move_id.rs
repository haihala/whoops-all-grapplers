use bevy::prelude::*;

#[derive(
    Reflect, FromReflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub enum MoveId {
    #[default]
    Default, // To satisfy Inspectable.
    Up,
    Down,
    Left,
    Right,
    Primary,
    Secondary,
    Back,

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
    SonicBoom,
    BudgetBoom,
    HeavyHadouken,
    Hadouken,
    GroundSlam,
    AirSlam,
    BackThrow,
    ForwardThrow,
    AirThrow,
    Divekick,
    AirSlap,
    AntiAir,
    LowChop,
    BurnStraight,
    Slap,
    Sweep,
    Dodge,
}
