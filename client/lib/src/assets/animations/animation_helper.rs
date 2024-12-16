use std::{mem::take, time::Duration};

use bevy::{prelude::*, scene::SceneInstance};

use characters::AnimationRequest;
use foundation::{Animation, Hitstop, MatchState, Stats};

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
                .insert(animations.monograph.clone());
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

#[allow(clippy::too_many_arguments)]
pub fn update_animation(
    animations: Res<Animations>,
    mut main: Query<(&mut AnimationHelper, &Stats)>,
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    mut scenes: Query<&mut Transform, With<Handle<Scene>>>,
    maybe_hitstop: Option<ResMut<Hitstop>>,
    mut hitstop_last_frame: Local<bool>,
    match_state: Res<State<MatchState>>,
) {
    let hitstop_this_frame = maybe_hitstop.is_some();
    let (hitstop_started, hitstop_ended) = if hitstop_this_frame != *hitstop_last_frame {
        (hitstop_this_frame, *hitstop_last_frame)
    } else {
        (false, false)
    };
    let post_round = *match_state.get() == MatchState::PostRound;

    *hitstop_last_frame = hitstop_this_frame;

    for (mut helper, stats) in &mut main {
        let (mut player, mut transitions) = players.get_mut(helper.player_entity).unwrap();
        let mut scene_root = scenes.get_mut(helper.scene_root).unwrap();

        if let Some(request) = helper.request.take() {
            // New animation set to start
            let index = animations.get(request.animation);

            player.stop_all();
            let active = transitions.play(&mut player, index, Duration::ZERO);

            if request.looping {
                active.repeat();
            }

            if !request.ignore_action_speed {
                active.set_speed(stats.action_speed_multiplier);
            }

            helper.playing = Some(request);
            scene_root.translation = request.position_offset.extend(0.0);
        } else if hitstop_ended && !post_round {
            // Don't pause in post round time, as that would make animations play during the
            // freeze time, invalidating pause_animations. Last hit has hitstop.
            player.resume_all();
        } else if hitstop_started {
            player.pause_all();
        }
    }
}

// This is called when entering postround, so the freeze frame is still
pub fn pause_animations(mut players: Query<&mut AnimationPlayer>) {
    for mut player in &mut players {
        player.pause_all();
    }
}
