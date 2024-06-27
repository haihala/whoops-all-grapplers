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
    AirStab,
    AirStagger,
    AirThrowHit,
    AirThrowStartup,
    AirThrowTarget,
    Sway,
    Block,
    Crouch,
    CrouchBlock,
    CrouchStagger,
    CrouchThrowHit,
    CrouchThrowStartup,
    CrouchThrowTarget,
    DashBack,
    DashForward,
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
    LowStab,
    Overhead,
    Stagger,
    StandPose,
    StandThrowHit,
    StandThrowStartup,
    StandThrowTarget,
    #[default]
    TPose,
    Uppercut,
    SwordStance,
    GrisingSun,
    ViperStrike,
    Sharpen,
    WalkBack,
    WalkForward,

    // Legacy, unused, here to ensure animation loading works as they are still in the gltf
    Sweep,
    ArisingSun,
    SwayCancel,
    SwayDash,
    SwayLow,
    SwayOverhead,
    Pilebunker,
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
}
