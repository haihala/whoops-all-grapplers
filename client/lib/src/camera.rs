use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use core::Player;

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
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::FixedHorizontal(VIEWPORT_HALFWIDTH * 2.0),
        ..default()
    };

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(0.0, 1.8, 10.0),
            ..default()
        })
        .insert(Name::new("Cameras"))
        .insert(WorldCamera)
        .with_children(|parent| {
            parent
                .spawn_bundle(Camera3dBundle {
                    projection: projection.clone().into(),
                    ..default()
                })
                .insert(Name::new("3d Cam"));
            parent
                .spawn_bundle(Camera2dBundle {
                    transform: Transform::from_translation(Vec3::ZERO),
                    camera: Camera {
                        // Higher is rendered later
                        priority: 1,
                        ..default()
                    },
                    camera_2d: Camera2d {
                        // Don't draw a clear color on top of the 3d stuff
                        clear_color: ClearColorConfig::None,
                    },
                    projection,
                    ..default()
                })
                .insert(Name::new("2d Cam"));
        });
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
