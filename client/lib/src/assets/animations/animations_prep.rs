use bevy::{asset::AssetPath, prelude::*, utils::HashMap};

use foundation::{Animation, SamuraiAnimation};

#[derive(Debug, Default, Resource)]
pub struct Animations {
    pub monograph: Handle<AnimationGraph>,
    animations: HashMap<Animation, AnimationNodeIndex>,
}

impl Animations {
    pub fn new(
        monograph: Handle<AnimationGraph>,
        animations: HashMap<Animation, AnimationNodeIndex>,
    ) -> Self {
        Self {
            monograph,
            animations,
        }
    }

    pub fn get(&self, animation: Animation) -> AnimationNodeIndex {
        *self.animations.get(&animation).unwrap()
    }
}

pub fn animation_paths() -> HashMap<Animation, AssetPath<'static>> {
    // Every time a new animation is added, other animations may get affected
    // They are alphabetically ordered

    load_glb_animations(
        "samurai.glb".to_owned(),
        vec![
            SamuraiAnimation::Air,
            SamuraiAnimation::AirStab,
            SamuraiAnimation::AirStagger,
            SamuraiAnimation::AirThrowHit,
            SamuraiAnimation::AirThrowStartup,
            SamuraiAnimation::AirThrowTarget,
            SamuraiAnimation::Block,
            SamuraiAnimation::Crouch,
            SamuraiAnimation::CrouchBlock,
            SamuraiAnimation::CrouchStagger,
            SamuraiAnimation::CrouchThrowHit,
            SamuraiAnimation::CrouchThrowStartup,
            SamuraiAnimation::CrouchThrowTarget,
            SamuraiAnimation::BackDash,
            SamuraiAnimation::AirForwardDash,
            SamuraiAnimation::GroundForwardDash,
            SamuraiAnimation::FalconKnee,
            SamuraiAnimation::FootDiveHold,
            SamuraiAnimation::FootDiveRelease,
            SamuraiAnimation::Getup,
            SamuraiAnimation::GiParry,
            SamuraiAnimation::HeelKick,
            SamuraiAnimation::HighStab,
            SamuraiAnimation::Idle,
            SamuraiAnimation::Jump,
            SamuraiAnimation::KneeThrust,
            SamuraiAnimation::KunaiThrow,
            SamuraiAnimation::LowKick,
            SamuraiAnimation::RC,
            SamuraiAnimation::SkyStab,
            SamuraiAnimation::Stagger,
            SamuraiAnimation::StandThrowHit,
            SamuraiAnimation::StandThrowStartup,
            SamuraiAnimation::StandThrowTarget,
            SamuraiAnimation::StandPose,
            SamuraiAnimation::SwordStance,
            SamuraiAnimation::StanceCancel,
            SamuraiAnimation::FastViperStrike,
            SamuraiAnimation::SlowViperStrike,
            SamuraiAnimation::FastSwordSlam,
            SamuraiAnimation::SlowSwordSlam,
            SamuraiAnimation::FastSharpen,
            SamuraiAnimation::SlowSharpen,
            SamuraiAnimation::FastRisingSun,
            SamuraiAnimation::SlowRisingSun,
            SamuraiAnimation::TPose,
            SamuraiAnimation::Uppercut,
            SamuraiAnimation::WalkBack,
            SamuraiAnimation::WalkForward,
        ],
    )
}

fn load_glb_animations(
    file_path: String,
    animations: Vec<impl Into<Animation>>,
) -> HashMap<Animation, AssetPath<'static>> {
    animations
        .into_iter()
        .enumerate()
        .map(|(index, animation)| {
            (
                animation.into(),
                GltfAssetLabel::Animation(index).from_asset(file_path.to_owned()),
            )
        })
        .collect()
}
