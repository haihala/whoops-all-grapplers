use bevy::utils::HashMap;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Icon {
    #[default]
    Blank,

    ThumbTacks(usize),
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
    BladeOil,
    SmithyCoupon,
}
impl Icon {
    pub fn paths() -> HashMap<Icon, String> {
        vec![
            (Icon::Blank, "icons/blank.png".into()),
            (Icon::Boots, "icons/boots.png".into()),
            (Icon::Crowbar, "icons/crowbar.png".into()),
            (Icon::Feather, "icons/feather.png".into()),
            (Icon::OliveOil, "icons/olive-oil.png".into()),
            (Icon::RedPaint, "icons/red-paint.png".into()),
            (Icon::Dumbbell, "icons/dumbbell.png".into()),
            (Icon::Stopwatch, "icons/stopwatch.png".into()),
            (Icon::HockeyPads, "icons/hockeypads.png".into()),
            (Icon::Cigarettes, "icons/cigarettes.png".into()),
            (Icon::PreWorkout, "icons/pre-workout.png".into()),
            (Icon::Gi, "icons/gi.png".into()),
            (Icon::PigeonWing, "icons/pigeon-wing.png".into()),
            (Icon::FeatheredBoots, "icons/feathered-boots.png".into()),
            (Icon::DivingHelmet, "icons/diving-helmet.png".into()),
            (Icon::SafetyBoots, "icons/safety-boots.png".into()),
            (Icon::TrackSpikes, "icons/track-spikes.png".into()),
            (Icon::GoalieGear, "icons/goalie-gear.png".into()),
            (Icon::Kunai, "icons/kunai.png".into()),
            (Icon::KunaiPouch, "icons/kunai-pouch.png".into()),
            (Icon::SpaceSuitBoots, "icons/space-boots.png".into()),
            (Icon::BladeOil, "icons/blade-oil.png".into()),
            (Icon::SmithyCoupon, "icons/coupon.png".into()),
        ]
        .into_iter()
        .chain((1..9).map(|id| {
            (
                Icon::ThumbTacks(id),
                format!("icons/thumbtack{}.png", usize::pow(2, (id - 1) as u32)),
            )
        }))
        .collect()
    }
}
