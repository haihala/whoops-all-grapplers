use bevy::{prelude::*, utils::HashMap};

mod animations;
mod asset_updater;
mod loaders;
mod materials;
mod models;
mod sounds;
mod vfx;

pub use animations::{AnimationHelper, AnimationHelperSetup, Animations};
pub use asset_updater::{start_animation, start_vfx};
pub use materials::{ExtendedFlashMaterial, FlashMaterial};
pub use models::{Models, PlayerModelHook};
pub use sounds::Sounds;
pub use vfx::Vfx;

use wag_core::{Icon, MatchState, RollbackSchedule, WAGStage};

#[derive(Debug, Resource)]
pub struct Fonts {
    pub basic: Handle<Font>,
}

#[derive(Debug, Resource)]
pub struct Icons(pub HashMap<Icon, Handle<Image>>);

#[derive(Resource, Debug, Default)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsLoading>()
            .add_plugins((
                MaterialPlugin::<materials::HitSparkMaterial>::default(),
                MaterialPlugin::<materials::BlankMaterial>::default(),
                MaterialPlugin::<materials::BlockEffectMaterial>::default(),
                MaterialPlugin::<materials::ClashSparkMaterial>::default(),
                MaterialPlugin::<materials::RingRippleMaterial>::default(),
                MaterialPlugin::<materials::LineFieldMaterial>::default(),
                MaterialPlugin::<materials::FocalPointLinesMaterial>::default(),
                MaterialPlugin::<materials::LightningBoltMaterial>::default(),
                MaterialPlugin::<ExtendedFlashMaterial>::default(),
            ))
            .add_systems(
                Startup,
                (
                    loaders::fonts,
                    loaders::icons,
                    loaders::models,
                    loaders::animations,
                    loaders::sounds,
                    loaders::vfx,
                ),
            )
            .add_systems(
                Update,
                (
                    animations::setup_helpers,
                    animations::mirror_after_load,
                    models::prep_player_gltf,
                )
                    .run_if(in_state(MatchState::Loading)),
            )
            .add_systems(
                RollbackSchedule,
                (
                    asset_updater::clear_empty_audio_players,
                    asset_updater::update_generic_animation,
                    animations::update_animation,
                    vfx::handle_requests,
                )
                    .chain()
                    .in_set(WAGStage::Presentation),
            )
            .observe(asset_updater::play_audio);
    }
}
