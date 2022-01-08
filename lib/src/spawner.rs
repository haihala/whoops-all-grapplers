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
        app.add_system(spawn_hitboxes.system())
            .add_system(update_hitboxes.system())
            .add_system(register_hits.system());
    }
}

struct TimedEvent {
    id: MoveId,
    frame: Option<usize>,
}
impl TimedEvent {
    fn now(id: MoveId) -> Self {
        Self { id, frame: None }
    }
    fn later(id: MoveId, frame: usize) -> Self {
        Self {
            id,
            frame: Some(frame),
        }
    }
}

#[derive(Default)]
pub struct Spawner {
    registered_hitboxes: HashMap<MoveId, Hitbox>,

    spawn_requests: Vec<TimedEvent>,
    spawned: HashMap<MoveId, Entity>,
    despawn_requests: Vec<TimedEvent>,
}
impl Spawner {
    pub fn load(target: HashMap<MoveId, Hitbox>, player: Player) -> Spawner {
        Spawner {
            registered_hitboxes: target
                .into_iter()
                .map(|(id, mut hitbox)| {
                    hitbox.owner = Some(player);
                    (id, hitbox)
                })
                .collect(),
            ..Default::default()
        }
    }

    fn add_hitbox(&mut self, id: MoveId, time_of_death: usize) {
        self.spawn_requests.push(TimedEvent::now(id));
        self.despawn_requests
            .push(TimedEvent::later(id, time_of_death));
    }

    fn handle_requests(
        &mut self,
        commands: &mut Commands,
        colors: &Res<Colors>,
        flipped: bool,
        parent: Entity,
        frame: usize,
    ) {
        for id in self
            .spawn_requests
            .drain_filter(|event| (event.frame.is_none() || event.frame.unwrap() <= frame))
            .map(|event| event.id)
            .collect::<Vec<MoveId>>()
            .into_iter()
        {
            self.spawn_box(commands, colors, id, flipped, parent);
        }

        for id in self
            .despawn_requests
            .drain_filter(|event| (event.frame.is_none() || event.frame.unwrap() <= frame))
            .map(|event| event.id)
            .collect::<Vec<MoveId>>()
            .iter()
        {
            self.despawn_entity(commands, id);
        }
    }

    fn spawn_box(
        &mut self,
        commands: &mut Commands,
        colors: &Res<Colors>,
        id: MoveId,
        flipped: bool,
        parent: Entity,
    ) {
        let hitbox = self.registered_hitboxes.get(&id).unwrap();

        let spawned_box = commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: hitbox.get_offset(flipped),
                    ..Default::default()
                },
                material: colors.hurtbox.clone(),
                sprite: Sprite::new(hitbox.size),
                ..Default::default()
            })
            .insert(hitbox.to_owned())
            .id();

        commands.entity(parent).push_children(&[spawned_box]);
        self.spawned.insert(id, spawned_box);
    }

    fn despawn_entity(&mut self, commands: &mut Commands, id: &MoveId) {
        if let Some(spawned) = self.spawned.get(id) {
            commands.entity(*spawned).despawn();
            self.spawned.remove(id);
        }
    }
}

pub fn spawn_hitboxes(clock: Res<Clock>, mut hitboxes: Query<(&mut Spawner, &mut PlayerState)>) {
    for (mut hitboxes, mut state) in hitboxes.iter_mut() {
        for event in state.get_events() {
            if let StateEvent::Hitbox { move_id, ttl } = event {
                hitboxes.add_hitbox(move_id, ttl + clock.frame);

                state.consume_event(event);
            }
        }
    }
}

pub fn update_hitboxes(
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
