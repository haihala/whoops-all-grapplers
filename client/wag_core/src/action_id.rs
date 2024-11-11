use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecialVersion {
    Metered,
    Strong,
    Fast,
}

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Component)]
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
    GiParry,
    TrackSpikesDashForward,
    TrackSpikesDashBack,

    // Test moves
    TestMove,
    SecondTestMove,

    Dummy(DummyActionId),
    Samurai(SamuraiAction),
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
pub enum SamuraiAction {
    SwordStance(SpecialVersion),
    StanceCancel(SpecialVersion),
    StanceForwardDash(SpecialVersion),
    StanceBackDash(SpecialVersion),
    ViperStrike(SpecialVersion),
    RisingSun(SpecialVersion),
    Sharpen(SpecialVersion),
    SwordSlam(SpecialVersion),
    Stanceport(SpecialVersion),

    KunaiThrow(SpecialVersion),

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
    FootDiveHold,
    FootDiveRelease,
    HeelKick,
    LowKick,
    FalconKnee,
    KneeThrust,
}

impl From<SamuraiAction> for ActionId {
    fn from(value: SamuraiAction) -> Self {
        Self::Samurai(value)
    }
}
