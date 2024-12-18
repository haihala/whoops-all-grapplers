use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{Area, Model};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Pickup {
    Kunai,
}

impl Pickup {
    pub fn spawn_info(self) -> (Model, Transform) {
        (
            Model::Kunai,
            Transform::from_rotation(Quat::from_rotation_z(PI / 2.0))
                .with_translation(Vec3::Y * 0.5),
        )
    }

    pub fn allow_pickup_by(&self, is_owner: bool) -> bool {
        match self {
            // Only owner can pick up
            Pickup::Kunai => is_owner,
        }
    }
}

#[derive(Debug, Clone, Copy, Event)]
pub struct PickupRequest {
    pub pickup: Pickup,
    pub spawn_point: Vec2,
    pub spawn_velocity: Vec2,
    pub size: Area,
    pub gravity: f32,
    pub lifetime: Option<usize>,
    pub flip_owner: bool,
}
