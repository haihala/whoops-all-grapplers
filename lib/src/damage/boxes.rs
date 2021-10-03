use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use input_parsing::InputReader;
use uuid::Uuid;

use crate::physics::rect_collision;
use crate::{Colors, Player};

use super::Health;

#[derive(Clone, Copy)]
pub struct Hurtbox {
    offset: Vec3,
    size: Vec2,
}
impl Hurtbox {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            offset: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Hitbox {
    offset: Vec3,
    size: Vec2,
    on_hit_damage: Option<f32>,
    owner: Option<Player>,
}
impl Hitbox {
    fn get_offset(&self, flipped: bool) -> Vec3 {
        if flipped {
            Vec3::new(-self.offset.x, self.offset.y, self.offset.z)
        } else {
            self.offset
        }
    }

    pub fn new(offset: Vec2, size: Vec2, damage: Option<f32>) -> Self {
        Self {
            offset: offset.extend(0.0),
            size,
            on_hit_damage: damage,
            owner: None,
        }
    }
}

#[derive(Default)]
pub struct HitboxManager {
    registered: HashMap<Uuid, Hitbox>,

    spawn_requests: HashSet<Uuid>,
    spawned: HashMap<Uuid, Entity>,
    despawn_requests: HashSet<Uuid>,
}
impl HitboxManager {
    pub fn register(&mut self, id: Uuid, hurtbox: Hitbox) {
        self.registered.insert(id, hurtbox);
    }

    pub fn spawn(&mut self, id: Uuid) {
        // Tell the system that a box has been requested
        self.spawn_requests.insert(id);
    }
    pub fn despawn(&mut self, id: Uuid) {
        // Tell the system that a box has been requested to not exist anymore
        self.despawn_requests.insert(id);
    }

    fn handle_requests(
        &mut self,
        commands: &mut Commands,
        colors: &Res<Colors>,
        flipped: bool,
        player: Player,
        parent: Entity,
    ) {
        let spawn_requests: HashSet<Uuid> = self.spawn_requests.drain().collect();
        for id in spawn_requests {
            self.spawn_box(commands, colors, id, flipped, player, parent);
        }

        let despawn_requests: HashSet<Uuid> = self.despawn_requests.drain().collect();
        for id in despawn_requests {
            self.despawn_box(commands, id);
        }
    }

    fn spawn_box(
        &mut self,
        commands: &mut Commands,
        colors: &Res<Colors>,
        id: Uuid,
        flipped: bool,
        player: Player,
        parent: Entity,
    ) {
        let hitbox = self.registered.get_mut(&id).unwrap();
        hitbox.owner = Some(player);

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
            .insert(*hitbox)
            .id();

        commands.entity(parent).push_children(&[spawned_box]);
        self.spawned.insert(id, spawned_box);
        self.spawn_requests.remove(&id);
    }

    fn despawn_box(&mut self, commands: &mut Commands, id: Uuid) {
        commands.entity(*self.spawned.get(&id).unwrap()).despawn();

        self.spawned.remove(&id);
        self.despawn_requests.remove(&id);
    }
}

pub fn hurtbox_manager(
    mut commands: Commands,
    colors: Res<Colors>,
    mut hitboxes: Query<(Entity, &mut HitboxManager, &InputReader, &Player)>,
) {
    for (parent, mut hitboxes, reader, attacking_player) in hitboxes.iter_mut() {
        hitboxes.handle_requests(
            &mut commands,
            &colors,
            reader.flipped,
            *attacking_player,
            parent,
        );
    }
}

pub fn handle_hits(
    mut hitboxes: Query<(&mut Hitbox, &GlobalTransform)>,
    mut hurtboxes: Query<(&Hurtbox, &GlobalTransform, &mut Health, &Player)>,
) {
    for (mut hitbox, tf1) in hitboxes.iter_mut() {
        for (hurtbox, tf2, mut health, defending_player) in hurtboxes.iter_mut() {
            if hitbox.owner.is_none() || hitbox.owner.unwrap() == *defending_player {
                // You can't hit yourself
                // If a hitbox owner is None, it already hit and can't do so again
                continue;
            }

            if rect_collision(
                tf2.translation + hurtbox.offset,
                hurtbox.size,
                tf1.translation,
                hitbox.size,
            ) {
                if let Some(amount) = hitbox.on_hit_damage {
                    health.hurt(amount);
                    hitbox.owner = None;
                }
            } else {
                dbg!(
                    tf1.translation,
                    hitbox.size,
                    tf2.translation + hurtbox.offset,
                    hurtbox.size,
                );
            }
        }
    }
}
