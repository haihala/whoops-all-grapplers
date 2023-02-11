use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use bevy::{
    prelude::{Component, Entity},
    utils::HashMap,
};

// For the Dummy model
// Using facing was considered, but that has an issue with creating the iterator so we can map nodes to the joints
#[derive(Debug, EnumIter, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Joint {
    Abdomen,
    Chest,
    Neck,
    Head,

    ShoulderR,
    ShoulderL,
    UpperArmR,
    UpperArmL,
    ForeArmR,
    ForeArmL,
    // Model does contain individual fingers, but those can be ignored
    HandR,
    HandL,

    HipR,
    HipL,
    ThighR,
    ThighL,
    ShinR,
    ShinL,
    FootR,
    FootL,
    ToesR,
    ToesL,
}

impl Joint {
    pub fn from_model_string(input: impl Into<String>) -> Option<Self> {
        let str_input: String = input.into();

        Self::iter().find(|&variant| str_input == variant.model_string())
    }

    pub fn model_string(&self) -> String {
        match self {
            Joint::Abdomen => "Abdomen",
            Joint::Chest => "Chest",
            Joint::Neck => "Neck",
            Joint::Head => "Head",

            Joint::ShoulderR => "Shoulder.R",
            Joint::ShoulderL => "Shoulder.L",
            Joint::UpperArmR => "UpperArm.R",
            Joint::UpperArmL => "UpperArm.L",
            Joint::ForeArmR => "ForeArm.R",
            Joint::ForeArmL => "ForeArm.L",
            Joint::HandR => "Hand.R",
            Joint::HandL => "Hand.L",

            Joint::HipR => "Hip.R",
            Joint::HipL => "Hip.L",
            Joint::ThighR => "Thigh.R",
            Joint::ThighL => "Thigh.L",
            Joint::ShinR => "Shin.R",
            Joint::ShinL => "Shin.L",
            Joint::FootR => "Foot.R",
            Joint::FootL => "Foot.L",
            Joint::ToesR => "Toes.R",
            Joint::ToesL => "Toes.L",
        }
        .into()
    }
}

#[derive(Debug, Default, Component)]
pub struct Joints {
    pub content: HashMap<Joint, Entity>,
    pub initialized: bool,
}
