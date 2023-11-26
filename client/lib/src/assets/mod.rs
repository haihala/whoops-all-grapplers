use bevy::prelude::*;

mod animations;
mod loaders;
mod models;
mod particles;
mod sounds;

pub use animations::{AnimationHelper, AnimationHelperSetup, Animations};
pub use models::Models;
pub use particles::{ParticleRequest, Particles};
pub use sounds::Sounds;

#[derive(Debug, Resource)]
pub struct Colors {
    pub notification_text: Color,
    pub notification_background: Color,
    pub hitbox: Color,
    pub hurtbox: Color,
    pub pushbox: Color,
    pub text: Color,
    pub default_item_slot: Color,
    pub highlighted_item_slot: Color,
    pub disabled_item_slot: Color,
    pub shop_timer_background: Color,
}

#[derive(Debug, Resource)]
pub struct Fonts {
    pub basic: Handle<Font>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (
                loaders::colors,
                loaders::fonts,
                loaders::models,
                loaders::animations,
                loaders::sounds,
                loaders::particles,
            ),
        )
        .add_systems(
            Update,
            (
                animations::setup_helpers,
                animations::update_animation,
                animations::mirror_after_load,
                models::find_joints,
                sounds::play_queued,
                particles::handle_requests,
            ),
        );
    }
}
