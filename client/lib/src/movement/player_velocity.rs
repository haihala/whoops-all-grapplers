use bevy::prelude::*;

use characters::Movement;
use wag_core::Facing;

#[derive(Debug, Reflect, Clone, Default, Copy)]
pub struct AppliedMovement {
    amount: Vec2,
    until_frame: usize,
}
#[derive(Debug, Reflect, Default, Clone, Component)]
pub struct PlayerVelocity {
    velocity: Vec2,
    movements: Vec<AppliedMovement>,
    /// Keep track of if pushing is currently happening for wall clamp reasons
    pub(super) pushing: bool,
    pub on_floor: bool,
    pub next_pos: Vec2,
}

const PROPORTIONAL_DRAG: f32 = 0.03;
const LINEAR_DRAG: f32 = 0.3;
const WALK_BACK_SPEED_MULTIPLIER: f32 = 0.7;

impl PlayerVelocity {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn sync_with(&mut self, other: &PlayerVelocity) {
        self.velocity = other.velocity;
        // This may need other actions, used primarily when snapping to the other player
    }

    pub(super) fn get_shift(&self) -> Vec2 {
        self.velocity / wag_core::FPS
    }
    pub fn add_impulse(&mut self, impulse: Vec2) {
        self.velocity += impulse;
    }

    pub(super) fn clear_movements(&mut self) {
        self.movements.clear();
        self.velocity = Vec2::ZERO; // TODO: This may be a mistake
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
    pub(super) fn handle_walking_velocity(
        &mut self,
        walk_speed: f32,
        facing: Facing,
        direction: Facing,
    ) {
        // Makes the change go the right way
        let direction_multiplier = if direction == Facing::Left { -1.0 } else { 1.0 };

        // Makes you walk slower backwards
        let magnitude_multiplier = if direction != facing {
            WALK_BACK_SPEED_MULTIPLIER
        } else {
            1.0
        };

        self.velocity.x = direction_multiplier * magnitude_multiplier * walk_speed;
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
    pub(super) fn apply_movements(&mut self) {
        self.add_impulse(self.movements.iter().map(|am| am.amount).sum());
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
