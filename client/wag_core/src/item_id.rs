use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ItemId {
    Gi,
    Gun,
    Boots,
    SafetyBoots,
    HockeyPads,

    // Character specific
    // Dummy
    Roids,

    // Mizku
    Kunai,

    #[default]
    Default,
}

impl ItemId {
    pub fn display_name(&self) -> String {
        match self {
            Self::HockeyPads => "Hockey pads".into(),
            Self::SafetyBoots => "Safety boots".into(),
            Self::Gi => "Gi of the old masters".into(),
            _ => format!("{:?}", self),
        }
    }
}
