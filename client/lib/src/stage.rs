use bevy::prelude::*;
use foundation::{InMatch, MatchState, Model};

use crate::assets::Models;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MatchState::Loading), (setup_lights, add_stage));
    }
}

#[derive(Debug, Component)]
struct Stage;

fn add_stage(mut commands: Commands, models: Res<Models>, stages: Query<&Stage>) {
    if stages.get_single().is_ok() {
        // Stage already exists
        return;
    }

    commands.spawn((
        SceneBundle {
            scene: models[&Model::TrainingStage].clone(),
            ..default()
        },
        Name::new("Stage"),
        StateScoped(InMatch),
    ));
}

#[derive(Debug, Component)]
struct Spotlight;

fn setup_lights(mut commands: Commands, lights: Query<&Spotlight>) {
    if lights.get_single().is_ok() {
        // Light already exists
        return;
    }

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
        StateScoped(InMatch),
    ));
}
