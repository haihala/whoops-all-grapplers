use bevy::prelude::*;

pub struct Colors {
    pub transparent: Handle<ColorMaterial>,
    pub health: Handle<ColorMaterial>,
    pub hitbox: Handle<ColorMaterial>,
    pub hurbox: Handle<ColorMaterial>,
    pub collision_box: Handle<ColorMaterial>,
}

pub struct Fonts {
    pub basic: Handle<Font>,
}

pub struct Sprites {
    pub background_image: Handle<StandardMaterial>,
}
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(
            crate::labels::StartupStageLabel::LoadAssets,
            colors.system(),
        )
        .add_startup_system_to_stage(crate::labels::StartupStageLabel::LoadAssets, fonts.system())
        .add_startup_system_to_stage(
            crate::labels::StartupStageLabel::LoadAssets,
            sprites.system(),
        );
    }
}

fn colors(mut commands: Commands, mut color_assets: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Colors {
        transparent: color_assets.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
        health: color_assets.add(Color::rgb(0.9, 0.0, 0.0).into()),
        hitbox: color_assets.add(Color::rgb(1.0, 0.0, 0.0).into()),
        hurbox: color_assets.add(Color::rgb(0.0, 1.0, 0.0).into()),
        collision_box: color_assets.add(Color::rgb(0.0, 0.0, 1.0).into()),
    })
}

fn fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let basic = asset_server.load("FiraSans-Bold.ttf");

    commands.insert_resource(Fonts { basic })
}

fn sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_assets: ResMut<Assets<StandardMaterial>>,
) {
    let texture = asset_server.load("CPT-2018-Stage.png");

    commands.insert_resource(Sprites {
        background_image: sprite_assets.add(StandardMaterial {
            base_color_texture: Some(texture),
            unlit: true,
            ..Default::default()
        }),
    })
}
