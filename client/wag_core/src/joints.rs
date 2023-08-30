use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use bevy::{prelude::*, utils::HashMap};

// For the Dummy model
// Using facing was considered, but that has an issue with creating the iterator so we can map nodes to the joints
#[derive(Debug, EnumIter, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
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
    // This would be somewhat cleaner if the format wasn't so bad
    pub fn flip(self) -> Self {
        match self {
            Abdomen => Abdomen,
            Chest => Chest,
            Neck => Neck,
            Head => Head,
            ShoulderR => ShoulderL,
            ShoulderL => ShoulderR,
            UpperArmR => UpperArmL,
            UpperArmL => UpperArmR,
            ForeArmR => ForeArmL,
            ForeArmL => ForeArmR,
            HandR => HandL,
            HandL => HandR,
            HipR => HipL,
            HipL => HipR,
            ThighR => ThighL,
            ThighL => ThighR,
            ShinR => ShinL,
            ShinL => ShinR,
            FootR => FootL,
            FootL => FootR,
            ToesR => ToesL,
            ToesL => ToesR,
        }
    }

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

use Joint::*;

#[derive(Debug, Component, Clone, Deref, DerefMut, PartialEq)]
pub struct JointCollider(pub Vec<Joint>);
impl From<Vec<Joint>> for JointCollider {
    fn from(value: Vec<Joint>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Component)]
pub struct Joints {
    pub nodes: HashMap<Joint, Entity>,
    pub colliders: Vec<JointCollider>,
}
impl Default for Joints {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            colliders: vec![
                // Head
                vec![Head, Neck, ShoulderL, ShoulderR].into(),
                // Torso
                vec![
                    Neck, Chest, Abdomen, ShoulderL, ShoulderR, HipL, HipR, UpperArmL, UpperArmR,
                ]
                .into(),
                // Right arm
                vec![UpperArmR, ForeArmR].into(),
                vec![ForeArmR, HandR].into(),
                // Left arm
                vec![UpperArmL, ForeArmL].into(),
                vec![ForeArmL, HandL].into(),
                // Right leg
                vec![ThighR, ShinR].into(),
                vec![ShinR, FootR].into(),
                // Left leg
                vec![ThighL, ShinL].into(),
                vec![ShinL, FootL].into(),
            ],
        }
    }
}
