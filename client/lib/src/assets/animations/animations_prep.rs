use bevy::{prelude::*, utils::HashMap}; // Need to use this one for reflect to work

use wag_core::{Animation, DummyAnimation, Facing, MizkuAnimation};

#[derive(Debug, Default, Resource)]
pub struct Animations {
    normal: HashMap<Animation, Handle<AnimationClip>>,
    mirrored: HashMap<Animation, Handle<AnimationClip>>,
}

impl Animations {
    pub fn new(animations: HashMap<Animation, Handle<AnimationClip>>) -> Self {
        Self {
            normal: animations,
            mirrored: default(),
        }
    }

    fn all_loaded(&self, assets: &Assets<AnimationClip>) -> bool {
        self.normal
            .values()
            .map(|handle| assets.get(handle))
            .all(|clip| clip.is_some())
    }

    pub(super) fn get(&self, animation: Animation, flipped: &Facing) -> Handle<AnimationClip> {
        if animation == Animation::default() {
            // Default is not mirrored and mirrored animations may not be ready by the time it is requested
            // This should be irrelevant after a real loading screen.
            return self.normal[&Animation::default()].clone();
        }

        match flipped {
            Facing::Right => self.normal.get(&animation),
            Facing::Left => self.mirrored.get(&animation),
        }
        .unwrap()
        .clone()
    }
}

pub fn mirror_after_load(
    mut animations: ResMut<Animations>,
    maybe_assets: Option<ResMut<Assets<AnimationClip>>>, // For integration tests
    mut done: Local<bool>,
) {
    if *done || maybe_assets.is_none() {
        return;
    }
    let assets = &mut maybe_assets.unwrap();

    if animations.all_loaded(assets) && animations.mirrored.is_empty() {
        animations.mirrored = animations
            .normal
            .iter()
            .map(|(animation, handle)| {
                let clip = assets.get(handle).unwrap();

                let reflected: Box<dyn Struct> = Box::new(clip.to_owned());
                let ref_paths = reflected.field("paths").unwrap();
                let paths = ref_paths
                    .downcast_ref::<HashMap<EntityPath, usize>>()
                    .unwrap();

                let mirrored = paths.into_iter().fold(
                    AnimationClip::default(),
                    |mut clip_acc, (path, curves_index)| {
                        let mirrored_path = mirror_path(path.to_owned());

                        for curve in clip.get_curves(*curves_index).unwrap().iter() {
                            clip_acc.add_curve_to_path(
                                mirrored_path.clone(),
                                mirror_curve(curve.to_owned()),
                            );
                        }

                        clip_acc
                    },
                );

                (animation.to_owned(), assets.add(mirrored))
            })
            .collect();
        *done = true;
    }
}

fn mirror_path(original: EntityPath) -> EntityPath {
    EntityPath {
        parts: original
            .parts
            .into_iter()
            .map(|mut name| {
                // Transforms
                // - Bone.L -> Bone.R
                // - Bone.R -> Bone.L
                // - Bone.L.001 -> Bone.R.001
                // - Bone.R.001 -> Bone.L.001
                // Could be smarter, but I think that risks false positive hits and those seem annoying.
                // Assumes there are fewer than 100 bones with the same name

                name.mutate(|old_name| {
                    if let Some(base_name) = old_name.strip_suffix(".L") {
                        *old_name = base_name.to_owned() + ".R";
                    } else if let Some(base_name) = old_name.strip_suffix(".R") {
                        *old_name = base_name.to_owned() + ".L";
                    } else if old_name.contains(".R.0") {
                        *old_name = old_name.replace(".R.0", ".L.0");
                    } else if old_name.contains(".L.0") {
                        *old_name = old_name.replace(".L.0", ".R.0");
                    }
                });
                name
            })
            .collect(),
    }
}

