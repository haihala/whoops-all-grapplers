use bevy::prelude::*;
use bevy::render::camera::{
    camera_system, Camera, Camera2d, Camera3d, CameraProjection, DepthCalculation,
};
use bevy::render::primitives::Frustum;
use bevy::render::view::VisibleEntities;
use types::Player;

use crate::physics::ARENA_WIDTH;

#[derive(Debug, Component, Default)]
pub struct WorldCamera;

const CAMERA_FAR_DISTANCE: f32 = 10000.0;
const CAMERA_HEIGHT: f32 = 2.0;
pub const VIEWPORT_HALFWIDTH: f32 = 5.0;
const CAMERA_CLAMP: f32 = ARENA_WIDTH - VIEWPORT_HALFWIDTH;

// Originally from
// https://bevy-cheatbook.github.io/cookbook/custom-projection.html?highlight=window#custom-camera-projection
// Edited somewhat
#[derive(Default, Component, Clone, Copy)]
struct SimpleOrthoProjection {
    viewport_height: f32,
}
impl CameraProjection for SimpleOrthoProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            -VIEWPORT_HALFWIDTH,
            VIEWPORT_HALFWIDTH,
            -self.viewport_height,
            self.viewport_height,
            0.0,
            CAMERA_FAR_DISTANCE,
        )
    }

    // what to do on window resize
    fn update(&mut self, width: f32, height: f32) {
        self.viewport_height = VIEWPORT_HALFWIDTH * height / width;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        // for 2D (camera doesn't rotate)
        // DepthCalculation::ZDifference

        // otherwise
        DepthCalculation::Distance
    }

    fn far(&self) -> f32 {
        CAMERA_FAR_DISTANCE
    }
}

pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_cameras).add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(center_camera)
                .with_system(camera_system::<SimpleOrthoProjection>.after(center_camera)),
        );
    }
}

fn add_cameras(mut commands: Commands) {
    let projection = SimpleOrthoProjection::default();
    let camera = Camera {
        near: 0.0,
        far: CAMERA_FAR_DISTANCE,
        ..default()
    };
    // position the camera like bevy would do by default for 2D:
    let transform = Transform::from_xyz(0.0, CAMERA_HEIGHT, CAMERA_FAR_DISTANCE / 2.0);
    // frustum construction code copied from Bevy
    let view_projection = projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = Frustum::from_view_projection(
        &view_projection,
        &transform.translation,
        &transform.back(),
        CAMERA_FAR_DISTANCE,
    );

    commands.spawn_bundle((
        camera,
        projection,
        frustum,
        VisibleEntities::default(),
        transform,
        GlobalTransform::default(),
        Camera3d,
        Camera2d,
        WorldCamera,
    ));

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
