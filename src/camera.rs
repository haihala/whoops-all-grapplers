use bevy::prelude::*;

use crate::Materials;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_camera.system())
            .add_startup_system(add_stage.system());
    }
}

fn add_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale = crate::constants::CAMERA_SCALE.into();
    camera.transform.translation = crate::constants::CAMERA_POSITION.into();

    commands.spawn_bundle(camera);
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
