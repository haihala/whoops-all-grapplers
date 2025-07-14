use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecialVersion {
    Metered,
    Strong,
    Fast,
}

// NOTE: Order matters, later actions take priority.
#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Component)]
pub enum ActionId {
    #[default]
    Default, // TODO: Get rid of this

    // Universal
    ForwardDash,
    BackDash,
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
    MeteredForwardDash,
    MeteredBackDash,
    AirForwardDash,
    AirBackDash,
    MeteredAirForwardDash,
    MeteredAirBackDash,

    // Test moves
    TestMove,
    SecondTestMove,

    Ronin(RoninAction),
    CPO(CPOAction),
}

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoninAction {
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

impl From<RoninAction> for ActionId {
    fn from(value: RoninAction) -> Self {
        Self::Ronin(value)
    }
}

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CPOAction {
    Jackpot,

    // Throws
    AirThrowStartup,
    AirThrowHit,
    AirThrowRecipient,
    ForwardThrowStartup,
    BackThrowStartup,
    GroundThrowHit,
    ForwardThrowRecipient,
    BackThrowRecipient,

    // Normals
    BodySplash,
    Stomp3,
    Stomp2,
    Stomp1,
    HookPunch,
    JumpingKnees,
    DickJab,
    Chop,

    // Specials
    AirTimewinder(SpecialVersion),
    GroundTimeWinderStraight(SpecialVersion),
    GroundTimeWinderLow(SpecialVersion),
    PayCheckStartup,
    PayCheckHit,
    PayCheckRecipient,
    AdBreak,
    Sugarcoat,
}

impl From<CPOAction> for ActionId {
    fn from(value: CPOAction) -> Self {
        Self::CPO(value)
    }
}
