#![feature(drain_filter)]
mod assets;
mod camera;
mod character;
mod damage;
mod inspector;
mod meter;
mod physics;
mod spawner;
mod ui;

use bevy::prelude::*;
// Only thing exported out of this crate
pub struct WAGLib;
impl PluginGroup for WAGLib {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group // Order matters here
            .add(assets::AssetsPlugin)
            .add(ui::UIPlugin)
            .add(camera::CustomCameraPlugin)
            .add(character::PlayerPlugin)
            .add(damage::DamagePlugin)
            .add(inspector::InspectorPlugin)
            .add(physics::PhysicsPlugin)
            .add(spawner::SpawnerPlugin)
            .add(player_state::PlayerStatePlugin)
            .add(time::TimePlugin);
    }
}
