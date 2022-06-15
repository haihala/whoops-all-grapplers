use bevy::prelude::*;

mod animations;
mod loaders;
mod models;

pub use animations::{AnimationHelper, AnimationHelperSetup, Animations};
pub use models::{Model, ModelRequest, Models};

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

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, loaders::colors)
            .add_startup_system_to_stage(StartupStage::PreStartup, loaders::fonts)
            .add_startup_system_to_stage(StartupStage::PreStartup, loaders::sprites)
            .add_startup_system_to_stage(StartupStage::PreStartup, loaders::models)
            .add_startup_system_to_stage(StartupStage::PreStartup, loaders::animations)
            .add_system(models::model_spawner)
            .add_system(animations::setup_helpers)
            .add_system(animations::update_animation);
    }
}
