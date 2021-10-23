use bevy::prelude::*;

use crate::{game_flow::GameState, Sprites};

mod bars;
mod round_text;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(bars::setup.system())
            .add_system(bars::update.system())
            .add_startup_system(round_text::setup.system())
            .add_system_set(
                SystemSet::on_enter(GameState::Combat)
                    .with_system(round_text::round_start.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::PostRound)
                    .with_system(round_text::round_over.system()),
            )
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
            translation: constants::BACKGROUND_POSITION.into(),
            scale: constants::BACKGROUND_SCALE.into(),
            ..Default::default()
        },

        ..Default::default()
    });
}
