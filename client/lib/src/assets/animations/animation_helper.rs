use std::mem::take;

use bevy::{prelude::*, scene::SceneInstance};

use characters::AnimationRequest;
use wag_core::{Animation, Facing, Hitstop, Stats};

use super::Animations;

#[derive(Debug, Component, Clone, Copy)]
pub struct AnimationHelper {
    pub player_entity: Entity,
    pub scene_root: Entity,
    default_animation: Animation,
    facing: Facing,
    request: Option<AnimationRequest>,
    playing: AnimationRequest,
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
            facing: Facing::default(),
            request: None,
            playing: AnimationRequest::default(),
        }
    }

    pub fn reset(&mut self) {
        self.play(self.default_animation.into());
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
pub struct AnimationHelperSetup(pub Animation);

pub fn setup_helpers(
    mut commands: Commands,
    to_setup: Query<(Entity, &AnimationHelperSetup)>,
    children: Query<&Children>,
    players: Query<&AnimationPlayer>,
    scenes: Query<&SceneInstance>,
) {
    for (host_entity, helper) in &to_setup {
        if let (Some(animation_player), Some(scene_root)) =
            find_animation_player_entity(host_entity, &children, &players, &scenes)
        {
            commands
                .entity(host_entity)
                .remove::<AnimationHelperSetup>()
                .insert(AnimationHelper::new(animation_player, scene_root, helper.0));
            // This is how I find it later and what I query for
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
    mut commands: Commands,
    animations: Res<Animations>,
    mut main: Query<(&mut AnimationHelper, &Facing, &Stats)>,
    mut players: Query<&mut AnimationPlayer>,
    mut scenes: Query<&mut Transform, With<Handle<Scene>>>,
    maybe_hitstop: Option<ResMut<Hitstop>>,
    graphs: ResMut<Assets<AnimationGraph>>,
) {
    for (mut helper, facing, stats) in &mut main {
        let mut player = players.get_mut(helper.player_entity).unwrap();
        let mut scene_root = scenes.get_mut(helper.scene_root).unwrap();

        if let Some(request) = helper.request.take() {
            // New animation set to start
            let (graph_handle, index) = animations.get(
                request.animation,
                &if request.invert {
                    facing.opposite()
                } else {
                    *facing
                },
                &graphs,
            );

            commands.entity(helper.player_entity).insert(graph_handle);

            let animation =
                player
                    .start(index)
                    .seek_to(0.0)
                    .set_speed(if request.ignore_action_speed {
                        1.0
                    } else {
                        stats.action_speed_multiplier
                    });

            // FIXME: There is something wrong with this.
            // First frames of the animation bleed through occasionally
            // It seems like the animation holds the first frame after it's done?
            if request.looping {
                animation.repeat();
            }

            helper.playing = request;
            helper.facing = *facing;
            scene_root.translation = request.position_offset.extend(0.0);

            // Looping animations like idle ought to turn when the sides switch. Non looping like moves should not
        } else if *facing != helper.facing && helper.playing.looping {
            // Sideswitch
            let (graph, index) = animations.get(helper.playing.animation, facing, &graphs);
            commands.entity(helper.player_entity).insert(graph);
            let elapsed = player.playing_animations().next().unwrap().1.elapsed();
            player.start(index).seek_to(elapsed).repeat();
            helper.facing = *facing;
        } else if maybe_hitstop.is_none() && player.all_paused() {
            // Hitstop is over, resume playing
            player.resume_all();
        } else if maybe_hitstop.is_some() && !player.all_paused() {
            // Hitstop started, pause
            player.pause_all();
        }
    }
}
