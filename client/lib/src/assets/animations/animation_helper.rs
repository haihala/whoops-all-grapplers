use bevy::prelude::*;

use core::{Animation, Facing};

use super::Animations;

#[derive(Debug, Component)]
pub struct AnimationHelper {
    pub player_entity: Entity,
    pub current: Animation,
    facing: Facing,
    next: Option<(Animation, usize)>,
}
impl AnimationHelper {
    fn new(player_entity: Entity) -> AnimationHelper {
        AnimationHelper {
            player_entity,
            current: Animation::TPose,
            facing: Facing::default(),
            next: None,
        }
    }
    pub fn play(&mut self, new: Animation) {
        self.play_with_offset(new, 0);
    }

    pub fn play_with_offset(&mut self, new: Animation, offset: usize) {
        self.next = if new != self.current {
            Some((new, offset))
        } else {
            None
        }
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
    animations: Res<Animations>,
    mut main: Query<(&mut AnimationHelper, &Facing)>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for (mut helper, facing) in &mut main {
        let mut player = players.get_mut(helper.player_entity).unwrap();
        if let Some((animation, offset)) = helper.next.take() {
            let asset = animations.get(animation, facing);
            player
                .play(asset)
                .set_elapsed(offset as f32 * core::FPS)
                .repeat();
            helper.set_playing(animation, *facing);
        } else if *facing != helper.facing {
            let asset = animations.get(helper.current, facing);
            let elapsed = player.elapsed();
            player.play(asset).set_elapsed(elapsed).repeat();
            let current = helper.current;
            helper.set_playing(current, *facing);
        }
    }
}
