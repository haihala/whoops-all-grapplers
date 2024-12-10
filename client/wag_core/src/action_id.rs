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
    Left,
    Right,
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
    RomaineCancel,
    TrackSpikesDashForward,
    TrackSpikesDashBack,

    // Test moves
    TestMove,
    SecondTestMove,

    Samurai(SamuraiAction),
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
