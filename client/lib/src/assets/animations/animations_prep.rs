use bevy::prelude::*;
use std::collections::HashMap;

use wag_core::{Animation, DummyAnimation, Facing};

#[derive(Debug, Default)]
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
        if animation == Animation::TPose {
            // TPose is not mirrored and mirrored animations may not be ready by the time TPose is requested
            // This should be irrelevant after a real loading screen.
            return self.normal[&Animation::TPose].clone();
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
    mut maybe_assets: Option<ResMut<Assets<AnimationClip>>>, // For integration tests
    mut done: Local<bool>,
) {
    if !*done {
        if let Some(ref mut assets) = maybe_assets {
            if animations.all_loaded(assets) && animations.mirrored.is_empty() {
                animations.mirrored = animations
                    .normal
                    .iter()
                    .map(|(animation, handle)| {
                        let mirrored = assets.get(handle).unwrap().curves().into_iter().fold(
                            AnimationClip::default(),
                            |clip, (path, curves)| {
                                let mirrored_path = mirror_path(path.to_owned());
                                curves.iter().cloned().fold(clip, |mut acc, curve| {
                                    acc.add_curve_to_path(
                                        mirrored_path.clone(),
                                        mirror_curve(curve),
                                    );
                                    acc
                                })
                            },
                        );
                        (animation.to_owned(), assets.add(mirrored))
                    })
                    .collect();
                *done = true;
            }
        } else {
            // We're in integration tests
            *done = true;
        }
    }
}

fn mirror_path(original: EntityPath) -> EntityPath {
    EntityPath {
        parts: original
            .parts
            .into_iter()
            .map(|mut name| {
                // Transforms Bone.L -> Bone.R and Bone.R -> Bone.L
                name.mutate(|old_name| {
                    if let Some(base_name) = old_name.strip_suffix(".L") {
                        *old_name = base_name.to_owned() + ".R";
                    } else if let Some(base_name) = old_name.strip_suffix(".R") {
                        *old_name = base_name.to_owned() + ".L";
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
            Animation::Dummy(DummyAnimation::AirIdle),
            Animation::Dummy(DummyAnimation::AirSlam),
            Animation::Dummy(DummyAnimation::AirSlap),
            Animation::Dummy(DummyAnimation::AirStun),
            Animation::Dummy(DummyAnimation::AntiAir),
            Animation::Dummy(DummyAnimation::DashBack),
            Animation::Dummy(DummyAnimation::BurnStraight),
            Animation::Dummy(DummyAnimation::Crouch),
            Animation::Dummy(DummyAnimation::CrouchBlock),
            Animation::Dummy(DummyAnimation::CrouchChop),
            Animation::Dummy(DummyAnimation::CrouchStun),
            Animation::Dummy(DummyAnimation::Divekick),
            Animation::Dummy(DummyAnimation::Dodge),
            Animation::Dummy(DummyAnimation::DashForward),
            Animation::Dummy(DummyAnimation::GroundSlam),
            Animation::Dummy(DummyAnimation::Idle),
            Animation::Dummy(DummyAnimation::Jump),
            Animation::Dummy(DummyAnimation::NormalThrow),
            Animation::Dummy(DummyAnimation::NormalThrowRecipient),
            Animation::Dummy(DummyAnimation::Getup),
            Animation::Dummy(DummyAnimation::Slap),
            Animation::Dummy(DummyAnimation::StandBlock),
            Animation::Dummy(DummyAnimation::StandStun),
            Animation::Dummy(DummyAnimation::Sweep),
            Animation::TPose,
            Animation::Dummy(DummyAnimation::WalkBack),
            Animation::Dummy(DummyAnimation::WalkForward),
        ],
    )
}

fn load_glb_animations(
    file_path: String,
    animations: Vec<Animation>,
) -> HashMap<Animation, String> {
    animations
        .into_iter()
        .enumerate()
        .map(|(index, animation)| (animation, format!("{file_path}#Animation{index}")))
        .collect()
}
