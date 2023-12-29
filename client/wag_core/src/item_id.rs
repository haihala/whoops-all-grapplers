use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ItemId {
    // Universal consumables
    PreWorkout,

    // Universal basic
    Gi,
    Boots,
    HockeyPads,
    RedPaint,
    Stopwatch,
    Crowbar,
    Dumbbell,
    Feather,
    Cigarettes,

    ThumbTacks(usize),

    // Universal upgrades
    SafetyBoots,
    GoaleeGear,
    TrackSpikes,
    FeatheredBoots,
    PidgeonWing,
    Flyweight, // TODO: Directional fast fall -> air dash

    // Character specific
    // Dummy
    Roids,

    // Mizku
    Kunai,
    SteelHeelBoots,
    SpaceSuitBoots,

    #[default]
    Default,
}

impl ItemId {
    pub fn display_name(&self) -> String {
        match self {
            Self::PidgeonWing => "Pidgeon wing".into(),
            Self::FeatheredBoots => "Feathered boots".into(),
            Self::SteelHeelBoots => "Steel heel boots".into(),
            Self::TrackSpikes => "Track spikes".into(),
            Self::SpaceSuitBoots => "Space suit boots".into(),
            Self::Cigarettes => "Pack of cigs".into(),
            Self::PreWorkout => "Pre-workout".into(),
            Self::RedPaint => "Can of red paint".into(),
            Self::HockeyPads => "Hockey pads".into(),
            Self::GoaleeGear => "Goalee gear".into(),
            Self::SafetyBoots => "Safety boots".into(),
            Self::Gi => "Gi of the old masters".into(),
            Self::ThumbTacks(n) => match n {
                1 => "A single thumbtack".into(),
                5 => "Fistful of thumbtacks".into(),
                6 => "Carton of thumbtacks".into(),
                7 => "Stack of thumbtacks".into(),
                8 => "Mountain of thumbtacks".into(),
                9 => "Lifetime supply of thumbtacks".into(),
                other => format!("{} thumbtacks", usize::pow(2, *other as u32 - 1)),
            },
            _ => format!("{:?}", self),
        }
    }
}
