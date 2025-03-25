use bevy::{asset::AssetPath, prelude::*, utils::HashMap};

use foundation::{Animation, RoninAnimation};

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
            RoninAnimation::Air,
            RoninAnimation::AirStab,
            RoninAnimation::AirStagger,
            RoninAnimation::AirThrowHit,
            RoninAnimation::AirThrowStartup,
            RoninAnimation::AirThrowTarget,
            RoninAnimation::Block,
            RoninAnimation::Crouch,
            RoninAnimation::CrouchBlock,
            RoninAnimation::CrouchStagger,
            RoninAnimation::CrouchThrowHit,
            RoninAnimation::CrouchThrowStartup,
            RoninAnimation::CrouchThrowTarget,
            RoninAnimation::BackDash,
            RoninAnimation::AirForwardDash,
            RoninAnimation::GroundForwardDash,
            RoninAnimation::FalconKnee,
            RoninAnimation::FootDiveHold,
            RoninAnimation::FootDiveRelease,
            RoninAnimation::Getup,
            RoninAnimation::GiParry,
            RoninAnimation::HeelKick,
            RoninAnimation::HighStab,
            RoninAnimation::Idle,
            RoninAnimation::Jump,
            RoninAnimation::KneeThrust,
            RoninAnimation::KunaiThrow,
            RoninAnimation::LowKick,
            RoninAnimation::RC,
            RoninAnimation::SkyStab,
            RoninAnimation::Stagger,
            RoninAnimation::StandThrowHit,
            RoninAnimation::StandThrowStartup,
            RoninAnimation::StandThrowTarget,
            RoninAnimation::StandPose,
            RoninAnimation::SwordStance,
            RoninAnimation::StanceCancel,
            RoninAnimation::FastViperStrike,
            RoninAnimation::SlowViperStrike,
            RoninAnimation::FastSwordSlam,
            RoninAnimation::SlowSwordSlam,
            RoninAnimation::FastSharpen,
            RoninAnimation::SlowSharpen,
            RoninAnimation::FastRisingSun,
            RoninAnimation::SlowRisingSun,
            RoninAnimation::TPose,
            RoninAnimation::Uppercut,
            RoninAnimation::WalkBack,
            RoninAnimation::WalkForward,
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
