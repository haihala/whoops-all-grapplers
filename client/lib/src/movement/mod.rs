mod followers;
mod player_velocity;
mod stick_movement;

use core::f32;

pub use followers::Follow;
pub use player_velocity::PlayerVelocity;

use bevy::prelude::*;

use characters::ActionEvent;
use player_state::PlayerState;
use wag_core::{Area, Clock, Facing, Players, RollbackSchedule, Stats, WAGStage};

use crate::{
    camera::{CameraWrapper, VIEWPORT_HALFWIDTH},
    damage::{Combo, HitTracker, HitboxSpawner},
};

pub const GROUND_PLANE_HEIGHT: f32 = 0.0;
pub const ARENA_WIDTH: f32 = 9.5;

#[derive(Debug, Default, Reflect, Component)]
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
                stick_movement::movement_input,
                player_gravity,
                player_input,
                set_target_position,
                resolve_constraints,
                move_constants,
                followers::update_followers,
            )
                .chain()
                .in_set(WAGStage::Physics),
        );
    }
}

const CAMERA_EDGE_COLLISION_PADDING: f32 = 0.5;
fn update_walls(mut walls: ResMut<Walls>, cam_query: Query<&Transform, With<CameraWrapper>>) {
    let camera_x = cam_query.single().translation.x;
    walls.left = camera_x - VIEWPORT_HALFWIDTH + CAMERA_EDGE_COLLISION_PADDING;
    walls.right = camera_x + VIEWPORT_HALFWIDTH - CAMERA_EDGE_COLLISION_PADDING;
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
        if state.active_cinematic().is_some() {
            continue;
        }

        velocity.on_floor = tf.translation.y <= GROUND_PLANE_HEIGHT;

        if velocity.on_floor {
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

fn player_input(
    clock: Res<Clock>,
    mut query: Query<(&mut PlayerState, &mut PlayerVelocity, &Stats, &Facing)>,
) {
    for (mut state, mut velocity, status_effects, facing) in &mut query {
        if state.active_cinematic().is_some() {
            continue;
        }

        for _ in state.drain_matching_actions(|action| {
            if ActionEvent::ClearMovement == *action {
                Some(())
            } else {
                None
            }
        }) {
            velocity.clear_movements();
        }

        for movement in state.drain_matching_actions(|action| {
            if let ActionEvent::Movement(movement) = action {
                Some(movement.to_owned())
            } else {
                None
            }
        }) {
            velocity.handle_movement(clock.frame, *facing, movement);
        }

        if let Some(walk_direction) = state.get_walk_direction() {
            velocity.handle_walking_velocity(status_effects.walk_speed, *facing, walk_direction);
        } else if state.is_grounded() {
            velocity.drag();
        }

        velocity.cleanup_movements(clock.frame);
        velocity.apply_movements();
    }
}

fn set_target_position(mut query: Query<(&Transform, &PlayerState, &mut PlayerVelocity)>) {
    for (tf, state, mut velocity) in &mut query {
        velocity.next_pos = tf.translation.truncate();
        if state.active_cinematic().is_some() {
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
                std::cmp::Ordering::Equal => panic!("Perfectly identical next pos"),
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

    let left_wall = walls.left + pushbox_left.width() / 2.0;
    let left_wall_collision = velocity_left.next_pos.x - shift < left_wall;

    let right_wall = walls.right - pushbox_right.width() / 2.0;
    let right_wall_collision = velocity_right.next_pos.x + shift > right_wall;

    if left_wall_collision {
        velocity_left.next_pos.x = left_wall;
        velocity_left.x_collision();

        let pseudo_wall = left_wall + pushbox_left.width() / 2.0 + pushbox_right.width() / 2.0;
        velocity_right.next_pos.x = (velocity_right.next_pos.x + shift).max(pseudo_wall);
    } else if right_wall_collision {
        velocity_right.next_pos.x = right_wall;
        velocity_right.x_collision();

        let pseudo_wall = right_wall - pushbox_left.width() / 2.0 - pushbox_right.width() / 2.0;
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
