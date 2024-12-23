use bevy::prelude::*;
use foundation::{InMatch, Model, RollbackSchedule, SystemStep};

use crate::assets::Models;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RollbackSchedule,
            (setup_lights, add_stage).in_set(SystemStep::SetupStage),
        );
    }
}

#[derive(Debug, Component)]
struct Stage;

fn add_stage(mut commands: Commands, models: Res<Models>, stages: Query<&Stage>) {
    if stages.is_empty() {
        commands.spawn((
            SceneRoot(models[&Model::TrainingStage].clone()),
            Name::new("Stage"),
            StateScoped(InMatch),
            Stage,
        ));
    }
}

#[derive(Debug, Component)]
struct MainLight;

fn setup_lights(mut commands: Commands, lights: Query<&MainLight>) {
    if lights.is_empty() {
        commands.insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        });

        commands.spawn((
            PointLight::default(),
            Transform::from_xyz(0.0, 5.0, 2.0),
            Name::new("Point light"),
            MainLight,
            StateScoped(InMatch),
        ));
    }
}
