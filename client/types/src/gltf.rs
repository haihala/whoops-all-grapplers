use bevy_inspector_egui::Inspectable;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Inspectable)]
pub enum Model {
    Dummy,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Inspectable)]
pub enum DummyAnimation {
    // Basics
    #[default]
    Idle,
    Crouch,
    WalkForward,
    WalkBack,
    StandStun,
    CrouchStun,
    AirIdle,
    AirStun,

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
    StandIdle,
    CrouchIdle,
    AirIdle,
    StandStun,
    CrouchStun,
    AirStun,
    WalkForward,
    WalkBack,
}
