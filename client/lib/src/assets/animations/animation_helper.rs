use bevy::{prelude::*, scene::SceneInstance};

use wag_core::{Animation, Facing};

use super::Animations;

#[derive(Debug, Default, Clone, Copy)]
pub struct AnimationRequest {
    pub animation: Animation,
    pub time_offset: usize,
    pub position_offset: Vec2,
    pub invert: bool,
    pub looping: bool,
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
    pub current: Animation,
    facing: Facing,
    request: Option<AnimationRequest>,
}
impl AnimationHelper {
    fn new(player_entity: Entity, scene_root: Entity) -> AnimationHelper {
        AnimationHelper {
            player_entity,
            scene_root,
            current: Animation::TPose,
            facing: Facing::default(),
            request: None,
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
            for candidate in next_candidates.drain(..).collect::<Vec<Entity>>() {
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
    mut main: Query<(&mut AnimationHelper, &Facing)>,
    mut players: Query<&mut AnimationPlayer>,
    mut scenes: Query<&mut Transform, With<Handle<Scene>>>,
) {
    for (mut helper, facing) in &mut main {
        let mut player = players.get_mut(helper.player_entity).unwrap();
        let mut scene_root = scenes.get_mut(helper.scene_root).unwrap();

        let handle = animations.get(helper.current, facing);
        if let Some(request) = helper.request.take() {
            // New animation set to start
            player
                .start(animations.get(
                    request.animation,
                    &if request.invert {
                        facing.opposite()
                    } else {
                        *facing
                    },
                ))
                .set_elapsed(request.time_offset as f32 * wag_core::FPS);

            if request.looping {
                player.repeat();
            }

            helper.set_playing(request.animation, *facing);
            scene_root.translation = request.position_offset.extend(0.0);
        } else if *facing != helper.facing {
            // Sideswitch
            let elapsed = player.elapsed();
            player.play(handle).set_elapsed(elapsed).repeat();
            let current = helper.current;
            helper.set_playing(current, *facing);
        }
    }
}
