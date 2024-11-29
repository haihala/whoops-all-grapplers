mod followers;
mod player_velocity;
mod stick_movement;

use std::f32::consts::PI;

pub use followers::Follow;
pub use player_velocity::PlayerVelocity;

use bevy::prelude::*;

use player_state::PlayerState;
use wag_core::{
    Area, Clock, Combo, Facing, Player, Players, RollbackSchedule, Stats, StatusFlag, WAGStage, FPS,
};

use crate::{
    damage::{HitTracker, HitboxSpawner},
    event_spreading::{AddMovement, ClearMovement, TeleportEvent},
};

pub const GROUND_PLANE_HEIGHT: f32 = 0.0;
pub const ARENA_WIDTH: f32 = 8.5;
pub const MAX_PLAYER_DISTANCE: f32 = 8.0;

#[derive(Debug, Default, Reflect, Component, Clone, Copy)]
pub struct ObjectVelocity {
    pub speed: Vec3,
    pub acceleration: Vec3,
    pub face_forward: bool,
}
impl ObjectVelocity {
    pub fn new(speed: Vec3, gravity: f32) -> ObjectVelocity {
        ObjectVelocity {
            speed,
            acceleration: -Vec3::Y * gravity,
            face_forward: true,
        }
    }
}

#[derive(Debug, Default, Reflect, Component, Deref, DerefMut, Clone, Copy)]
pub struct Pushbox(pub Area);

#[derive(Debug, Reflect, Resource, Clone, Copy)]
pub struct Walls {
    left: f32,
    right: f32,
}

impl Default for Walls {
    fn default() -> Self {
        Self {
            left: -f32::INFINITY,
            right: f32::INFINITY,
        }
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Walls>().add_systems(
            RollbackSchedule,
            (
                update_walls,
                player_gravity,
                player_input,
                set_target_position,
                resolve_floor,
                resolve_x_constraints,
                stick_movement::movement_input,
                move_objects,
                followers::update_followers,
            )
                .chain()
                .in_set(WAGStage::Physics),
        );
    }
}

const CAMERA_EDGE_COLLISION_PADDING: f32 = 0.5;
fn update_walls(mut walls: ResMut<Walls>, player_query: Query<&Transform, With<Player>>) {
    let mut xs: Vec<_> = player_query
        .into_iter()
        .map(|tf| tf.translation.x)
        .collect();
    xs.sort_by(|a, b| a.total_cmp(b));
    let [left, right] = xs[..] else { panic!() };
    walls.left = (right - MAX_PLAYER_DISTANCE + CAMERA_EDGE_COLLISION_PADDING).max(-ARENA_WIDTH);
    walls.right = (left + MAX_PLAYER_DISTANCE - CAMERA_EDGE_COLLISION_PADDING).min(ARENA_WIDTH);
}

#[allow(clippy::type_complexity)]
fn player_gravity(mut players: Query<(&mut PlayerVelocity, &PlayerState, &Stats, Option<&Combo>)>) {
    for (mut velocity, state, stats, combo) in &mut players {
        if state.has_flag(StatusFlag::MovementLock) {
            continue;
        }

        if state.is_grounded() {
            continue;
        }

        velocity.add_impulse(
            -Vec2::Y
                * (stats.gravity + stats.gravity_scaling * combo.map_or(0.0, |c| c.hits as f32)),
        );
    }
}

pub fn clear_movement(trigger: Trigger<ClearMovement>, mut query: Query<&mut PlayerVelocity>) {
    let mut vel = query.get_mut(trigger.entity()).unwrap();
    vel.clear_movements();
}

pub fn add_movement(
    trigger: Trigger<AddMovement>,
    clock: Res<Clock>,
    mut query: Query<(&mut PlayerVelocity, &Facing)>,
) {
    let (mut vel, facing) = query.get_mut(trigger.entity()).unwrap();
    vel.handle_movement(clock.frame, *facing, trigger.event().0);
}

pub fn handle_teleports(
    trigger: Trigger<TeleportEvent>,
    mut query: Query<(&mut PlayerVelocity, &Facing)>,
) {
    let (mut vel, facing) = query.get_mut(trigger.entity()).unwrap();
    vel.teleport = Some(facing.mirror_vec2(trigger.event().0));
}

fn player_input(
    clock: Res<Clock>,
    mut query: Query<(&PlayerState, &mut PlayerVelocity, &Stats, &Facing, &Stats)>,
) {
    for (state, mut velocity, status_effects, facing, stats) in &mut query {
        if state.has_flag(StatusFlag::MovementLock) {
            continue;
        }

        if let Some(walk_direction) = state.get_walk_direction() {
            velocity.handle_walking_velocity(
                status_effects.walk_speed,
                *facing,
                walk_direction,
                stats,
            );
        } else if state.is_grounded() {
            velocity.drag();
        }

        velocity.apply_movements();
        velocity.cleanup_movements(clock.frame);
    }
}

fn set_target_position(mut query: Query<(&Transform, &PlayerState, &mut PlayerVelocity)>) {
    for (tf, state, mut velocity) in &mut query {
        velocity.next_pos = tf.translation.truncate();
        if state.has_flag(StatusFlag::MovementLock) {
            continue;
        }

        let np = tf.translation.truncate() + velocity.get_shift() + velocity.get_teleport();
        velocity.next_pos = np;
    }
}

