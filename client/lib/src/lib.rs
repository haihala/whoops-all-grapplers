#![feature(drain_filter)]
mod assets;
mod camera;
mod damage;
mod dev;
mod economy;
mod physics;
mod player;
mod stage;
mod ui;

use bevy::prelude::*;

// So it can be disabled in integration tests
pub use dev::DevPlugin;
// Only thing exported out of this crate
pub struct WAGLib;
impl PluginGroup for WAGLib {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group // Order matters here, loaded in the defined order
            .add(bevy_hanabi::HanabiPlugin)
            .add(time::TimePlugin) // Has to be first, since it defines labels for ordering other systems
            .add(assets::AssetsPlugin) // Has to be before those assets are used
            .add(ui::UIPlugin)
            .add(camera::CustomCameraPlugin)
            .add(player::PlayerPlugin)
            .add(economy::EconomyPlugin)
            .add(damage::DamagePlugin)
            .add(DevPlugin)
            .add(physics::PhysicsPlugin)
            .add(input_parsing::InputParsingPlugin)
            .add(stage::StagePlugin);
    }
}
