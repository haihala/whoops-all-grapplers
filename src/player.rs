use bevy::prelude::*;

use crate::Materials;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(move_left.system());
    }
}

pub struct Player;

fn setup(mut commands: Commands, assets: Res<Materials>) {
    let width = 10.;
    let height = 15.;

    commands
        .spawn_bundle(SpriteBundle {
            material: assets.collision_box_color.clone(),
            sprite: Sprite::new(Vec2::new(width, height)),
            ..Default::default()
        })
        .insert(Player);
}

fn move_left(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += 1.;
    }
}
