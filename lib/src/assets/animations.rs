use bevy::{prelude::*, utils::HashMap};
use types::{Animation, DummyAnimation};

#[derive(Deref, DerefMut)]
pub struct Animations(pub HashMap<Animation, Handle<AnimationClip>>);

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

pub fn update_animation(
    animations: Res<Animations>,
    mut main: Query<&mut AnimationHelper>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for mut helper in main.iter_mut() {
        if let Some(next) = helper.next {
            let mut player = players.get_mut(helper.player_entity).unwrap();
            let asset = animations[&next].clone();
            player.play(asset).repeat();
            helper.current = next;
        }
    }
}

#[derive(Debug, Component)]
pub struct AnimationHelperSetup;

pub fn setup_helpers(
    mut commands: Commands,
    to_setup: Query<(Entity, &AnimationHelperSetup)>,
    children: Query<&Children>,
    players: Query<&AnimationPlayer>,
) {
    for (entity, _) in to_setup.iter() {
        if let Some(player_entity) = find_animation_player_entity(entity, &children, &players) {
            let mut e = commands.entity(entity);
            e.remove::<AnimationHelperSetup>();
            e.insert(AnimationHelper::new(player_entity));
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
    vec![(
        Animation::Dummy(DummyAnimation::Idle),
        "dummy-character.glb#Animation0",
    )]
    .into_iter()
    .collect()
}
