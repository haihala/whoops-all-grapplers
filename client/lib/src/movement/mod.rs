mod followers;
mod player_velocity;
mod stick_movement;

use core::f32;

pub use followers::Follow;
pub use player_velocity::PlayerVelocity;

use bevy::prelude::*;

use player_state::PlayerState;
use wag_core::{
    Area, Clock, Combo, Facing, Player, Players, RollbackSchedule, Stats, StatusFlag, WAGStage,
};

use crate::{
    damage::{HitTracker, HitboxSpawner},
    event_spreading::{AddMovement, ClearMovement},
};

pub const GROUND_PLANE_HEIGHT: f32 = 0.0;
pub const ARENA_WIDTH: f32 = 8.5;
pub const MAX_PLAYER_DISTANCE: f32 = 8.0;

#[derive(Debug, Default, Reflect, Component, Clone, Copy)]
pub struct ConstantVelocity {
    pub shift: Vec3,
    pub speed: Vec3,
}
impl ConstantVelocity {
    pub fn new(speed: Vec3) -> ConstantVelocity {
        ConstantVelocity {
            speed,
            shift: speed / wag_core::FPS,
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
                resolve_constraints,
                stick_movement::movement_input,
                move_constants,
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
fn player_gravity(
    clock: Res<Clock>,
    mut players: Query<(
        &mut PlayerVelocity,
        &mut PlayerState,
        &mut HitboxSpawner,
        &mut Transform,
        &Stats,
        Option<&Combo>,
    )>,
) {
    for (mut velocity, mut state, mut spawner, mut tf, stats, combo) in &mut players {
        if state.has_flag(StatusFlag::MovementLock) {
            continue;
        }

        let on_floor = tf.translation.y <= GROUND_PLANE_HEIGHT;

        if on_floor {
            if !state.is_grounded() && velocity.get_shift().y <= 0.0 {
                // Velocity check ensures that we don't call land on the frame we're being launched
                state.land(clock.frame);
                spawner.despawn_on_landing();
                velocity.y_collision();
                tf.translation.y = GROUND_PLANE_HEIGHT;
            }
        } else {
            velocity.add_impulse(
                -Vec2::Y
                    * (stats.gravity
                        + stats.gravity_scaling * combo.map_or(0.0, |c| c.hits as f32)),
            );

            if state.is_grounded() {
                state.jump();
            }
        }
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

fn player_input(
    clock: Res<Clock>,
    mut query: Query<(&PlayerState, &mut PlayerVelocity, &Stats, &Facing)>,
) {
    for (state, mut velocity, status_effects, facing) in &mut query {
        if state.has_flag(StatusFlag::MovementLock) {
            continue;
        }

        if let Some(walk_direction) = state.get_walk_direction() {
            velocity.handle_walking_velocity(status_effects.walk_speed, *facing, walk_direction);
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

        let np = tf.translation.truncate() + velocity.get_shift();
        velocity.next_pos = np;
    }
}

fn resolve_constraints(
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
                    dbg!(v1, v2);
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

fn move_constants(
    mut commands: Commands,
    clock: Res<Clock>,
    mut query: Query<(
        Entity,
        &ConstantVelocity,
        Option<&HitTracker>,
        &mut Transform,
    )>,
) {
    for (entity, velocity, hit_tracker, mut transform) in &mut query {
        if hit_tracker
            .map(|tracker| !tracker.active(clock.frame))
            .unwrap_or(false)
        {
            continue;
        }

        transform.translation += velocity.shift;

        // Despawn the thing if it's outside of the arena
        if transform.translation.length() > ARENA_WIDTH + 10.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
