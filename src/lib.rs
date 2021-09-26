mod assets;
mod bars;
mod camera;
mod character;
mod clock;
mod constants;
mod input;
mod inspector;
mod labels;
mod physics;
mod player;

// Get these out to main.rs
pub use assets::AssetsPlugin;
pub use bars::BarsPlugin;
pub use camera::CameraPlugin;
pub use clock::ClockPlugin;
pub use inspector::InspectorPlugin;
pub use labels::StagePlugin;
pub use physics::PhysicsPlugin;
pub use player::PlayerPlugin;

// Make these more easily accessable internally
pub(crate) use assets::{Colors, Fonts, Sprites};
pub(crate) use clock::Clock;
pub(crate) use constants::*;
pub(crate) use player::{Health, Meter, Player};
