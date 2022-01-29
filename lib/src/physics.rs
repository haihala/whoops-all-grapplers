use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use constants::PLAYER_GRAVITY_PER_FRAME;
use moves::MoveMobility;
use player_state::PlayerState;
use types::{LRDirection, MoveId, Player};

use crate::{
    camera::{WorldCamera, VIEWPORT_WIDTH},
    clock::run_max_once_per_combat_frame,
};

pub const GROUND_PLANE_HEIGHT: f32 = -0.4;
pub const ARENA_WIDTH: f32 = 10.0;

pub struct ConstantVelocity {
    pub shift: Vec3,
    pub speed: Vec3,
}
impl ConstantVelocity {
    pub fn new(speed: Vec3) -> ConstantVelocity {
        ConstantVelocity {
            speed,
            shift: speed / constants::FPS,
        }
    }
}

#[derive(Debug, Inspectable, Clone, Default, Copy)]
pub struct CurrentMove {
    id: MoveId,
    base_velocity: Vec3,
}
#[derive(Debug, Inspectable, Clone, Default, Copy)]
pub struct PlayerVelocity {
    velocity: Vec3,
    current_move: Option<CurrentMove>,
}

impl PlayerVelocity {
    pub fn get_shift(&self) -> Vec3 {
        self.velocity / constants::FPS
    }
    pub fn add_impulse(&mut self, impulse: Vec3) {
        self.velocity += impulse;
    }
    pub fn drag(&mut self) {
        self.velocity = Vec3::new(
            if self.velocity.x.abs() > constants::DRAG {
                self.velocity.x.signum() * (self.velocity.x.abs() - constants::DRAG)
            } else {
                self.current_move = None;
                0.0
            },
            self.velocity.y,
            0.0,
        );
    }
    fn handle_move_velocity(
        &mut self,
        move_id: MoveId,
        mobility: MoveMobility,
        facing: &LRDirection,
    ) {
        match mobility {
            MoveMobility::Impulse(amount) => {
                self.handle_move_velocity_chaining(move_id, facing.mirror_vec(amount), false);
            }
            MoveMobility::Perpetual(amount) => {
                self.handle_move_velocity_chaining(move_id, facing.mirror_vec(amount), true);
            }
            MoveMobility::None => panic!("None MoveMobility in move velocity handling"),
        }
    }
    fn handle_move_velocity_chaining(&mut self, id: MoveId, amount: Vec3, perpetual: bool) {
        let first_move = self.current_move.is_none();

        if first_move {
            // Move started
            self.velocity = amount;
            self.current_move = Some(CurrentMove {
                id,
                base_velocity: Vec3::ZERO,
            });
        } else {
            let current_move = self.current_move.unwrap();
            let move_continues = current_move.id == id;

            if move_continues {
                if perpetual {
                    // Continue perpetual motion
                    self.velocity = current_move.base_velocity + amount;
                }
            } else {
                // Cancel into a new move
                self.add_impulse(amount);

                self.current_move = Some(CurrentMove {
                    id,
                    base_velocity: self.velocity,
                });
            }
        }
    }

    fn handle_walking_velocity(&mut self, direction: LRDirection) {
        let proposed_walk_velocity =
            self.velocity.x + direction.mirror_f32(constants::PLAYER_ACCELERATION);

        self.velocity.x = direction.mirror_f32(
            proposed_walk_velocity
                .abs()
                .clamp(constants::MINIMUM_WALK_SPEED, constants::MAXIMUM_WALK_SPEED),
        );
        self.current_move = None;
    }

    fn x_collision(&mut self) {
        // Just stop for now, but can be used to implement bounces and whatnot in the future
        self.velocity.x = 0.0;
    }

    fn y_collision(&mut self) {
        // Hit the floor
        self.velocity.y = 0.0;
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(run_max_once_per_combat_frame.system())
                .with_system(player_input.system())
                .with_system(sideswitcher.system())
                .with_system(push_players.system())
                .with_system(move_players.system())
                .with_system(move_constants.system())
                .with_system(player_gravity.system()),
        );
    }
}

fn player_input(mut query: Query<(&PlayerState, &mut PlayerVelocity, &LRDirection)>) {
    for (state, mut velocity, facing) in query.iter_mut() {
        if let Some(walk_direction) = state.get_walk_direction() {
            velocity.handle_walking_velocity(walk_direction);
        } else if let Some((id, mobility)) = state.get_move_mobility() {
            velocity.handle_move_velocity(id, mobility, facing);
        } else if state.is_grounded() {
            velocity.drag();
        }
    }
}

fn sideswitcher(
    mut players: Query<(Entity, &Transform, &mut LRDirection), With<Player>>,
    others: Query<(Entity, &Transform), With<Player>>,
) {
    for (entity, transform, mut facing) in players.iter_mut() {
        for (e, tf) in others.iter() {
            if e == entity {
                continue;
            }

            facing.set_flipped(transform.translation.x > tf.translation.x);
        }
    }
}

