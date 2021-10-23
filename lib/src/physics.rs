use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::Inspectable;

use types::{Player, PlayerState};

use crate::clock::Clock;

#[derive(Debug, Default, Inspectable, Clone, Copy)]
pub struct PhysicsObject {
    pub velocity: Vec3,
    impulse: Vec3,
    drag_multiplier: f32,
}
impl PhysicsObject {
    pub fn add_impulse(&mut self, impulse: Vec3) {
        self.impulse += impulse;
    }
    fn use_impulse(&mut self) -> Vec3 {
        let impulse = self.impulse.clone();
        self.impulse = Vec3::ZERO;
        impulse
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(constants::FPS_F64))
                .with_system(gravity.system())
                .with_system(player_input.system())
                .with_system(sideswitcher.system())
                .with_system(move_players.system())
                .with_system(push_players.system()),
        );
    }
}

fn gravity(mut query: Query<(&mut PhysicsObject, &PlayerState)>) {
    for (mut object, state) in query.iter_mut() {
        if !state.is_grounded() {
            object.velocity.y -= constants::PLAYER_GRAVITY_PER_FRAME;
        }
    }
}

fn player_input(mut query: Query<(&mut PlayerState, &mut PhysicsObject)>, clock: Res<Clock>) {
    // TODO: This could use a once-over, but the problem is complex
    // Try to dash
    // If can't dash, try to jump
    // If can't jump, walk
    // If can't walk, drag
    // If can't drag, do nothing
    for (mut state, mut object) in query.iter_mut() {
        if let Some(dash_phase) = state.get_dash_phase(clock.frame) {
            // Dashing
            let direction = state.get_dash_direction().unwrap();

            object.velocity = match dash_phase {
                types::DashPhase::Start => direction * constants::DASH_START_SPEED,
                types::DashPhase::Recovery => direction * constants::DASH_RECOVERY_SPEED,
            };
        } else {
            if let Some(impulse) = state.get_jump_impulse() {
                // Jumping
                object.add_impulse(impulse);
            } else {
                if let Some(direction) = state.get_walk_direction() {
                    // Walking
                    object.velocity = direction * constants::WALK_SPEED;
                } else if state.is_grounded() {
                    // Drag
                    if object.velocity.length() < constants::DRAG {
                        object.velocity = Vec3::ZERO;
                    } else {
                        object.velocity = (object.velocity.length() - constants::DRAG)
                            * object.velocity.normalize();
                    }
                }
            }
        }
    }
}

fn sideswitcher(
    mut players: Query<(Entity, &Transform, &mut PlayerState), With<Player>>,
    others: Query<(Entity, &Transform), With<Player>>,
) {
    for (entity, transform, mut state) in players.iter_mut() {
        for (e, tf) in others.iter() {
            if e == entity {
                continue;
            }

            state.set_flipped(transform.translation.x > tf.translation.x);
        }
    }
}

fn move_players(mut players: Query<(&mut PhysicsObject, &mut Transform, &mut PlayerState)>) {
    // Handle static collision
    for (mut object, mut transform, mut state) in players.iter_mut() {
        let impulse = object.use_impulse();
        object.velocity += impulse;
        let shift = object.velocity / constants::FPS;

        if let Some(collision) = static_collision(transform.translation, shift) {
            transform.translation += collision.legal_movement;
            if collision.x_collision {
                object.velocity.x = 0.0;
            }

            if collision.y_collision {
                object.velocity.y = 0.0;
                state.land()
            }
        } else {
            transform.translation += shift;
        }
    }
}

fn push_players(
    players: Query<Entity, With<Player>>,
    mut query_set: QuerySet<(
        Query<(&PhysicsObject, &Transform)>,
        Query<(&mut PhysicsObject, &mut Transform, &mut PlayerState)>,
    )>,
) {
    for entity1 in players.iter() {
        for entity2 in players.iter() {
            if entity1 != entity2 {
                let (object1, transform1) = query_set.q0().get(entity1).unwrap();
                let (object2, transform2) = query_set.q0().get(entity2).unwrap();

                let future_position1 = transform1.translation + object1.velocity / constants::FPS;
                let future_position2 = transform2.translation + object2.velocity / constants::FPS;

                if rect_collision(
                    future_position1,
                    constants::PLAYER_COLLIDER_SIZE.into(),
                    future_position2,
                    constants::PLAYER_COLLIDER_SIZE.into(),
                ) {
                    // Player-player collision is happening
                    let difference = future_position1 - future_position2;

                    // Leads into some wonky interactions, but allows for pushing
                    // Should probably be based on the distance, but turned off when too close
                    if difference.length() > constants::PUSHING_DEAD_ZONE {
                        let push_vector = Vec3::new(
                            difference.normalize_or_zero().x / difference.length(),
                            0.0,
                            0.0,
                        );

                        let (mut object, _, _) = query_set.q1_mut().get_mut(entity1).unwrap();
                        object.add_impulse(push_vector);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct StaticCollision {
    legal_movement: Vec3, // How much space there is to move
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

fn static_collision(current_position: Vec3, movement: Vec3) -> Option<StaticCollision> {
    let mut future_position = current_position + movement;
    let mut ratio = 1.0;

    let mut collision = StaticCollision {
        legal_movement: movement,
        x_collision: false,
        y_collision: false,
    };

    let distance_to_ground = future_position.y - constants::GROUND_PLANE_HEIGHT;
    if distance_to_ground <= 0.0 {
        collision.y_collision = true;

        ratio = (current_position.y - constants::GROUND_PLANE_HEIGHT) / movement.y;

        collision.legal_movement = movement * ratio;
        future_position = current_position + collision.legal_movement;
    }

    if future_position.x.abs() > constants::ARENA_WIDTH {
        collision.x_collision = true;

        ratio = if future_position.x > 0.0 {
            // Colliding to the right wall
            ratio.min((current_position.x - constants::ARENA_WIDTH) / movement.x)
        } else {
            // Colliding to the left wall
            ratio.min((current_position.x + constants::ARENA_WIDTH) / movement.x)
        };
        collision.legal_movement = movement * ratio;
    }

    collision.wrap()
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
