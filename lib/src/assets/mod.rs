use bevy::{gltf::Gltf, prelude::*};

mod animations;
mod loaders;
mod models;

pub use animations::AnimationRequest;
pub use models::ModelRequest;

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
        app.add_startup_system_to_stage(StartupStage::PreStartup, loaders::colors)
            .add_startup_system_to_stage(StartupStage::PreStartup, loaders::fonts)
            .add_startup_system_to_stage(StartupStage::PreStartup, loaders::sprites)
            .add_startup_system_to_stage(StartupStage::PreStartup, loaders::models)
            .add_system(models::model_spawner)
            .add_system(animations::animation_starter);
    }
}
