use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct AnimationRequest {
    pub animation: Handle<AnimationClip>,
    pub looping: bool,
}

pub fn animation_starter(
    mut commands: Commands,
    requests: Query<(Entity, &AnimationRequest)>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for (master, request) in requests.iter() {
        let mut player = find_player(master, &children, &mut players).unwrap();
        player.play(request.animation.clone());
        if request.looping {
            player.repeat();
        }

        commands.entity(master).remove::<AnimationRequest>();
    }
}

fn find_player<'a>(
    parent: Entity,
    children: &Query<&Children>,
    players: &'a mut Query<&mut AnimationPlayer>,
) -> Option<Mut<'a, AnimationPlayer>> {
    // NGL this shit makes me want to puke, but it ought to safely and recursively find an AnimationPlayer under a parent if one exists
    if let Ok(candidates) = children.get(parent) {
        let mut next_candidates: Vec<Entity> = candidates.iter().map(|e| e.to_owned()).collect();
        while !next_candidates.is_empty() {
            for candidate in next_candidates.drain(..).collect::<Vec<Entity>>() {
                if players.get(candidate).is_ok() {
                    return Some(players.get_mut(candidate).unwrap());
                } else {
                    next_candidates.extend(children.get(candidate).unwrap().iter());
                }
            }
        }
    }
    None
}
