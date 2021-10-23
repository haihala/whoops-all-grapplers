use bevy::{core::FixedTimestep, prelude::*};
use bevy_inspector_egui::Inspectable;
use num::clamp;
use types::{Player, PlayerState};

#[derive(Debug, Default, Inspectable)]
pub struct PhysicsObject {
    pub velocity: Vec3,
    pub desired_velocity: Option<Vec3>,
    impulse: Vec3,
    drag_multiplier: f32,
}
impl PhysicsObject {
    pub fn add_impulse(&mut self, impulse: Vec3) {
        self.impulse += impulse;
    }
    fn use_impulse(&mut self) -> Vec3 {
        let impulse = self.impulse;
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
                .with_system(player_drag.system())
                .with_system(incorporate_desired_velocity.system())
                .with_system(sideswitcher.system())
                .with_system(move_objects.system()),
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

fn player_drag(mut query: Query<(&mut PhysicsObject, &PlayerState)>) {
    for (mut object, state) in query.iter_mut() {
        let drag = object.drag_multiplier
            * if !state.is_grounded() {
                constants::AIR_DRAG
            } else {
                constants::GROUND_DRAG
            };

        if drag > 0.0 {
            let speed = (object.velocity.length() - drag).max(0.0);
            object.velocity = object.velocity.normalize_or_zero() * speed;
        }
    }
}

fn incorporate_desired_velocity(mut query: Query<&mut PhysicsObject>) {
    for mut object in query.iter_mut() {
        if let Some(desired) = object.desired_velocity {
            let desired_direction = desired.x.signum();
            let current_direction = object.velocity.x.signum();

            object.velocity.y = desired.y;

            #[allow(clippy::float_cmp)]
            if object.velocity.x == 0.0 || current_direction == desired_direction {
                object.velocity.x =
                    desired_direction * object.velocity.x.abs().max(desired.x.abs());
                object.drag_multiplier = 0.0;
            } else {
                object.drag_multiplier = constants::REVERSE_DRAG_MULTIPLIER;
            }
        } else {
            object.drag_multiplier = 1.0;
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

fn move_objects(mut query: Query<(&mut PhysicsObject, &mut Transform, &mut PlayerState)>) {
    for (mut object, mut transform, mut state) in query.iter_mut() {
        let impulse = object.use_impulse();
        object.velocity += impulse;
        transform.translation += (object.velocity) / constants::FPS;

        if transform.translation.y < constants::GROUND_PLANE_HEIGHT {
            object.velocity.y = clamp(object.velocity.y, 0.0, f32::MAX);
            transform.translation.y = constants::GROUND_PLANE_HEIGHT;
            state.land();
        }

        if transform.translation.x.abs() > constants::ARENA_WIDTH {
            object.velocity.x = 0.0;
            transform.translation.x = transform.translation.x.signum() * constants::ARENA_WIDTH;
        }
    }
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
