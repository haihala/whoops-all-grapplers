use bevy::prelude::*;

use crate::assets::Sprites;

mod light;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(light::setup)
            .add_startup_system(add_stage);
    }
}

fn add_stage(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // As it is in Bevy 0.7, you can't put 3d assets on top of 2d sprites
    // Because of this, use a quad for a background.

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 20.0;
    let quad_height = quad_width * 9.0 / 16.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_height,
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(sprites.background_image.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // textured quad - normal
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle,
        material: material_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 2.4, -5.0),
            ..default()
        },
        ..default()
    });
}
