use bevy::prelude::*;
use wag_core::Model;

use crate::assets::Models;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_lights, add_stage));
    }
}

fn add_stage(mut commands: Commands, models: Res<Models>) {
    commands.spawn(SceneBundle {
        scene: models[&Model::TrainingStage].clone(),
        ..default()
    });
}

pub fn setup_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 1.0,
        ..default()
    });

    commands.spawn((
        PointLightBundle {
            transform: Transform::from_xyz(0.0, 5.0, 2.0),
            ..default()
        },
        Name::new("Point light"),
    ));
}
