use bevy::prelude::*;

#[derive(
    Reflect, FromReflect, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub enum ItemId {
    Gi,
    Gun,
    HandMeDownKen,
    Roids,

    #[default]
    Default,
}

impl ItemId {
    pub fn display_name(&self) -> String {
        match self {
            Self::HandMeDownKen => String::from("Hand me down -ken"),
            _ => format!("{:?}", self),
        }
    }
}
