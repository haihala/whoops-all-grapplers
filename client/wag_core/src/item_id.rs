use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ItemId {
    Gi,
    Gun,
    Boots,
    SafetyBoots,
    HockeyPads,
    ThumbTacks(usize),

    // Character specific
    // Dummy
    Roids,

    // Mizku
    Kunai,
    SteelHeelBoots,

    #[default]
    Default,
}

impl ItemId {
    pub fn display_name(&self) -> String {
        match self {
            Self::HockeyPads => "Hockey pads".into(),
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
