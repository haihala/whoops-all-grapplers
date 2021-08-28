mod assets;
mod camera;
mod clock;
mod player;
mod stages;

pub use assets::AssetsPlugin;
pub use camera::CameraPlugin;
pub use clock::ClockPlugin;
pub use player::PlayerPlugin;
pub use stages::StagePlugin;

pub(crate) use assets::Materials;
pub(crate) use clock::Clock;
