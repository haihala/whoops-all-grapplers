use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ActionId {
    #[default]
    Default, // To satisfy Inspectable.
    Up,
    Down,
    Back,
    Forward,
    Primary,
    Secondary,
    Cancel,
    Start,

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
    HighGiParry,
    ParryFlash,

    // Test moves
    TestMove,
    SecondTestMove,

    Dummy(DummyActionId),
    Mizku(MizkuActionId),
}

// Earlier = Higher priority = Will happen if both inputs present
// This will get better once specificity based priorisation happens
#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DummyActionId {
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

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MizkuActionId {
    Sharpen,

    UpwardsSlash,
    KunaiThrow,

    // Throws
    AirThrow,
    AirThrowHit,
    Sweep,
    ForwardThrow,
    BackThrow,
    GroundThrowHit,

    // Sway stuff
    ShortBackSway,
    LongBackSway,
    SwayDash,
    ShortHighSlice,
    LongHighSlice,
    ShortLowSlice,
    LongLowSlice,
    LongHorizontalSlice,
    ShortHorizontalSlice,

    // Normals
    LowKick,
    FalconKnee,
    KneeThrust,
    Overhead,
    Uppercut,
    FootDiveRelease,
    FootDiveStart,
    HeelKick,
}
