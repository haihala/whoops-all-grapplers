use bevy::{asset::AssetPath, prelude::*, utils::HashMap};

use foundation::{Animation, CPOAnimation, RoninAnimation};

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

    let ronin_anims: Vec<_> = load_glb_animations(
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
    .collect();

    let cpo_anims: Vec<_> = load_glb_animations(
        "cpo.glb".to_owned(),
        vec![
            CPOAnimation::BlockCrouch,
            CPOAnimation::BlockStand,
            CPOAnimation::BodySplash,
            CPOAnimation::Chop,
            CPOAnimation::Dance,
            CPOAnimation::DashAirBack,
            CPOAnimation::DashAirForward,
            CPOAnimation::DashGroundBack,
            CPOAnimation::DashGroundForward,
            CPOAnimation::DickJab,
            CPOAnimation::Getup,
            CPOAnimation::GiParry,
            CPOAnimation::HitAir,
            CPOAnimation::HitCrouch,
            CPOAnimation::HitStand,
            CPOAnimation::HookPunch,
            CPOAnimation::IdleAir,
            CPOAnimation::IdleCrouch,
            CPOAnimation::IdleStand,
            CPOAnimation::Jump,
            CPOAnimation::JumpingKnees,
            CPOAnimation::NeutralStandPose, // Actually reasonable default pose
            CPOAnimation::Overhead,
            CPOAnimation::RC,
            CPOAnimation::PayCheckHit,
            CPOAnimation::PayCheckRecipient,
            CPOAnimation::PayCheckStartup,
            CPOAnimation::Stomp1,
            CPOAnimation::Stomp2,
            CPOAnimation::Stomp3,
            CPOAnimation::Sugarcoat,
            CPOAnimation::TPose,
            CPOAnimation::ThrowAirHit,
            CPOAnimation::ThrowAirRecipient,
            CPOAnimation::ThrowAirStartup,
            CPOAnimation::ThrowGroundBackRecipient,
            CPOAnimation::ThrowGroundForwardRecipient,
            CPOAnimation::ThrowGroundHit,
            CPOAnimation::ThrowGroundStartup,
            CPOAnimation::TimewinderAirShoulder,
            CPOAnimation::TimewinderAirStrike,
            CPOAnimation::TimewinderGroundLow,
            CPOAnimation::TimewinderGroundShoulder,
            CPOAnimation::TimewinderGroundStraight,
            CPOAnimation::WalkBack,
            CPOAnimation::WalkForward,
        ],
    )
    .collect();

    [ronin_anims, cpo_anims].into_iter().flatten().collect()
}

fn load_glb_animations(
    file_path: String,
    animations: Vec<impl Into<Animation>>,
) -> impl Iterator<Item = (Animation, AssetPath<'static>)> {
    animations
        .into_iter()
        .enumerate()
        .map(move |(index, animation)| {
            (
                animation.into(),
                GltfAssetLabel::Animation(index).from_asset(file_path.to_owned()),
            )
        })
}
