use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    DivingHelmet,
    MoonBoots,
    ComicBook,
    RomaineLettuce,
    Wing,

    // Character specific
    // Ronin
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

    // TODO: Unused
    SafetyBoots,
}

impl ItemId {
    pub fn display_name(&self) -> String {
        match self {
            Self::BladeOil => "Blade oil",
            Self::Cigarettes => "Pack of cigs",
            Self::ComicBook => "Comic book",
            Self::DivingHelmet => "Diving helmet",
            Self::FeatheredBoots => "Feathered boots",
            Self::Gi => "Gi of the old masters",
            Self::GoalieGear => "Goalie gear",
            Self::HockeyPads => "Hockey pads",
            Self::IceCube => "Ice cube",
            Self::KunaiBelt => "Kunai belt",
            Self::KunaiPouch => "Kunai pouch",
            Self::MiniTasers => "Mini tasers",
            Self::MoonBoots => "Moon boots",
            Self::OliveOil => "Olive Oil",
            Self::PreWorkout => "Pre-workout",
            Self::RedPaint => "Can of red paint",
            Self::RomaineLettuce => "Romaine lettuce",
            Self::SafetyBoots => "Safety boots",
            Self::SmithyCoupon => "Smithy coupon",
            Self::SmokeBomb => "Smoke bomb",
            Self::SpareKunai => "Spare kunai",
            Self::ThumbTacks => "Thumbtacks",
            Self::TrackSpikes => "Track spikes",
            Self::Wing => "Pigeon wing",
            Self::Boots
            | Self::Crowbar
            | Self::Dumbbell
            | Self::Feather
            | Self::Fireaxe
            | Self::Protractor
            | Self::Stopwatch => return format!("{:?}", self),
        }
        .into()
    }
}
