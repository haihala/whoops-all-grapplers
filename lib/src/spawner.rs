use bevy::prelude::*;
use bevy::utils::HashMap;

use player_state::{PlayerState, StateEvent};
use types::{Hitbox, MoveId};

use crate::assets::Colors;
use crate::clock::{Clock, ROUND_TIME};
use crate::physics::ConstantVelocity;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(handle_hitbox_events.system())
            .add_system(handle_requests.system());
    }
}

struct SpawnRequest {
    id: MoveId,
    hitbox: Hitbox,
    attached_to_player: bool,
    speed: f32,
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
        self.spawn_requests.push(SpawnRequest {
            id,
            hitbox,
            attached_to_player: true,
            speed: 0.0,
        });
        self.despawn_requests.push(DespawnRequest {
            id,
            frame: time_of_death,
        });
    }

    fn add_projectile(&mut self, hitbox: Hitbox, id: MoveId, time_of_death: usize, speed: f32) {
        self.spawn_requests.push(SpawnRequest {
            id,
            hitbox,
            attached_to_player: false,
            speed,
        });
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
        parent_position: Vec3,
        frame: usize,
    ) {
        for request in self.spawn_requests.drain(..) {
            let offset = request.hitbox.get_offset(flipped);
            let translation = if request.attached_to_player {
                offset
            } else {
                parent_position + offset
            };

            let spawned_box = commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation,
                        ..Default::default()
                    },
                    material: colors.hurtbox.clone(),
                    sprite: Sprite::new(request.hitbox.size),
                    ..Default::default()
                })
                .insert(request.hitbox.to_owned())
                .insert(ConstantVelocity::new(request.speed, flipped))
                .id();

            if request.attached_to_player {
                commands.entity(parent).push_children(&[spawned_box]);
            }

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

pub fn handle_hitbox_events(clock: Res<Clock>, mut query: Query<(&mut Spawner, &mut PlayerState)>) {
    for (mut spawner, mut state) in query.iter_mut() {
        for event in state.get_events() {
            if let StateEvent::Hitbox {
                hitbox,
                move_id,
                ttl,
            } = event
            {
                spawner.add_hitbox(hitbox, move_id, ttl + clock.frame);
                state.consume_event(event);
            } else if let StateEvent::Projectile {
                hitbox,
                move_id,
                ttl,
                speed,
            } = event
            {
                spawner.add_projectile(
                    hitbox,
                    move_id,
                    ttl.map(|frame| frame + clock.frame)
                        // If not specified, despawn after a round. Despawned early if moved too far
                        .unwrap_or((ROUND_TIME * constants::FPS) as usize),
                    speed,
                );
                state.consume_event(event);
            }
        }
    }
}

pub fn handle_requests(
    mut commands: Commands,
    clock: Res<Clock>,
    colors: Res<Colors>,
    mut hitboxes: Query<(Entity, &Transform, &mut Spawner, &PlayerState)>,
) {
    for (parent, tf, mut hitboxes, state) in hitboxes.iter_mut() {
        hitboxes.handle_requests(
            &mut commands,
            &colors,
            state.flipped(),
            parent,
            tf.translation,
            clock.frame,
        );
    }
}
