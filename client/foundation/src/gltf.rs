use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Reflect)]
pub enum Model {
    Ronin,
    Fireball,
    Kunai,
    TrainingStage,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect)]
pub enum RoninAnimation {
    Air,
    AirStab,
    AirStagger,
    AirThrowHit,
    AirThrowStartup,
    AirThrowTarget,
    Block,
    Crouch,
    CrouchBlock,
    CrouchStagger,
    CrouchThrowHit,
    CrouchThrowStartup,
    CrouchThrowTarget,
    BackDash,
    AirForwardDash,
    GroundForwardDash,
    FalconKnee,
    FootDiveHold,
    FootDiveRelease,
    Getup,
    GiParry,
    HeelKick,
    HighStab,
    Idle,
    Jump,
    KneeThrust,
    KunaiThrow,
    LowKick,
    RC,
    SkyStab,
    Stagger,
    StandPose,
    StandThrowHit,
    StandThrowStartup,
    StandThrowTarget,
    SwordStance,
    StanceCancel,
    FastViperStrike,
    SlowViperStrike,
    FastRisingSun,
    SlowRisingSun,
    #[default]
    TPose,
    Uppercut,
    FastSharpen,
    SlowSharpen,
    FastSwordSlam,
    SlowSwordSlam,
    WalkBack,
    WalkForward,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Animation {
    Ronin(RoninAnimation),
}

impl From<RoninAnimation> for Animation {
    fn from(value: RoninAnimation) -> Self {
        Animation::Ronin(value)
    }
}

// For state to be able to return a generic animation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum AnimationType {
    AirIdle,
    AirStun,

    StandIdle,
    StandBlock,
    StandStun,
    WalkForward,
    WalkBack,

    CrouchIdle,
    CrouchBlock,
    CrouchStun,

    Getup,

    #[default]
    Default,
}
