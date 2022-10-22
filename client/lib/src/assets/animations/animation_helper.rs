use bevy::prelude::*;

use core::{Animation, Facing};

use super::Animations;

#[derive(Debug, Default)]
pub struct AnimationRequest {
    pub animation: Animation,
    pub offset: usize,
    pub low_priority: bool,
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
    pub current: Animation,
    pub low_priority: bool, // This exists to see if it can be overridden by general animations
    facing: Facing,
    request: Option<AnimationRequest>,
}
impl AnimationHelper {
    fn new(player_entity: Entity) -> AnimationHelper {
        AnimationHelper {
            player_entity,
            current: Animation::TPose,
            facing: Facing::default(),
            request: None,
            low_priority: true,
        }
    }
    pub fn play(&mut self, new: AnimationRequest) {
        self.request = Some(new);
    }

    fn set_playing(&mut self, animation: Animation, facing: Facing) {
        self.current = animation;
        self.facing = facing;
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
    for host_entity in &to_setup {
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

pub fn update_animation(
    assets: Res<Assets<AnimationClip>>,
    animations: Res<Animations>,
    mut main: Query<(&mut AnimationHelper, &Facing)>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for (mut helper, facing) in &mut main {
        let mut player = players.get_mut(helper.player_entity).unwrap();
        let handle = animations.get(helper.current, facing);
        if let Some(request) = helper.request.take() {
            // New animation set to start
            player
                .play(animations.get(request.animation, facing))
                .set_elapsed(request.offset as f32 * core::FPS);
            helper.set_playing(request.animation, *facing);
            helper.low_priority = request.low_priority;
        } else if *facing != helper.facing {
            // Sideswitch
            let elapsed = player.elapsed();
            player.play(handle).set_elapsed(elapsed).repeat();
            let current = helper.current;
            helper.set_playing(current, *facing);
        } else if player.elapsed() >= assets.get(&handle).unwrap().duration() {
            // Animation has been playing for over it's duration, loop it
            if helper.low_priority {
                player.play(animations.get(helper.current, facing));
            } else {
                // Nothing is low priority after first loop
                helper.low_priority = true;
            }
        }
    }
}
