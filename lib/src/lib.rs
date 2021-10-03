mod animation;
mod assets;
mod bars;
mod camera;
mod character;
mod clock;
mod constants;
mod damage;
mod inspector;
mod labels;
mod meter;
mod physics;

use bevy::prelude::*;
// Only thing exported out of this crate
pub struct WAGLib;
impl PluginGroup for WAGLib {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group // Order matters here
            .add(labels::StagePlugin)
            .add(assets::AssetsPlugin)
            .add(clock::ClockPlugin)
            .add(animation::AnimationPlugin)
            .add(bars::BarsPlugin)
            .add(camera::CameraPlugin)
            .add(character::PlayerPlugin)
            .add(damage::DamagePlugin)
            .add(inspector::InspectorPlugin)
            .add(physics::PhysicsPlugin);
    }
}

// Make these more easily accessable internally
use assets::{Colors, Fonts, Sprites};
use character::{Player, PlayerState};
use clock::Clock;
use constants::*;
use damage::Health;
use meter::Meter;
