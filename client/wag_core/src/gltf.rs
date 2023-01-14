use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Reflect, FromReflect)]
pub enum Model {
    Dummy,
    Fireball,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect, FromReflect,
)]
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

#[derive(
    Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Reflect, FromReflect,
)]
pub enum Animation {
    #[default]
    TPose,
    Dummy(DummyAnimation),
}
impl From<DummyAnimation> for Animation {
    fn from(value: DummyAnimation) -> Self {
        Animation::Dummy(value)
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
