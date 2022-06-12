use bevy::{gltf::Gltf, prelude::*};

pub struct Colors {
    pub health: Color,
    pub meter: Color,
    pub charge_default: Color,
    pub charge_full: Color,
    pub hitbox: Color,
    pub hurtbox: Color,
    pub collision_box: Color,
    pub text: Color,
}

pub struct Fonts {
    pub basic: Handle<Font>,
}

pub struct Sprites {
    pub background_image: Handle<Image>,
}

pub struct Models {
    pub ryan: Handle<Gltf>,
}
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, colors)
            .add_startup_system_to_stage(StartupStage::PreStartup, fonts)
            .add_startup_system_to_stage(StartupStage::PreStartup, sprites)
            .add_startup_system_to_stage(StartupStage::PreStartup, models);
    }
}

fn colors(mut commands: Commands) {
    commands.insert_resource(Colors {
        health: Color::rgb(0.9, 0.0, 0.0),
        meter: Color::rgb(0.04, 0.5, 0.55),
        charge_default: Color::rgb(0.05, 0.4, 0.55),
        charge_full: Color::rgb(0.9, 0.1, 0.3),
        hitbox: Color::rgb(1.0, 0.0, 0.0),
        hurtbox: Color::rgb(0.0, 1.0, 0.0),
        collision_box: Color::rgb(0.0, 0.0, 1.0),
        text: Color::WHITE,
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

fn models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Models {
        ryan: asset_server.load("dummy-character.glb"),
    });
}
