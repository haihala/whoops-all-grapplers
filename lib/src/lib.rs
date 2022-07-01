#![feature(drain_filter)]
mod assets;
mod camera;
mod damage;
mod dev;
mod physics;
mod player;
mod spawner;
mod stage;
mod ui;

use bevy::prelude::*;
// Only thing exported out of this crate
pub struct WAGLib;
impl PluginGroup for WAGLib {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group // Order matters here, loaded in the defined order
            .add(time::TimePlugin) // Has to be first, since it defines labels for ordering other systems
            .add(assets::AssetsPlugin) // Has to be before those assets are used
            .add(ui::UIPlugin)
            .add(camera::CustomCameraPlugin)
            .add(player::PlayerPlugin)
            .add(damage::DamagePlugin)
            .add(dev::DevPlugin)
            .add(physics::PhysicsPlugin)
            .add(spawner::SpawnerPlugin)
            .add(input_parsing::InputParsingPlugin)
            .add(stage::StagePlugin)
            .add(player_state::PlayerStatePlugin);
    }
}
