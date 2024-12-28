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

    ThumbTacks,

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
            Self::DivingHelmet => "Diving helmet",
            Self::OliveOil => "Olive Oil",
            Self::Wing => "Pigeon wing",
            Self::FeatheredBoots => "Feathered boots",
            Self::TrackSpikes => "Track spikes",
            Self::MoonBoots => "Moon boots",
            Self::Cigarettes => "Pack of cigs",
            Self::PreWorkout => "Pre-workout",
            Self::RedPaint => "Can of red paint",
            Self::HockeyPads => "Hockey pads",
            Self::GoalieGear => "Goalie gear",
            Self::SafetyBoots => "Safety boots",
            Self::Gi => "Gi of the old masters",
            Self::SpareKunai => "Spare kunai",
            Self::KunaiPouch => "Kunai pouch",
            Self::KunaiBelt => "Kunai belt",
            Self::MiniTasers => "Mini tasers",
            Self::BladeOil => "Blade oil",
            Self::SmithyCoupon => "Smithy coupon",
            Self::IceCube => "Ice cube",
            Self::ComicBook => "Comic book",
            Self::RomaineLettuce => "Romaine lettuce",
            Self::ThumbTacks => "ThumbTacks",
            _ => return format!("{:?}", self),
        }
        .into()
    }
}
