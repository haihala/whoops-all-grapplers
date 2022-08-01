use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use types::{Facing, MoveId};

#[derive(Debug, Inspectable, Clone, Default, Copy)]
pub struct CurrentMove {
    id: (MoveId, i32),
    base_velocity: Vec3,
}
#[derive(Debug, Inspectable, Clone, Default, Copy, Component)]
pub struct PlayerVelocity {
    velocity: Vec3,
    /// Keep track of if pushing is currently happening for wall clamp reasons
    pub(super) pushing: bool,
    pub(super) current_move: Option<CurrentMove>,
}
// Drag
const MINIMUM_WALK_SPEED: f32 = 3.0;
const MAXIMUM_WALK_SPEED: f32 = 4.0;
const ACCELERATION_TIME: f32 = 1.0;

const ACCELERATION_DELTA: f32 = MAXIMUM_WALK_SPEED - MINIMUM_WALK_SPEED;
const PLAYER_ACCELERATION: f32 = ACCELERATION_DELTA / ACCELERATION_TIME / constants::FPS;

const PROPORTIONAL_DRAG: f32 = 0.03;
const LINEAR_DRAG: f32 = 0.3;

impl PlayerVelocity {
    pub(super) fn get_shift(&self) -> Vec3 {
        self.velocity / constants::FPS
    }
    pub fn add_impulse(&mut self, impulse: Vec3) {
        self.velocity += impulse;
    }
    pub(super) fn drag(&mut self) {
        let abs_x = self.velocity.x.abs() * (1.0 - PROPORTIONAL_DRAG);

        self.velocity = Vec3::new(
            if abs_x > LINEAR_DRAG {
                self.velocity.x.signum() * (abs_x - LINEAR_DRAG)
            } else {
                0.0
            },
            self.velocity.y,
            0.0,
        );
    }

    pub(super) fn handle_walking_velocity(&mut self, direction: Facing) {
        let proposed_walk_velocity = self.velocity.x + direction.mirror_f32(PLAYER_ACCELERATION);

        self.velocity.x = direction.mirror_f32(
            proposed_walk_velocity
                .abs()
                .clamp(MINIMUM_WALK_SPEED, MAXIMUM_WALK_SPEED),
        );
        self.current_move = None;
    }

    pub(super) fn x_collision(&mut self) {
        // Just stop for now, but can be used to implement bounces and whatnot in the future
        self.velocity.x = 0.0;
    }

    pub(super) fn y_collision(&mut self) {
        // Hit the floor
        self.velocity.y = 0.0;
    }
}
