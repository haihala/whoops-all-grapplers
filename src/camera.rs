use bevy::prelude::*;
use bevy::render::camera::{
    camera_system, Camera, CameraProjection, DepthCalculation, VisibleEntities,
};
use bevy::render::render_graph::base::camera::CAMERA_2D;

use crate::player::Player;
use crate::Materials;

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
            -crate::constants::VIEWPORT_WIDTH,
            crate::constants::VIEWPORT_WIDTH,
            -self.viewport_height,
            self.viewport_height,
            0.0,
            crate::constants::CAMERA_FAR_DISTANCE,
        )
    }

    // what to do on window resize
    fn update(&mut self, width: f32, height: f32) {
        self.viewport_height = crate::constants::VIEWPORT_WIDTH * height / width;
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
        app.add_startup_system(add_camera.system())
            .add_startup_system(add_stage.system())
            .add_system_to_stage(
                CoreStage::PostUpdate,
                camera_system::<SimpleOrthoProjection>.system(),
            )
            .add_system_to_stage(CoreStage::PostUpdate, center_camera.system());
    }
}

fn add_camera(mut commands: Commands) {
    let projection = SimpleOrthoProjection::default();
    let cam_name = CAMERA_2D;
    let mut camera = Camera::default();
    camera.name = Some(cam_name.to_string());

    commands.spawn_bundle((
        // position the camera like bevy would do by default for 2D:
        Transform::from_translation(Vec3::new(
            0.0,
            crate::constants::CAMERA_HEIGHT,
            crate::constants::CAMERA_FAR_DISTANCE - 0.1,
        )),
        GlobalTransform::default(),
        VisibleEntities::default(),
        camera,
        projection,
    ));
}

fn add_stage(mut commands: Commands, materials: Res<Materials>, mut meshes: ResMut<Assets<Mesh>>) {
    let uvs = vec![[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];

    let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    commands.spawn_bundle(PbrBundle {
        material: materials.background_image.clone(),
        mesh: meshes.add(mesh),
        transform: Transform {
            translation: crate::constants::BACKGROUND_POSITION.into(),
            scale: crate::constants::BACKGROUND_SCALE.into(),
            ..Default::default()
        },

        ..Default::default()
    });
}

fn center_camera(
    mut queryies: QuerySet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Camera>>,
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
