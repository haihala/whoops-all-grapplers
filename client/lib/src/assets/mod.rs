use bevy::prelude::*;

mod animations;
mod loaders;
mod models;
mod particles;
mod sounds;

pub use animations::{AnimationHelper, AnimationHelperSetup, AnimationRequest, Animations};
pub use models::Models;
pub use particles::{ParticleRequest, Particles};
pub use sounds::Sounds;

#[derive(Debug, Resource)]
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
    pub default_item_slot: Color,
    pub highlighted_item_slot: Color,
    pub disabled_item_slot: Color,
}

#[derive(Debug, Resource)]
pub struct Fonts {
    pub basic: Handle<Font>,
}

#[derive(Debug, Resource)]
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
            .add_system(animations::setup_helpers)
            .add_system(animations::update_animation)
            .add_system(animations::mirror_after_load)
            .add_system(models::find_joints)
            .add_system(sounds::play_queued)
            .add_system(particles::handle_requests);
    }
}
