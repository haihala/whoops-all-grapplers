use bevy::prelude::*;
use map_macro::map;
use std::collections::HashMap;

use types::{Animation, DummyAnimation, Facing};

#[derive(Debug, Default)]
pub struct Animations {
    normal: HashMap<Animation, Handle<AnimationClip>>,
    mirrored: HashMap<Animation, Handle<AnimationClip>>,
}

impl Animations {
    pub fn new(animations: HashMap<Animation, Handle<AnimationClip>>) -> Self {
        Self {
            normal: animations,
            mirrored: map! {},
        }
    }

    fn all_loaded(&self, assets: &Assets<AnimationClip>) -> bool {
        self.normal
            .iter()
            .map(|(_, handle)| handle)
            .all(|handle| assets.get(handle).is_some())
    }

    fn get(&self, animation: Animation, flipped: &Facing) -> Handle<AnimationClip> {
        match flipped {
            Facing::Right => self.normal.get(&animation),
            Facing::Left => self.mirrored.get(&animation),
        }
        .unwrap()
        .clone()
    }
}

#[derive(Debug, Component)]
pub struct AnimationHelper {
    pub player_entity: Entity,
    pub current: Animation,
    next: Option<Animation>,
}

impl AnimationHelper {
    fn new(player_entity: Entity) -> AnimationHelper {
        AnimationHelper {
            player_entity,
            current: Animation::TPose,
            next: None,
        }
    }
    pub fn play(&mut self, new: Animation) {
        self.next = if new != self.current { Some(new) } else { None }
    }
}

pub(super) fn mirror_after_load(
    mut animations: ResMut<Animations>,
    mut assets: ResMut<Assets<AnimationClip>>,
) {
    if animations.all_loaded(&assets) && animations.mirrored.is_empty() {
        animations.mirrored = animations
            .normal
            .iter()
            .map(|(animation, handle)| {
                let mirrored = assets.get(handle).unwrap().curves().into_iter().fold(
                    AnimationClip::default(),
                    |clip, (path, curves)| {
                        let mirrored_path = mirror_path(path.to_owned());
                        curves.iter().cloned().fold(clip, |mut acc, curve| {
                            acc.add_curve_to_path(mirrored_path.clone(), mirror_curve(curve));
                            acc
                        })
                    },
                );
                (animation.to_owned(), assets.add(mirrored))
            })
            .collect();
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

pub fn update_animation(
    animations: Res<Animations>,
    mut main: Query<(&mut AnimationHelper, &Facing)>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for (mut helper, facing) in main.iter_mut() {
        if let Some(next) = helper.next {
            let mut player = players.get_mut(helper.player_entity).unwrap();
            let asset = animations.get(next, facing);
            player.play(asset).repeat(); // Just in case, non-repeating actions change state so this doesn't matter
            helper.current = next;
        }
    }
}

#[derive(Debug, Component)]
pub struct AnimationHelperSetup;

pub fn setup_helpers(
    mut commands: Commands,
    to_setup: Query<Entity, With<AnimationHelperSetup>>,
    children: Query<&Children>,
    players: Query<&AnimationPlayer>,
) {
    for host_entity in to_setup.iter() {
        if let Some(animation_player) =
            find_animation_player_entity(host_entity, &children, &players)
        {
            commands
                .entity(host_entity)
                .remove::<AnimationHelperSetup>()
                .insert(AnimationHelper::new(animation_player)); // This is how I find it later and  what I query for
        }
    }
}

fn find_animation_player_entity(
    parent: Entity,
    children: &Query<&Children>,
    players: &Query<&AnimationPlayer>,
) -> Option<Entity> {
    if let Ok(candidates) = children.get(parent) {
        let mut next_candidates: Vec<Entity> = candidates.iter().map(|e| e.to_owned()).collect();
        while !next_candidates.is_empty() {
            for candidate in next_candidates.drain(..).collect::<Vec<Entity>>() {
                if players.get(candidate).is_ok() {
                    return Some(candidate);
                } else if let Ok(new) = children.get(candidate) {
                    next_candidates.extend(new.iter());
                }
            }
        }
    }
    None
}

pub(super) fn animation_paths() -> HashMap<Animation, &'static str> {
    map! {
        Animation::Dummy(DummyAnimation::Idle) => "dummy.glb#Animation0",
        Animation::TPose => "dummy.glb#Animation1",
    }
}