#[allow(clippy::type_complexity)]
fn resolve_floor(
    clock: Res<Clock>,
    mut players: Query<(
        &mut PlayerVelocity,
        &mut PlayerState,
        &mut HitboxSpawner,
        &mut Transform,
        &Pushbox,
    )>,
) {
    for (mut velocity, mut state, mut spawner, tf, pushbox) in &mut players {
        let on_floor =
            pushbox.with_center(tf.translation.truncate()).bottom() <= GROUND_PLANE_HEIGHT;

        let just_landed = on_floor && !state.is_grounded() && velocity.get_shift().y <= 0.0;
        if just_landed {
            // Velocity check ensures that we don't call land on the frame we're being launched
            state.land(clock.frame);
            spawner.despawn_on_landing();
            velocity.y_collision();
            velocity.next_pos.y = GROUND_PLANE_HEIGHT;
        }

        if !on_floor && state.is_grounded() {
            error!("Not at ground level, state is not airborne");
        }
    }
}

fn resolve_x_constraints(
    players: Res<Players>,
    mut player_query: Query<(&mut PlayerVelocity, &mut Transform, &Pushbox)>,
    walls: Res<Walls>,
) {
    let mut components = player_query
        .get_many_mut([players.one, players.two])
        .unwrap();

    // Sort primarily by x
    // If on top of each other (corner), bottom one is "closer" to the corner
    components.sort_by(
        |(v1, _, _), (v2, _, _)| match v1.next_pos.x.total_cmp(&v2.next_pos.x) {
            std::cmp::Ordering::Equal => match v1.next_pos.y.total_cmp(&v2.next_pos.y) {
                std::cmp::Ordering::Less => {
                    if v1.next_pos.x < 0.0 {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                }
                std::cmp::Ordering::Greater => {
                    if v1.next_pos.x < 0.0 {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                }
                std::cmp::Ordering::Equal => {
                    // This could be due to a NaN
                    error!(
                        "Player positions are perfectly equal: {:?} == {:?} ",
                        v1, v2
                    );
                    panic!("Perfectly identical next pos");
                }
            },
            other => other,
        },
    );

    let [(mut velocity_left, mut tf_left, pb_left), (mut velocity_right, mut tf_right, pb_right)] =
        components;

    let pushbox_left = pb_left.with_center(velocity_left.next_pos);
    let pushbox_right = pb_right.with_center(velocity_right.next_pos);

    let shift = if let Some(overlap) = pushbox_left.intersection(&pushbox_right) {
        overlap.width() / 2.0
    } else {
        0.0
    };

    let left_wall = walls.left + (pushbox_left.width() / 2.0);
    let left_wall_collision = velocity_left.next_pos.x - shift < left_wall;

    let right_wall = walls.right - (pushbox_right.width() / 2.0);
    let right_wall_collision = velocity_right.next_pos.x + shift > right_wall;

    let half_widths = pushbox_left.width() / 2.0 + pushbox_right.width() / 2.0;

    if left_wall_collision && right_wall_collision {
        return;
    } else if left_wall_collision && !right_wall_collision {
        velocity_left.next_pos.x = left_wall;
        velocity_left.x_collision();

        let pseudo_wall = left_wall + half_widths;
        velocity_right.next_pos.x = (velocity_right.next_pos.x + shift).max(pseudo_wall);
    } else if right_wall_collision && !left_wall_collision {
        velocity_right.next_pos.x = right_wall;
        velocity_right.x_collision();

        let pseudo_wall = right_wall - half_widths;
        velocity_left.next_pos.x = (velocity_left.next_pos.x - shift).min(pseudo_wall);
    } else {
        // No wall
        velocity_left.next_pos.x -= shift;
        velocity_right.next_pos.x += shift;
    }

    tf_left.translation = velocity_left.next_pos.extend(0.0);
    tf_right.translation = velocity_right.next_pos.extend(0.0);
}

#[allow(clippy::type_complexity)]
fn move_objects(
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<(
        Entity,
        &mut ObjectVelocity,
        Option<&HitTracker>,
        &mut Transform,
        Option<&mut Follow>,
    )>,
) {
    for (entity, mut velocity, hit_tracker, mut transform, mut maybe_follow) in &mut query {
        if hit_tracker
            .map(|tracker| !tracker.active(clock.frame))
            .unwrap_or(false)
        {
            continue;
        }

        let acceleration = velocity.acceleration / FPS;
        velocity.speed += acceleration;
        let shift = velocity.speed / FPS;
        if let Some(ref mut follow) = maybe_follow {
            follow.offset += shift;
        } else {
            transform.translation += shift;
        }

        if velocity.face_forward {
            transform.look_to(velocity.speed.normalize(), Vec3::Z);
            transform.rotate_z(PI / 2.0);
        }

        // Despawn the thing if it's outside of the arena or under the floor
        if transform.translation.x.abs() > ARENA_WIDTH
            || transform.translation.y < GROUND_PLANE_HEIGHT
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}
