mod assets;
mod camera;
mod character;
mod clock;
mod constants;
mod input;
mod labels;
mod physics;
mod player;

// Get these out to main.rs
pub use assets::AssetsPlugin;
pub use camera::CameraPlugin;
pub use clock::ClockPlugin;
pub use labels::StagePlugin;
pub use physics::PhysicsPlugin;
pub use player::PlayerPlugin;

// Make these more easily accessable internally
pub(crate) use assets::Materials;
pub(crate) use clock::Clock;
pub(crate) use player::Player;
