use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 1.0,
        ..default()
    });

    commands
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(0.0, 5.0, 2.0),
            ..default()
        })
        .insert(Name::new("Point light"));
}
