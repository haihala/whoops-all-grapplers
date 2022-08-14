use bevy_inspector_egui::Inspectable;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Inspectable)]
pub enum Model {
    Dummy,
    Fireball,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Inspectable)]
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

    // Strikes
    Slap,
    CrouchChop,
    BurnStraight,
    AntiAir,
    AirSlap,
    Divekick,

    // Throws
    NormalThrow,
    NormalThrowRecipient,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Inspectable)]
pub enum Animation {
    #[default]
    TPose,
    Dummy(DummyAnimation),
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
