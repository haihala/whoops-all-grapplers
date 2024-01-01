use bevy::utils::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Icon {
    ThumbTacks(usize),
    Boots,
    Crowbar,
    Feather,
    OliveOil,
    RedPaint,
    Dumbbell,
    Stopwatch,
}
impl Icon {
    pub fn paths() -> HashMap<Icon, String> {
        vec![
            (Icon::Boots, "icons/boots.png".into()),
            (Icon::Crowbar, "icons/crowbar.png".into()),
            (Icon::Feather, "icons/feather.png".into()),
            (Icon::OliveOil, "icons/olive-oil.png".into()),
            (Icon::RedPaint, "icons/red-paint.png".into()),
            (Icon::Dumbbell, "icons/dumbbell.png".into()),
            (Icon::Stopwatch, "icons/stopwatch.png".into()),
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
