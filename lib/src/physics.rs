use bevy::{prelude::*, sprite};
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

#[derive(Debug, Default, Inspectable, Component)]
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
#[derive(Debug, Inspectable, Clone, Default, Copy, Component)]
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

    fn handle_collisions(&mut self, clamped_position: &ClampedPosition) {
        if clamped_position.touching_wall() {
            self.x_collision();
        }
        if clamped_position.touching_floor {
            self.y_collision();
        }
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
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(run_max_once_per_combat_frame)
                .with_system(player_input)
                .with_system(sideswitcher)
                .with_system(move_players)
                .with_system(move_constants)
                .with_system(player_gravity),
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
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[allow(clippy::type_complexity)]
fn move_players(
    mut queries: QuerySet<(
        QueryState<(&mut PlayerVelocity, &mut Transform, &PlayerState)>,
        QueryState<&Transform, With<WorldCamera>>,
    )>,
) {
    let arena_rect = legal_position_space(queries.q1().single().translation.x);

    let mut player_query = queries.q0();
    if let Some([(mut velocity1, mut tf1, state1), (mut velocity2, mut tf2, state2)]) =
        player_query.iter_combinations_mut().fetch_next()
    {
        let clamped_position1 = clamp_position(
            tf1.translation + velocity1.get_shift(),
            state1.get_collider_size(),
            arena_rect,
        );
        let clamped_position2 = clamp_position(
            tf2.translation + velocity2.get_shift(),
            state2.get_collider_size(),
            arena_rect,
        );

        tf1.translation = clamped_position1.position;
        tf2.translation = clamped_position2.position;

        velocity1.handle_collisions(&clamped_position1);
        velocity2.handle_collisions(&clamped_position2);

        if let Some(push_force) = push_force(
            clamped_position1.position,
            state1.get_collider_size(),
            clamped_position2.position,
            state2.get_collider_size(),
        ) {
            let can_move1 = clamped_position1.can_move_horizontally(push_force);
            let can_move2 = clamped_position2.can_move_horizontally(-push_force);
            assert!(
                can_move1 || can_move2,
                "Both players are blocked by walls somehow"
            );

            if can_move1 && can_move2 {
                // Both can move
                tf1.translation += Vec3::X * push_force / 2.0;
                tf2.translation -= Vec3::X * push_force / 2.0;
            } else if can_move1 {
                // 1 can move, 2 cannot
                velocity1.x_collision();
                tf1.translation += Vec3::X * push_force;
            } else {
                // 2 can move, 1 cannot
                velocity2.x_collision();
                tf2.translation -= Vec3::X * push_force;
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
struct ClampedPosition {
    position: Vec3,
    touching_right_wall: bool,
    touching_left_wall: bool,
    touching_floor: bool,
}
impl ClampedPosition {
    fn touching_wall(&self) -> bool {
        self.touching_left_wall || self.touching_right_wall
    }

    fn can_move_horizontally(&self, amount: f32) -> bool {
        if amount > 0.0 {
            // Moving right
            !self.touching_right_wall
        } else {
            // Moving left
            !self.touching_left_wall
        }
    }
}

const CAMERA_EDGE_COLLISION_PADDING: f32 = 1.0;

fn clamp_position(position: Vec3, size: Vec2, arena_rect: Rect<f32>) -> ClampedPosition {
    let halfsize = size / 2.0;

    let player_rect = Rect {
        left: position.x - halfsize.x,
        right: position.x + halfsize.x,
        bottom: position.y - halfsize.y,
        top: position.y + halfsize.y,
    };

    let touching_right_wall = player_rect.right >= arena_rect.right;
    let touching_left_wall = player_rect.left <= arena_rect.left;
    let clamped_x = if touching_right_wall {
        arena_rect.right - halfsize.x
    } else if touching_left_wall {
        arena_rect.left + halfsize.x
    } else {
        position.x
    };

    let touching_floor = player_rect.bottom <= arena_rect.bottom;
    let clamped_y = if touching_floor {
        arena_rect.bottom + halfsize.y
    } else {
        position.y
    };

    ClampedPosition {
        position: Vec3::new(clamped_x, clamped_y, 0.0),
        touching_right_wall,
        touching_left_wall,
        touching_floor,
    }
}

fn legal_position_space(camera_x: f32) -> Rect<f32> {
    Rect {
        bottom: GROUND_PLANE_HEIGHT,
        right: ARENA_WIDTH.min(camera_x + VIEWPORT_WIDTH - CAMERA_EDGE_COLLISION_PADDING),
        left: (-ARENA_WIDTH).max(camera_x - VIEWPORT_WIDTH + CAMERA_EDGE_COLLISION_PADDING),
        top: std::f32::INFINITY,
    }
}

pub fn rect_collision(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    // Bevy collide only detects collisions if the edges overlap, most of the time this is good enough
    // But occasionally a collider spawns inside another, in which case we need a check for that.
    let a = sprite::Rect {
        min: a_pos.truncate() - a_size / 2.0,
        max: a_pos.truncate() + a_size / 2.0,
    };
    let b = sprite::Rect {
        min: b_pos.truncate() - b_size / 2.0,
        max: b_pos.truncate() + b_size / 2.0,
    };

    let x_overlap = a.min.x < b.max.x && a.max.x > b.min.x;
    let y_overlap = a.min.y < b.max.y && a.max.y > b.min.y;
    x_overlap && y_overlap
}

fn push_force(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<f32> {
    if rect_collision(a_pos, a_size, b_pos, b_size) {
        let clean_distance = (a_size + b_size).x / 2.0;
        let distance = (a_pos - b_pos).x;
        Some(distance.signum() * ((clean_distance / distance.abs()) - 1.0))
    } else {
        None
    }
}
