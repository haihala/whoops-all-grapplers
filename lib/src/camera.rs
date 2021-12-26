use bevy::prelude::*;
use bevy::render::camera::{
    camera_system, Camera, CameraProjection, DepthCalculation, VisibleEntities,
};
use bevy::render::render_graph::base::camera::CAMERA_2D;
use types::Player;

struct WorldCamera;

const CAMERA_FAR_DISTANCE: f32 = 10000.0;
const CAMERA_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = 5.0;

// Originally from
// https://bevy-cheatbook.github.io/cookbook/custom-projection.html?highlight=window#custom-camera-projection
// Edited somewhat
#[derive(Default)]
struct SimpleOrthoProjection {
    viewport_height: f32,
}

impl CameraProjection for SimpleOrthoProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            -VIEWPORT_WIDTH,
            VIEWPORT_WIDTH,
            -self.viewport_height,
            self.viewport_height,
            0.0,
            CAMERA_FAR_DISTANCE,
        )
    }

    // what to do on window resize
    fn update(&mut self, width: f32, height: f32) {
        self.viewport_height = VIEWPORT_WIDTH * height / width;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        // for 2D (camera doesn't rotate)
        DepthCalculation::ZDifference

        // otherwise
        //DepthCalculation::Distance
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_cameras.system())
            .add_system_to_stage(
                CoreStage::PostUpdate,
                camera_system::<SimpleOrthoProjection>.system(),
            )
            .add_system_to_stage(CoreStage::PostUpdate, center_camera.system());
    }
}

fn add_cameras(mut commands: Commands) {
    let camera = Camera {
        name: Some(CAMERA_2D.to_string()),
        ..Default::default()
    };
    let projection = SimpleOrthoProjection::default();

    commands
        .spawn_bundle((
            // position the camera like bevy would do by default for 2D:
            Transform::from_translation(Vec3::new(0.0, CAMERA_HEIGHT, CAMERA_FAR_DISTANCE - 0.1)),
            GlobalTransform::default(),
            VisibleEntities::default(),
            camera,
            projection,
        ))
        .insert(WorldCamera);

    commands.spawn_bundle(UiCameraBundle::default());
}

#[allow(clippy::type_complexity)]
fn center_camera(
    mut queryies: QuerySet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<WorldCamera>>,
    )>,
) {
    if let Some(player_pos_sum) = queryies
        .q0()
        .iter()
        .map(|x| x.translation)
        .reduce(|a, b| a + b)
    {
        if let Ok(mut transform) = queryies.q1_mut().single_mut() {
            transform.translation.x = player_pos_sum.x / 2.0; // 2 players
        }
    }
}
