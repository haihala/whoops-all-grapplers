use bevy::prelude::*;

mod animations;
mod loaders;
mod models;
mod particles;
mod sounds;

pub use animations::{AnimationHelper, AnimationHelperSetup, Animations};
pub use models::{Models, UpdateMaterial};
pub use particles::{ParticleRequest, Particles};
pub use sounds::Sounds;

use wag_core::GameState;

#[derive(Debug, Resource)]
pub struct Fonts {
    pub basic: Handle<Font>,
}

#[derive(Resource, Debug, Default)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsLoading>()
            .add_systems(
                PreStartup,
                (
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
                    animations::mirror_after_load,
                    models::prep_player_gltf,
                )
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(
                Update,
                (
                    animations::update_animation,
                    sounds::play_queued,
                    particles::handle_requests,
                ),
            );
    }
}
