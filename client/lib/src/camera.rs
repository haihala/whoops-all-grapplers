use bevy::prelude::*;
use bevy::render::camera::{Camera2d, ScalingMode};
use types::Player;

use crate::physics::ARENA_WIDTH;

#[derive(Debug, Component, Default)]
pub struct WorldCamera;

pub const VIEWPORT_HALFWIDTH: f32 = 4.0; // This is used to control stage border relative to the camera
const CAMERA_CLAMP: f32 = ARENA_WIDTH - VIEWPORT_HALFWIDTH;

pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_cameras)
            .add_system_to_stage(CoreStage::PostUpdate, center_camera);
    }
}

fn add_cameras(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle {
            transform: Transform::from_xyz(0.0, 1.8, 10.0),
            orthographic_projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal,
                scale: 4.0,
                ..default()
            },
            ..OrthographicCameraBundle::new_3d()
        })
        .insert(WorldCamera)
        .insert(Camera2d);

    commands.spawn_bundle(UiCameraBundle::default());
}

#[allow(clippy::type_complexity)]
fn center_camera(
    mut queryies: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<WorldCamera>>,
    )>,
) {
    let player_pos_sum = queryies
        .p0()
        .iter()
        .fold(0.0, |acc, tf| acc + tf.translation.x)
        / 2.0;

    queryies.p1().single_mut().translation.x = player_pos_sum.max(-CAMERA_CLAMP).min(CAMERA_CLAMP);
}
