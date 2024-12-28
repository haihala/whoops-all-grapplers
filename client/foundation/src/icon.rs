use bevy::{prelude::*, utils::HashMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Resource)]
pub struct Icons(pub HashMap<Icon, Handle<Image>>);

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy, Asset, Reflect, EnumIter)]
pub enum Icon {
    #[default]
    Blank,

    // Items
    ThumbTack,
    Boots,
    Crowbar,
    Feather,
    OliveOil,
    RedPaint,
    Dumbbell,
    Stopwatch,
    HockeyPads,
    Cigarettes,
    PreWorkout,
    Gi,
    PigeonWing,
    FeatheredBoots,
    DivingHelmet,
    SafetyBoots,
    TrackSpikes,
    GoalieGear,
    Kunai,
    SpaceSuitBoots,
    KunaiPouch,
    KunaiBelt,
    BladeOil,
    SmithyCoupon,
    Fireaxe,
    SmokeBomb,
    Taser,
    Protractor,
    IceCube,
    ComicBook,
    Lettuce,

    // UI
    OkLink,
    GoodLink,
    PerfectLink,
    Star,
}
impl Icon {
    pub fn paths() -> HashMap<Icon, String> {
        Self::iter().map(|icon| (icon, icon.asset_path())).collect()
    }

    fn asset_path(&self) -> String {
        match self {
            Icon::Blank => "icons/blank.png",
            Icon::Boots => "icons/boots.png",
            Icon::Crowbar => "icons/crowbar.png",
            Icon::Feather => "icons/feather.png",
            Icon::OliveOil => "icons/olive-oil.png",
            Icon::RedPaint => "icons/red-paint.png",
            Icon::Dumbbell => "icons/dumbbell.png",
            Icon::Stopwatch => "icons/stopwatch.png",
            Icon::HockeyPads => "icons/hockeypads.png",
            Icon::Cigarettes => "icons/cigarettes.png",
            Icon::PreWorkout => "icons/pre-workout.png",
            Icon::Gi => "icons/gi.png",
            Icon::PigeonWing => "icons/pigeon-wing.png",
            Icon::FeatheredBoots => "icons/feathered-boots.png",
            Icon::DivingHelmet => "icons/diving-helmet.png",
            Icon::SafetyBoots => "icons/safety-boots.png",
            Icon::TrackSpikes => "icons/track-spikes.png",
            Icon::GoalieGear => "icons/goalie-gear.png",
            Icon::Kunai => "icons/kunai.png",
            Icon::KunaiPouch => "icons/kunai-pouch.png",
            Icon::KunaiBelt => "icons/kunai-bandolier.png",
            Icon::SpaceSuitBoots => "icons/space-boots.png",
            Icon::BladeOil => "icons/blade-oil.png",
            Icon::SmithyCoupon => "icons/coupon.png",
            Icon::Fireaxe => "icons/fireaxe.png",
            Icon::SmokeBomb => "icons/smoke-bomb.png",
            Icon::Taser => "icons/taser.png",
            Icon::Protractor => "icons/protractor.png",
            Icon::IceCube => "icons/ice-cube.png",
            Icon::ComicBook => "icons/comic-book.png",
            Icon::Lettuce => "icons/lettuce.png",
            Icon::OkLink => "icons/link-bonus-ok.png",
            Icon::GoodLink => "icons/link-bonus-good.png",
            Icon::PerfectLink => "icons/link-bonus-perfect.png",
            Icon::ThumbTack => "icons/thumbtack.png",
            Icon::Star => "icons/star.png",
        }
        .into()
    }
}
