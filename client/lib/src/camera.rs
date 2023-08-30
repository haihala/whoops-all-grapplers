use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use wag_core::Player;

use crate::physics::ARENA_WIDTH;

#[derive(Debug, Component, Default)]
pub struct WorldCamera;

pub const VIEWPORT_HALFWIDTH: f32 = 4.0; // This is used to control stage border relative to the camera
const CAMERA_CLAMP: f32 = ARENA_WIDTH - VIEWPORT_HALFWIDTH;

pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_cameras)
            .add_systems(Update, center_camera);
    }
}

fn add_cameras(mut commands: Commands) {
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::FixedHorizontal(VIEWPORT_HALFWIDTH * 2.0),
        ..default()
    };

    let camera_container = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 1.8, 10.0),
                ..default()
            },
            Name::new("Cameras"),
            WorldCamera,
        ))
        .id();

    commands
        .spawn((
            Camera3dBundle {
                projection: projection.clone().into(),
                ..default()
            },
            Name::new("3d Cam"),
        ))
        .set_parent(camera_container);

    commands
        .spawn((
            Camera2dBundle {
                transform: Transform::from_translation(Vec3::ZERO),
                camera: Camera {
                    // Higher is rendered later
                    order: 1,
                    ..default()
                },
                camera_2d: Camera2d {
                    // Don't draw a clear color on top of the 3d stuff
                    clear_color: ClearColorConfig::None,
                },
                projection,
                ..default()
            },
            Name::new("2d Cam"),
        ))
        .set_parent(camera_container);
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

    // Do some light lerping to make backthrows less jarring
    let mut camquery = queryies.p1();
    let mut tf = camquery.single_mut();
    let target = Vec3 {
        x: player_pos_sum.clamp(-CAMERA_CLAMP, CAMERA_CLAMP),
        ..tf.translation
    };
    tf.translation = tf.translation.lerp(target, 0.1);
}
