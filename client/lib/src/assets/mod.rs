use bevy::prelude::*;

mod animations;
mod announcer;
mod asset_updater;
mod loaders;
mod materials;
mod models;
mod music;
mod vfx;

pub use animations::{AnimationHelper, AnimationHelperSetup, Animations};
pub use announcer::Announcer;
pub use asset_updater::{play_voiceline, start_animation};
pub use materials::{ExtendedFlashMaterial, FlashMaterial};
pub use models::{shake_character, CharacterShake, Models, PlayerModelHook};
pub use music::Music;
pub use vfx::start_relative_vfx;

use foundation::{MatchState, RollbackSchedule, SystemStep};

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
                MaterialPlugin::<materials::SmokeBombMaterial>::default(),
                MaterialPlugin::<materials::MidFlashMaterial>::default(),
                MaterialPlugin::<ExtendedFlashMaterial>::default(),
            ))
            // Limit of plugins per call to add_plugins
            .add_plugins((
                MaterialPlugin::<materials::IconFlashMaterial>::default(),
                MaterialPlugin::<materials::JackpotRingMaterial>::default(),
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
            .add_systems(PostStartup, music::setup_music)
            .add_systems(
                RollbackSchedule,
                (animations::setup_helpers, models::prep_player_gltf)
                    .run_if(in_state(MatchState::Loading))
                    .in_set(SystemStep::SetupPlayerGLTF),
            )
            .add_systems(
                RollbackSchedule,
                (
                    asset_updater::mirror_models,
                    asset_updater::update_generic_animation,
                    models::do_character_shake,
                    animations::update_animation,
                    announcer::update_announcer,
                    music::update_music,
                )
                    .chain()
                    .in_set(SystemStep::Presentation),
            )
            .add_systems(OnEnter(MatchState::Combat), |mut ann: ResMut<Announcer>| {
                ann.fight()
            })
            .add_observer(asset_updater::play_audio)
            .add_observer(vfx::start_absolute_vfx)
            .add_observer(vfx::spawn_vfx::<materials::BlankMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::HitSparkMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::BlankMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::BlockEffectMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::ClashSparkMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::RingRippleMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::LineFieldMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::FocalPointLinesMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::LightningBoltMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::FlatWaveMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::DiagonalWaveMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::PebbleMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::SparkBurstMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::SmokeBombMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::MidFlashMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::JackpotRingMaterial>)
            .add_observer(vfx::spawn_vfx::<materials::IconFlashMaterial>);
    }
}
