use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Reflect)]
pub enum Model {
    Dummy,
    Mizku,
    Fireball,
    TrainingStage,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect)]
pub enum DummyAnimation {
    // Basics
    #[default]
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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Reflect)]
pub enum Animation {
    #[default]
    TPose,
    Dummy(DummyAnimation),
    Mizku(MizkuAnimation),
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
}
