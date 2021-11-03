mod assets;
mod camera;
mod character;
mod clock;
mod damage;
mod frame_data_manager;
mod game_flow;
mod inspector;
mod labels;
mod meter;
mod physics;
mod ui;

use bevy::prelude::*;
// Only thing exported out of this crate
pub struct WAGLib;
impl PluginGroup for WAGLib {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group // Order matters here
            .add(labels::StagePlugin)
            .add(assets::AssetsPlugin)
            .add(clock::ClockPlugin)
            .add(frame_data_manager::AnimationPlugin)
            .add(ui::UIPlugin)
            .add(camera::CameraPlugin)
            .add(character::PlayerPlugin)
            .add(damage::DamagePlugin)
            .add(inspector::InspectorPlugin)
            .add(physics::PhysicsPlugin)
            .add(game_flow::GameFlowPlugin);
    }
}
