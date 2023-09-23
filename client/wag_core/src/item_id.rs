use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ItemId {
    Gi,
    Gun,
    Roids,
    Boots,
    SafetyBoots,

    #[default]
    Default,
}

impl ItemId {
    pub fn display_name(&self) -> String {
        match self {
            Self::SafetyBoots => "Safety boots".into(),
            Self::Gi => "Gi of the old masters".into(),
            _ => format!("{:?}", self),
        }
    }
}
