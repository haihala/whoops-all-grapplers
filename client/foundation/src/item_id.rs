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
    OliveOil,

    ThumbTacks(usize),

    // Universal upgrades
    GoalieGear,
    TrackSpikes,
    FeatheredBoots,
    DivingHelmet, // TODO: Directional fast fall -> air dash
    MoonBoots,
    ComicBook,
    RomaineLettuce,
    Wing,

    // Character specific
    // Samurai
    SpareKunai,
    KunaiPouch,
    KunaiBelt,
    MiniTasers,
    Protractor,
    BladeOil,
    SmithyCoupon,
    Fireaxe,
    SmokeBomb,
    IceCube,

    #[default]
    Default,

    // TODO: Unused
    SafetyBoots,
}

impl ItemId {
    pub fn display_name(&self) -> String {
        match self {
            Self::DivingHelmet => "Diving helmet".into(),
            Self::OliveOil => "Olive Oil".into(),
            Self::Wing => "Pigeon wing".into(),
            Self::FeatheredBoots => "Feathered boots".into(),
            Self::TrackSpikes => "Track spikes".into(),
            Self::MoonBoots => "Moon boots".into(),
            Self::Cigarettes => "Pack of cigs".into(),
            Self::PreWorkout => "Pre-workout".into(),
            Self::RedPaint => "Can of red paint".into(),
            Self::HockeyPads => "Hockey pads".into(),
            Self::GoalieGear => "Goalie gear".into(),
            Self::SafetyBoots => "Safety boots".into(),
            Self::Gi => "Gi of the old masters".into(),
            Self::SpareKunai => "Spare kunai".into(),
            Self::KunaiPouch => "Kunai pouch".into(),
            Self::KunaiBelt => "Kunai belt".into(),
            Self::MiniTasers => "Mini tasers".into(),
            Self::BladeOil => "Blade oil".into(),
            Self::SmithyCoupon => "Smithy coupon".into(),
            Self::IceCube => "Ice cube".into(),
            Self::ComicBook => "Comic book".into(),
            Self::RomaineLettuce => "Romaine lettuce".into(),
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
