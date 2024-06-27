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
    NeutralJump,
    ForwardJump,
    BackJump,

    // Equipment
    NeutralAirJump,
    ForwardAirJump,
    BackAirJump,
    NeutralSuperJump,
    ForwardSuperJump,
    BackSuperJump,
    FastFall,
    HighGiParry,
    ParryFlash,
    TrackSpikesDashForward,
    TrackSpikesDashBack,

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
    FSwordStance,
    SSwordStance,
    ViperStrike,
    RisingSun,
    Sharpen,

    KunaiThrow,

    // Throws
    AirThrow,
    AirThrowHit,
    AirThrowTarget,
    ForwardThrow,
    BackThrow,
    StandThrowHit,
    StandThrowTarget,
    CrouchThrow,
    CrouchThrowHit,
    CrouchThrowTarget,

    // Normals
    SkySlash,
    HighStab,
    AirSlice,
    Overhead,
    Uppercut,
    FootDive,
    HeelKick,
    LowKick,
    FalconKnee,
    KneeThrust,
}
