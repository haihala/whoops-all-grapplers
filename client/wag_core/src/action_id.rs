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

    GrisingSun,
    ArisingSun,
    KunaiThrow,

    // Throws
    AirThrow,
    AirThrowHit,
    AirThrowTarget,
    Sweep,
    ForwardThrow,
    BackThrow,
    GroundThrowHit,
    GroundThrowTarget,

    // Sway stuff
    ShortBackSway,
    LongBackSway,
    ShortSwayDash,
    LongSwayDash,
    SwayOverhead,
    SwayLow,
    Pilebunker,
    SwayCancel,

    // Normals
    LowKick,
    FalconKnee,
    KneeThrust,
    Overhead,
    Uppercut,
    FootDive,
    HeelKick,
}
