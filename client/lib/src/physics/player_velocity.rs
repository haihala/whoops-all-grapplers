use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use characters::Movement;
use types::Facing;

#[derive(Debug, Inspectable, Clone, Default, Copy)]
pub struct AppliedMovement {
    amount: Vec2,
    until_frame: usize,
}
#[derive(Debug, Inspectable, Clone, Default, Component)]
pub struct PlayerVelocity {
    velocity: Vec2,
    movements: Vec<AppliedMovement>,
    /// Keep track of if pushing is currently happening for wall clamp reasons
    pub(super) pushing: bool,
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
    pub(super) fn get_shift(&self) -> Vec2 {
        self.velocity / constants::FPS
    }
    pub fn add_impulse(&mut self, impulse: Vec2) {
        self.velocity += impulse;
    }
    pub(super) fn handle_movement(&mut self, frame: usize, facing: Facing, movement: Movement) {
        // This will make it so that lengthening the duration of a movement will spread out the amount across the duration.
        // Basically, you can double the lenght and it shouldn't affect the total distance
        let amount = facing.mirror_vec2(movement.amount);
        self.movements.push(AppliedMovement {
            amount: amount.normalize() * (amount.length() / movement.duration as f32),
            until_frame: frame + movement.duration,
        });
    }
    pub(super) fn handle_walking_velocity(&mut self, direction: Facing) {
        let proposed_walk_velocity = self.velocity.x + direction.mirror_f32(PLAYER_ACCELERATION);

        self.velocity.x = direction.mirror_f32(
            proposed_walk_velocity
                .abs()
                .clamp(MINIMUM_WALK_SPEED, MAXIMUM_WALK_SPEED),
        );
    }
    pub(super) fn drag(&mut self) {
        let abs_x = self.velocity.x.abs() * (1.0 - PROPORTIONAL_DRAG);

        self.velocity = Vec2::new(
            if abs_x > LINEAR_DRAG {
                self.velocity.x.signum() * (abs_x - LINEAR_DRAG)
            } else {
                0.0
            },
            self.velocity.y,
        );
    }
    pub(super) fn sum_movements(&mut self) {
        self.add_impulse(
            self.movements
                .iter()
                .map(|am| am.amount)
                .fold(Vec2::ZERO, |collector, item| collector + item),
        );
    }
    pub(super) fn cleanup_movements(&mut self, frame: usize) {
        self.movements
            .retain(|movement| movement.until_frame > frame);
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
