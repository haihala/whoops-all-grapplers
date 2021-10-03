use bevy::prelude::*;

use crate::Sprites;

mod bars;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(bars::setup.system())
            .add_system(bars::update.system())
            .add_startup_system(add_stage.system());
    }
}

fn add_stage(mut commands: Commands, sprites: Res<Sprites>, mut meshes: ResMut<Assets<Mesh>>) {
    // TODO: This could probably be made better with some other mechanism.

    let uvs = vec![[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];

    let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    commands.spawn_bundle(PbrBundle {
        material: sprites.background_image.clone(),
        mesh: meshes.add(mesh),
        transform: Transform {
            translation: crate::BACKGROUND_POSITION.into(),
            scale: crate::BACKGROUND_SCALE.into(),
            ..Default::default()
        },

        ..Default::default()
    });
}
