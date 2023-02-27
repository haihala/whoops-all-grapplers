use bevy::prelude::*;

#[derive(
    Reflect, FromReflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub enum ItemId {
    Gi,
    Gun,
    HandMeDownKen,
    Roids,
    Boots,
    SafetyBoots,

    #[default]
    Default,
}

impl ItemId {
    pub fn display_name(&self) -> String {
        match self {
            Self::HandMeDownKen => "Hand me down -ken".into(),
            Self::SafetyBoots => "Safety boots".into(),
            Self::Gi => "Gi of the old masters".into(),
            _ => format!("{:?}", self),
        }
    }
}
