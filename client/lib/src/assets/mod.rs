use bevy::{prelude::*, utils::HashMap};

mod animations;
mod announcer;
mod asset_updater;
mod loaders;
mod materials;
mod models;
mod sounds;
mod vfx;

pub use animations::{AnimationHelper, AnimationHelperSetup, Animations};
pub use announcer::Announcer;
pub use asset_updater::{play_voiceline, start_animation};
pub use materials::{ExtendedFlashMaterial, FlashMaterial};
pub use models::{Models, PlayerModelHook};
pub use sounds::Sounds;
pub use vfx::start_relative_vfx;

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
            .init_resource::<Announcer>()
            .add_plugins((
                MaterialPlugin::<materials::HitSparkMaterial>::default(),
                MaterialPlugin::<materials::BlankMaterial>::default(),
                MaterialPlugin::<materials::BlockEffectMaterial>::default(),
                MaterialPlugin::<materials::ClashSparkMaterial>::default(),
                MaterialPlugin::<materials::RingRippleMaterial>::default(),
                MaterialPlugin::<materials::LineFieldMaterial>::default(),
                MaterialPlugin::<materials::FocalPointLinesMaterial>::default(),
                MaterialPlugin::<materials::LightningBoltMaterial>::default(),
                MaterialPlugin::<materials::FlatWaveMaterial>::default(),
                MaterialPlugin::<materials::DiagonalWaveMaterial>::default(),
                MaterialPlugin::<materials::PebbleMaterial>::default(),
                MaterialPlugin::<materials::SparkBurstMaterial>::default(),
                MaterialPlugin::<materials::MidFlashMaterial>::default(),
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
                    announcer::update_announcer,
                )
                    .chain()
                    .in_set(WAGStage::Presentation),
            )
            .add_systems(OnEnter(MatchState::PreRound), announcer::preround)
            .add_systems(OnEnter(MatchState::Combat), announcer::combat)
            .observe(asset_updater::play_audio)
            .observe(vfx::start_absolute_vfx)
            .observe(vfx::spawn_vfx::<materials::BlankMaterial>)
            .observe(vfx::spawn_vfx::<materials::HitSparkMaterial>)
            .observe(vfx::spawn_vfx::<materials::BlankMaterial>)
            .observe(vfx::spawn_vfx::<materials::BlockEffectMaterial>)
            .observe(vfx::spawn_vfx::<materials::ClashSparkMaterial>)
            .observe(vfx::spawn_vfx::<materials::RingRippleMaterial>)
            .observe(vfx::spawn_vfx::<materials::LineFieldMaterial>)
            .observe(vfx::spawn_vfx::<materials::FocalPointLinesMaterial>)
            .observe(vfx::spawn_vfx::<materials::LightningBoltMaterial>)
            .observe(vfx::spawn_vfx::<materials::FlatWaveMaterial>)
            .observe(vfx::spawn_vfx::<materials::DiagonalWaveMaterial>)
            .observe(vfx::spawn_vfx::<materials::PebbleMaterial>)
            .observe(vfx::spawn_vfx::<materials::SparkBurstMaterial>)
            .observe(vfx::spawn_vfx::<materials::MidFlashMaterial>);
    }
}
