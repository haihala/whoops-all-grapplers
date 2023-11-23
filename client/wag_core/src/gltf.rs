use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Reflect)]
pub enum Model {
    Dummy,
    Mizku,
    Fireball,
    Kunai,
    TrainingStage,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect)]
pub enum DummyAnimation {
    // Basics
    #[default]
    TPose,

    Idle,
    Crouch,
    CrouchStun,
    CrouchBlock,
    WalkForward,
    WalkBack,
    StandStun,
    StandBlock,
    AirIdle,
    AirStun,
    Getup,

    // Movement
    Jump,
    DashForward,
    DashBack,
    Dodge,

    // Strikes
    Slap,
    CrouchChop,
    BurnStraight,
    AntiAir,
    AirSlap,
    Divekick,
    AirSlam,
    GroundSlam,
    Sweep,

    // Throws
    NormalThrow,
    NormalThrowRecipient,
    AirThrow,
    AirThrowRecipient,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect)]
pub enum MizkuAnimation {
    Air,
    AirStagger,
    AirThrowHit,
    AirThrowStartup,
    AirThrowTarget,
    BackSway,
    Block,
    Crouch,
    CrouchBlock,
    CrouchStagger,
    DashBack,
    DashForward,
    FalconKnee,
    FootDiveHold,
    FootDiveRelease,
    Getup,
    GiParry,
    GroundThrowHit,
    GroundThrowStartup,
    GroundThrowTarget,
    HeelKick,
    HighSlice,
    HorizontalSlice,
    Idle,
    Jump,
    KneeThrust,
    KunaiThrow,
    LowKick,
    LowSlice,
    Sharpen,
    Stagger,
    StandPose,
    SwayDash,
    Sweep,
    #[default]
    TPose,
    Uppercut,
    UpwardsSlash,
    WalkBack,
    WalkForward,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Reflect)]
pub enum Animation {
    Dummy(DummyAnimation),
    Mizku(MizkuAnimation),
}

impl Default for Animation {
    fn default() -> Self {
        Animation::Dummy(DummyAnimation::TPose)
    }
}
impl From<DummyAnimation> for Animation {
    fn from(value: DummyAnimation) -> Self {
        Animation::Dummy(value)
    }
}
impl From<MizkuAnimation> for Animation {
    fn from(value: MizkuAnimation) -> Self {
        Animation::Mizku(value)
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

    // Not really used in the same way, but this is used to generate actions
    Jump,
}
