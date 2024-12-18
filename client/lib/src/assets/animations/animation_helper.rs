use std::{mem::take, time::Duration};

use bevy::{prelude::*, scene::SceneInstance};

use characters::AnimationRequest;
use foundation::{Animation, Hitstop, Stats};

use super::Animations;

#[derive(Debug, Component, Clone, Copy)]
pub struct AnimationHelper {
    pub player_entity: Entity,
    pub scene_root: Entity,
    default_animation: Animation,
    request: Option<AnimationRequest>,
    playing: Option<AnimationRequest>,
}
impl AnimationHelper {
    fn new(
        player_entity: Entity,
        scene_root: Entity,
        default_animation: Animation,
    ) -> AnimationHelper {
        AnimationHelper {
            player_entity,
            scene_root,
            default_animation,
            request: None,
            playing: None,
        }
    }

    pub fn reset(&mut self) {
        self.play(self.default_animation.into());
    }

    pub fn play(&mut self, new: AnimationRequest) {
        self.request = Some(new);
    }

    pub fn play_if_new(&mut self, new: AnimationRequest) {
        if self.playing != Some(new) {
            self.play(new);
        }
    }
}
#[derive(Debug, Component)]
pub struct AnimationHelperSetup(pub Animation);

pub fn setup_helpers(
    mut commands: Commands,
    to_setup: Query<(Entity, &AnimationHelperSetup)>,
    children: Query<&Children>,
    players: Query<&AnimationPlayer>,
    scenes: Query<&SceneInstance>,
    animations: Res<Animations>,
) {
    for (host_entity, helper) in &to_setup {
        if let (Some(animation_player), Some(scene_root)) =
            find_animation_player_entity(host_entity, &children, &players, &scenes)
        {
            commands
                .entity(host_entity)
                .remove::<AnimationHelperSetup>()
                .insert(AnimationHelper::new(animation_player, scene_root, helper.0));

            commands
                .entity(animation_player)
                .insert(AnimationTransitions::new())
                .insert(AnimationGraphHandle(animations.monograph.clone()));
        }
    }
}

fn find_animation_player_entity(
    parent: Entity,
    children: &Query<&Children>,
    players: &Query<&AnimationPlayer>,
    scenes: &Query<&SceneInstance>,
) -> (Option<Entity>, Option<Entity>) {
    if let Ok(candidates) = children.get(parent) {
        let mut next_candidates: Vec<Entity> = candidates.iter().map(|e| e.to_owned()).collect();
        let mut scene_root = None;
        while !next_candidates.is_empty() {
            for candidate in take(&mut next_candidates) {
                if players.get(candidate).is_ok() {
                    return (Some(candidate), scene_root);
                } else if let Ok(new) = children.get(candidate) {
                    if scenes.get(candidate).is_ok() {
                        scene_root = Some(candidate);
                    }
                    next_candidates.extend(new.iter());
                }
            }
        }
    }
    (None, None)
}

pub fn update_animation(
    animations: Res<Animations>,
    mut main: Query<(&mut AnimationHelper, &Stats, Option<&Hitstop>)>,
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    mut scenes: Query<&mut Transform, With<SceneRoot>>,
) {
    for (mut helper, stats, maybe_histop) in &mut main {
        let (mut player, mut transitions) = players.get_mut(helper.player_entity).unwrap();
        let mut scene_root = scenes.get_mut(helper.scene_root).unwrap();

        if let Some(request) = helper.request.take() {
            dbg!(request.animation);
            let index = animations.get(request.animation);

            player.stop_all();
            let active = transitions.play(&mut player, index, Duration::ZERO);

            if request.looping {
                active.repeat();
            }

            if !request.ignore_action_speed {
                active.set_speed(stats.action_speed_multiplier);
            }

            if maybe_histop.is_some() {
                active.pause();
            }

            helper.playing = Some(request);
            scene_root.translation = request.position_offset.extend(0.0);
        }
    }
}

// This is called when entering postround, so the freeze frame is still
pub fn pause_animations(mut players: Query<&mut AnimationPlayer>) {
    for mut player in &mut players {
        player.pause_all();
    }
}