fn mirror_curve(original: VariableCurve) -> VariableCurve {
    VariableCurve {
        keyframes: match original.keyframes {
            Keyframes::Rotation(frames) => Keyframes::Rotation(
                frames
                    .into_iter()
                    .map(|frame| {
                        let (axis, angle) = frame.to_axis_angle();
                        Quat::from_axis_angle(Vec3::new(-axis.x, axis.y, axis.z), -angle)
                    })
                    .collect(),
            ),
            Keyframes::Translation(frames) => Keyframes::Translation(
                frames
                    .into_iter()
                    .map(|frame| Vec3::new(-frame.x, frame.y, frame.z))
                    .collect(),
            ),
            scale => scale,
        },
        ..original
    }
}

pub fn animation_paths() -> HashMap<Animation, String> {
    // Every time a new animation is added, other animations may get affected
    // They are alphabetically ordered

    load_glb_animations(
        "dummy.glb".to_owned(),
        vec![
            DummyAnimation::AirIdle,
            DummyAnimation::AirSlam,
            DummyAnimation::AirSlap,
            DummyAnimation::AirStun,
            DummyAnimation::AirThrow,
            DummyAnimation::AirThrowRecipient,
            DummyAnimation::AntiAir,
            DummyAnimation::DashBack,
            DummyAnimation::BurnStraight,
            DummyAnimation::Crouch,
            DummyAnimation::CrouchBlock,
            DummyAnimation::CrouchChop,
            DummyAnimation::CrouchStun,
            DummyAnimation::Divekick,
            DummyAnimation::Dodge,
            DummyAnimation::DashForward,
            DummyAnimation::GroundSlam,
            DummyAnimation::Idle,
            DummyAnimation::Jump,
            DummyAnimation::NormalThrow,
            DummyAnimation::NormalThrowRecipient,
            DummyAnimation::Getup,
            DummyAnimation::Slap,
            DummyAnimation::StandBlock,
            DummyAnimation::StandStun,
            DummyAnimation::Sweep,
            DummyAnimation::TPose,
            DummyAnimation::WalkBack,
            DummyAnimation::WalkForward,
        ],
    )
    .into_iter()
    .chain(load_glb_animations(
        "mizuki.glb".to_owned(),
        vec![
            MizkuAnimation::Air,
            MizkuAnimation::AirStagger,
            MizkuAnimation::AirThrowHit,
            MizkuAnimation::AirThrowStartup,
            MizkuAnimation::AirThrowTarget,
            MizkuAnimation::BackSway,
            MizkuAnimation::Block,
            MizkuAnimation::Crouch,
            MizkuAnimation::CrouchBlock,
            MizkuAnimation::CrouchStagger,
            MizkuAnimation::DashBack,
            MizkuAnimation::DashForward,
            MizkuAnimation::FalconKnee,
            MizkuAnimation::FootDiveHold,
            MizkuAnimation::FootDiveRelease,
            MizkuAnimation::Getup,
            MizkuAnimation::GiParry,
            MizkuAnimation::GroundThrowHit,
            MizkuAnimation::GroundThrowStartup,
            MizkuAnimation::GroundThrowTarget,
            MizkuAnimation::HeelKick,
            MizkuAnimation::Idle,
            MizkuAnimation::Jump,
            MizkuAnimation::KneeThrust,
            MizkuAnimation::KunaiThrow,
            MizkuAnimation::LowKick,
            MizkuAnimation::Overhead,
            MizkuAnimation::Pilebunker,
            MizkuAnimation::ArisingSun,
            MizkuAnimation::GrisingSun,
            MizkuAnimation::Sharpen,
            MizkuAnimation::Stagger,
            MizkuAnimation::StandPose,
            MizkuAnimation::SwayCancel,
            MizkuAnimation::SwayDash,
            MizkuAnimation::SwayLow,
            MizkuAnimation::SwayOverhead,
            MizkuAnimation::Sweep,
            MizkuAnimation::TPose,
            MizkuAnimation::Uppercut,
            MizkuAnimation::WalkBack,
            MizkuAnimation::WalkForward,
        ],
    ))
    .collect()
}

fn load_glb_animations(
    file_path: String,
    animations: Vec<impl Into<Animation>>,
) -> HashMap<Animation, String> {
    animations
        .into_iter()
        .enumerate()
        .map(|(index, animation)| (animation.into(), format!("{file_path}#Animation{index}")))
        .collect()
}
