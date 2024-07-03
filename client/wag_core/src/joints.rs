use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
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

    // Only on some models, will cause panics if not present
    Katana,
}

impl Joint {
    // This would be somewhat cleaner if the format wasn't so bad
    pub fn flip(self) -> Self {
        match self {
            Abdomen => Abdomen,
            Chest => Chest,
            Neck => Neck,
            Head => Head,
            Katana => Katana,
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
        Some(match str_input.as_str() {
            // TODO: Later ones are from meta rig, untested
            // Symmetrical, not sure about these at all
            "Abdomen" | "DEF-spine" => Joint::Abdomen,
            "Chest" | "DEF-spine.002" => Joint::Chest,
            "Neck" | "DEF-spine.005" => Joint::Neck,
            "Head" | "DEF-spine.006" => Joint::Head,
            "DEF-sword-active" => Joint::Katana,

            // Hands
            "Shoulder.R" | "DEF-shoulder.R" => Joint::ShoulderR,
            "Shoulder.L" | "DEF-shoulder.L" => Joint::ShoulderL,
            "UpperArm.R" | "DEF-upper_arm.R" => Joint::UpperArmR,
            "UpperArm.L" | "DEF-upper_arm.L" => Joint::UpperArmL,
            "ForeArm.R" | "DEF-forearm.R" => Joint::ForeArmR,
            "ForeArm.L" | "DEF-forearm.L" => Joint::ForeArmL,
            "Hand.R" | "DEF-hand.R" => Joint::HandR,
            "Hand.L" | "DEF-hand.L" => Joint::HandL,

            // Feet
            "Hip.R" | "DEF-pelvis.R" => Joint::HipR,
            "Hip.L" | "DEF-pelvis.L" => Joint::HipL,
            "Thigh.R" | "DEF-thigh.R" => Joint::ThighR,
            "Thigh.L" | "DEF-thigh.L" => Joint::ThighL,
            "Shin.R" | "DEF-shin.R" => Joint::ShinR,
            "Shin.L" | "DEF-shin.L" => Joint::ShinL,
            "Foot.R" | "DEF-foot.R" => Joint::FootR,
            "Foot.L" | "DEF-foot.L" => Joint::FootL,
            "Toes.R" | "DEF-toe.R" => Joint::ToesR,
            "Toes.L" | "DEF-toe.L" => Joint::ToesL,
            _ => return None,
        })
    }
}

use Joint::*;

#[derive(Debug, Component, Default, Clone, PartialEq, Reflect)]
pub struct JointCollider {
    pub joints: Vec<Joint>,
    pub padding: f32,
}

#[derive(Debug, Component, Reflect)]
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
                JointCollider {
                    joints: vec![Head],
                    padding: 0.2,
                },
                // Torso
                JointCollider {
                    joints: vec![
                        Neck, Chest, Abdomen, ShoulderL, ShoulderR, HipL, HipR, UpperArmL,
                        UpperArmR,
                    ],
                    padding: 0.1,
                },
                // Right arm
                JointCollider {
                    joints: vec![UpperArmR, ForeArmR],
                    padding: 0.1,
                },
                JointCollider {
                    joints: vec![ForeArmR, HandR],
                    padding: 0.1,
                },
                // Left arm
                JointCollider {
                    joints: vec![UpperArmL, ForeArmL],
                    padding: 0.1,
                },
                JointCollider {
                    joints: vec![ForeArmL, HandL],
                    padding: 0.1,
                },
                // Right leg
                JointCollider {
                    joints: vec![ThighR, ShinR],
                    padding: 0.1,
                },
                JointCollider {
                    joints: vec![ShinR, FootR],
                    padding: 0.1,
                },
                // Left leg
                JointCollider {
                    joints: vec![ThighL, ShinL],
                    padding: 0.1,
                },
                JointCollider {
                    joints: vec![ShinL, FootL],
                    padding: 0.1,
                },
            ],
        }
    }
}
