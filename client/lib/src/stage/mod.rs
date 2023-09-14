use bevy::prelude::*;
use wag_core::Model;

use crate::assets::Models;

mod light;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (light::setup, add_stage));
    }
}

fn add_stage(mut commands: Commands, models: Res<Models>) {
    commands.spawn(SceneBundle {
        scene: models[&Model::TrainingStage].clone(),
        ..default()
    });
}
