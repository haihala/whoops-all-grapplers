use bevy::prelude::*;
use bevy::utils::HashMap;

use player_state::{PlayerState, StateEvent};
use types::{Hitbox, Hurtbox, MoveId, Player};

use crate::clock::Clock;
use crate::damage::Health;
use crate::{assets::Colors, physics::rect_collision};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(handle_hitbox_events.system())
            .add_system(handle_requests.system())
            .add_system(register_hits.system());
    }
}

struct SpawnRequest {
    id: MoveId,
    hitbox: Hitbox,
}

struct DespawnRequest {
    id: MoveId,
    frame: usize,
}

#[derive(Default)]
pub struct Spawner {
    spawn_requests: Vec<SpawnRequest>,
    spawned: HashMap<MoveId, Entity>,
    despawn_requests: Vec<DespawnRequest>,
}
impl Spawner {
    fn add_hitbox(&mut self, hitbox: Hitbox, id: MoveId, time_of_death: usize) {
        self.spawn_requests.push(SpawnRequest { id, hitbox });
        self.despawn_requests.push(DespawnRequest {
            id,
            frame: time_of_death,
        });
    }

    fn handle_requests(
        &mut self,
        commands: &mut Commands,
        colors: &Res<Colors>,
        flipped: bool,
        parent: Entity,
        frame: usize,
    ) {
        for request in self.spawn_requests.drain(..) {
            let spawned_box = commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: request.hitbox.get_offset(flipped),
                        ..Default::default()
                    },
                    material: colors.hurtbox.clone(),
                    sprite: Sprite::new(request.hitbox.size),
                    ..Default::default()
                })
                .insert(request.hitbox.to_owned())
                .id();

            commands.entity(parent).push_children(&[spawned_box]);
            self.spawned.insert(request.id, spawned_box);
        }

        for id in self
            .despawn_requests
            .drain_filter(|event| (event.frame <= frame))
            .map(|event| event.id)
        {
            if let Some(spawned) = self.spawned.get(&id) {
                commands.entity(*spawned).despawn();
                self.spawned.remove(&id);
            }
        }
    }
}

pub fn handle_hitbox_events(
    clock: Res<Clock>,
    mut hitboxes: Query<(&mut Spawner, &mut PlayerState)>,
) {
    for (mut hitboxes, mut state) in hitboxes.iter_mut() {
        for event in state.get_events() {
            if let StateEvent::Hitbox {
                hitbox,
                move_id,
                ttl,
            } = event
            {
                hitboxes.add_hitbox(hitbox, move_id, ttl + clock.frame);

                state.consume_event(event);
            }
        }
    }
}

pub fn handle_requests(
    mut commands: Commands,
    clock: Res<Clock>,
    colors: Res<Colors>,
    mut hitboxes: Query<(Entity, &mut Spawner, &PlayerState)>,
) {
    for (parent, mut hitboxes, state) in hitboxes.iter_mut() {
        hitboxes.handle_requests(&mut commands, &colors, state.flipped(), parent, clock.frame);
    }
}

pub fn register_hits(
    mut commands: Commands,
    mut hitboxes: Query<(Entity, &Hitbox, &GlobalTransform)>,
    mut hurtboxes: Query<(&Hurtbox, &GlobalTransform, &mut Health, &Player)>,
) {
    for (entity, hitbox, tf1) in hitboxes.iter_mut() {
        for (hurtbox, tf2, mut health, defending_player) in hurtboxes.iter_mut() {
            if hitbox.owner.unwrap() == *defending_player {
                // You can't hit yourself
                // If a hitbox active is false, it already hit and can't do so again
                continue;
            }

            if rect_collision(
                tf2.translation + hurtbox.offset,
                hurtbox.size,
                tf1.translation,
                hitbox.size,
            ) {
                health.hit(hitbox.hit);
                commands.entity(entity).despawn()
            }
        }
    }
}
