use std::mem::take;

use bevy::{prelude::*, scene::SceneInstance};

use wag_core::{Animation, Facing, Stats};

use super::Animations;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AnimationRequest {
    pub animation: Animation,
    pub time_offset: usize,
    pub position_offset: Vec2,
    pub invert: bool,
    pub looping: bool,
    pub ignore_action_speed: bool,
}
impl From<Animation> for AnimationRequest {
    fn from(animation: Animation) -> Self {
        Self {
            animation,
            ..default()
        }
    }
}

#[derive(Debug, Component)]
pub struct AnimationHelper {
    pub player_entity: Entity,
    pub scene_root: Entity,
    facing: Facing,
    request: Option<AnimationRequest>,
    playing: AnimationRequest,
}
impl AnimationHelper {
    fn new(player_entity: Entity, scene_root: Entity) -> AnimationHelper {
        AnimationHelper {
            player_entity,
            scene_root,
            facing: Facing::default(),
            request: None,
            playing: AnimationRequest::default(),
        }
    }

    pub fn play(&mut self, new: AnimationRequest) {
        self.request = Some(new);
    }

    pub fn play_if_new(&mut self, new: AnimationRequest) {
        if self.playing != new {
            self.play(new);
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
    scenes: Query<&SceneInstance>,
) {
    for host_entity in &to_setup {
        if let (Some(animation_player), Some(scene_root)) =
            find_animation_player_entity(host_entity, &children, &players, &scenes)
        {
            commands
                .entity(host_entity)
                .remove::<AnimationHelperSetup>()
                .insert(AnimationHelper::new(animation_player, scene_root)); // This is how I find it later and what I query for
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
    mut main: Query<(&mut AnimationHelper, &Facing, &Stats)>,
    mut players: Query<&mut AnimationPlayer>,
    mut scenes: Query<&mut Transform, With<Handle<Scene>>>,
) {
    for (mut helper, facing, stats) in &mut main {
        let mut player = players.get_mut(helper.player_entity).unwrap();
        let mut scene_root = scenes.get_mut(helper.scene_root).unwrap();

        if let Some(request) = helper.request.take() {
            // New animation set to start
            let handle = animations.get(
                request.animation,
                &if request.invert {
                    facing.opposite()
                } else {
                    *facing
                },
            );

            player
                .start(handle)
                .set_elapsed(request.time_offset as f32 / wag_core::FPS)
                .set_speed(if request.ignore_action_speed {
                    1.0
                } else {
                    stats.action_speed_multiplier
                });

            if request.looping {
                player.repeat();
            }

            helper.playing = request;
            helper.facing = *facing;
            scene_root.translation = request.position_offset.extend(0.0);

            // Looping animations like idle ought to turn when the sides switch. Non looping like moves should not
        } else if *facing != helper.facing && helper.playing.looping {
            // Sideswitch
            let handle = animations.get(helper.playing.animation, facing);
            let elapsed = player.elapsed();
            player.start(handle).set_elapsed(elapsed).repeat();
            helper.facing = *facing;
        }
    }
}
