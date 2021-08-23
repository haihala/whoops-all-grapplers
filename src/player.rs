use bevy::prelude::*;

use crate::Materials;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_player.system());
    }
}

pub struct Player;

fn add_player(mut commands: Commands, assets: Res<Materials>) {
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