fn move_constants(
    mut commands: Commands,
    mut query: Query<(Entity, &ConstantVelocity, &mut Transform)>,
) {
    // Handle static collision
    for (entity, velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.shift;

        // Despawn the thing if it's outside of the arena
        if transform.translation.length() > ARENA_WIDTH + 1.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[allow(clippy::type_complexity)]
fn move_players(
    mut queries: QuerySet<(
        Query<(
            &mut PlayerVelocity,
            &mut Transform,
            &PlayerState,
            &LRDirection,
        )>,
        Query<&Transform, With<WorldCamera>>,
    )>,
) {
    let camera_x = queries
        .q1()
        .single()
        .map(|camtf| camtf.translation.x)
        .unwrap_or_default();

    // Handle static collision
    for (mut velocity, mut transform, state, facing) in queries.q0_mut().iter_mut() {
        let shift = velocity.get_shift();

        if let Some(collision) = static_collision(
            transform.translation,
            shift,
            state.get_collider_size(),
            camera_x,
            *facing,
            // FIXME: This is supposed to be the opponent's width, but it's a nightmare to get here and a constant (for now)
            // With bevy 0.6 queries, the movement/collision system should be rewritten to use the correct width.
            state.get_width(),
        ) {
            transform.translation = collision.legal_position;
            if collision.x_collision {
                velocity.x_collision();
            }

            if collision.y_collision {
                velocity.y_collision();
            }
        } else {
            transform.translation += shift;
        }
    }
}

#[allow(clippy::type_complexity)]
fn push_players(
    players: Query<Entity, With<Player>>,
    mut query_set: QuerySet<(
        Query<(&PlayerVelocity, &Transform, &PlayerState, &LRDirection)>,
        Query<&mut PlayerVelocity>,
    )>,
) {
    for entity1 in players.iter() {
        for entity2 in players.iter() {
            if entity1 != entity2 {
                let (velocity1, transform1, player1, facing1) =
                    query_set.q0().get(entity1).unwrap();
                let (velocity2, transform2, player2, _) = query_set.q0().get(entity2).unwrap();

                let future_position1 = transform1.translation + velocity1.get_shift();
                let future_position2 = transform2.translation + velocity2.get_shift();

                if rect_collision(
                    future_position1,
                    player1.get_collider_size(),
                    future_position2,
                    player2.get_collider_size(),
                ) {
                    // Player-player collision is happening
                    let distance = (transform1.translation - transform2.translation).length();

                    let moving_closer = (future_position1 - future_position2).length() < distance;

                    // Don't push when really close, this is to prevent spazzing as directions change
                    let push_vector = Vec3::new(
                        if moving_closer {
                            // Go backwards
                            -facing1.to_signum()
                        } else {
                            // Go to current direction
                            let val = velocity1.velocity.x;
                            if val == 0.0 {
                                val
                            } else {
                                val.signum()
                            }
                        },
                        0.0,
                        0.0,
                    );

                    let mut object1 = query_set.q1_mut().get_mut(entity1).unwrap();
                    object1.add_impulse(push_vector);
                    drop(object1);
                    let mut object2 = query_set.q1_mut().get_mut(entity2).unwrap();
                    object2.add_impulse(-push_vector);
                }
            }
        }
    }
}

fn player_gravity(mut players: Query<(&mut PlayerVelocity, &mut PlayerState, &Transform)>) {
    for (mut velocity, mut state, tf) in players.iter_mut() {
        let player_bottom = tf.translation.y - state.get_height() / 2.0;
        let is_airborne = player_bottom > GROUND_PLANE_HEIGHT;

        if is_airborne {
            velocity.add_impulse(-Vec3::Y * PLAYER_GRAVITY_PER_FRAME);
            if state.is_grounded() {
                state.jump();
            }
        } else if !state.is_grounded() {
            state.land();
        }
    }
}

#[derive(Debug)]
pub struct StaticCollision {
    legal_position: Vec3, // How much space there is to move
    x_collision: bool,
    y_collision: bool,
}
impl StaticCollision {
    fn did_collide(&self) -> bool {
        self.x_collision || self.y_collision
    }

    fn wrap(self) -> Option<StaticCollision> {
        if self.did_collide() {
            Some(self)
        } else {
            None
        }
    }
}

const CAMERA_EDGE_COLLISION_PADDING: f32 = 1.0;

fn static_collision(
    current_position: Vec3,
    movement: Vec3,
    player_size: Vec2,
    camera_x: f32,
    extra_padding_side: LRDirection,
    extra_padding_amount: f32,
) -> Option<StaticCollision> {
    let future_position = current_position + movement;
    let relative_ground_plane = GROUND_PLANE_HEIGHT + player_size.y / 2.0;

    let distance_to_ground = future_position.y - relative_ground_plane;
    let y_collision = distance_to_ground < 0.0;
    let legal_y = if y_collision {
        relative_ground_plane
    } else {
        future_position.y
    };

    let (right_wall, left_wall) = if extra_padding_side.to_flipped() {
        (
            ARENA_WIDTH.min(camera_x + VIEWPORT_WIDTH - CAMERA_EDGE_COLLISION_PADDING),
            (-ARENA_WIDTH).max(camera_x - VIEWPORT_WIDTH + CAMERA_EDGE_COLLISION_PADDING)
                + extra_padding_amount,
        )
    } else {
        (
            ARENA_WIDTH.min(camera_x + VIEWPORT_WIDTH - CAMERA_EDGE_COLLISION_PADDING)
                - extra_padding_amount,
            (-ARENA_WIDTH).max(camera_x - VIEWPORT_WIDTH + CAMERA_EDGE_COLLISION_PADDING),
        )
    };

    let (legal_x, x_collision) = if future_position.x > right_wall {
        (right_wall, true)
    } else if future_position.x < left_wall {
        (left_wall, true)
    } else {
        (future_position.x, false)
    };

    StaticCollision {
        legal_position: Vec3::new(legal_x, legal_y, 0.0),
        x_collision,
        y_collision,
    }
    .wrap()
}

pub fn rect_collision(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    // Bevy collide only detects collisions if the edges overlap, most of the time this is good enough
    // But occasionally a collider spawns inside another, in which case we need a check for that.
    let a_min = a_pos.truncate() - (a_size / 2.0);
    let a_max = a_pos.truncate() + (a_size / 2.0);
    let b_min = b_pos.truncate() - (b_size / 2.0);
    let b_max = b_pos.truncate() + (b_size / 2.0);

    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        return true;
    }
    false
}
