use bevy::prelude::*;

pub struct Colors {
    pub transparent: Color,
    pub health: Color,
    pub meter: Color,
    pub hitbox: Color,
    pub hurtbox: Color,
    pub collision_box: Color,
}

pub struct Fonts {
    pub basic: Handle<Font>,
}

pub struct Sprites {
    pub background_image: Handle<Image>,
}
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, colors)
            .add_startup_system_to_stage(StartupStage::PreStartup, fonts)
            .add_startup_system_to_stage(StartupStage::PreStartup, sprites);
    }
}

fn colors(mut commands: Commands) {
    commands.insert_resource(Colors {
        transparent: Color::rgba(0.0, 0.0, 0.0, 0.0),
        health: Color::rgb(0.9, 0.0, 0.0),
        meter: Color::rgb(0.04, 0.5, 0.55),
        hitbox: Color::rgb(1.0, 0.0, 0.0),
        hurtbox: Color::rgb(0.0, 1.0, 0.0),
        collision_box: Color::rgb(0.0, 0.0, 1.0),
    })
}

fn fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let basic = asset_server.load("FiraSans-Bold.ttf");

    commands.insert_resource(Fonts { basic })
}

fn sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Sprites {
        background_image: asset_server.load("CPT-2018-Stage.png"),
    });
}
