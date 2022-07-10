use bevy::prelude::*;

mod animations;
mod loaders;
mod models;
mod particles;
mod sounds;

pub use animations::{AnimationHelper, AnimationHelperSetup, Animations};
pub use models::{ModelRequest, Models};
pub use particles::{ParticleRequest, Particles};
pub use sounds::Sounds;

pub struct Colors {
    pub notification_text: Color,
    pub notification_background: Color,
    pub health: Color,
    pub meter: Color,
    pub charge_default: Color,
    pub charge_full: Color,
    pub hitbox: Color,
    pub hurtbox: Color,
    pub pushbox: Color,
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
            .add_startup_system_to_stage(StartupStage::PreStartup, loaders::sounds)
            .add_startup_system_to_stage(StartupStage::PreStartup, loaders::particles)
            .add_system(models::model_spawner)
            .add_system(animations::setup_helpers)
            .add_system(animations::update_animation)
            .add_system(sounds::play_queued)
            .add_system(particles::handle_requests);
    }
}
