mod movement;
pub use movement::*;

// Inputs
pub const MAX_SECONDS_BETWEEN_SUBSEQUENT_MOTIONS: f32 = 0.1; // In seconds
pub const EVENT_REPEAT_PERIOD: f32 = 0.3; // In seconds
pub const STICK_DEAD_ZONE: f32 = 0.2;

// Camera
// I've understood this to be the distance, beyond which the camera stops drawing stuff
pub const CAMERA_FAR_DISTANCE: f32 = 10000.0;
pub const CAMERA_HEIGHT: f32 = 2.0;

// World units (meters) for how high and how wide the viewport should be.
// The eventual value will be 2x this, since the pixels go from -1 to 1 on both axis
pub const VIEWPORT_WIDTH: f32 = 5.0;

// Background
pub const BACKGROUND_POSITION: (f32, f32, f32) = (0.0, 5.0, 0.0);
pub const BACKGROUND_SCALE: (f32, f32, f32) = (30.0, 20.0, 1.0);

// Player size
pub const PLAYER_SPRITE_WIDTH: f32 = 0.80;
pub const PLAYER_SPRITE_HEIGHT: f32 = 1.80;
pub const PLAYER_COLLIDER_SIZE: (f32, f32) = (PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT);

// FPS
pub const FPS: f32 = 60.0;
pub const FPS_F64: f64 = FPS as f64;

// Damage
pub const CHIP_DAMAGE_MULTIPLIER: f32 = 0.01;

// Spawn point
pub const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)
pub const PLAYER_SPAWN_HEIGHT: f32 = GROUND_PLANE_HEIGHT + 0.001;

// Arena
pub const GROUND_PLANE_HEIGHT: f32 = 0.6;
pub const ARENA_WIDTH: f32 = 10.0;

// UI
const HEALTH_BAR_TIMER_WIDTH: f32 = 10.0;
const HEALTH_BAR_EDGE_PADDING: f32 = 5.0;
pub const HEALTH_BAR_ANCHOR: f32 = 50.0 + (HEALTH_BAR_TIMER_WIDTH / 2.0);
pub const RESOURCE_BAR_WIDTH: f32 =
    (100.0 - HEALTH_BAR_TIMER_WIDTH - HEALTH_BAR_EDGE_PADDING) / 2.0;

// Other
pub const ROUND_TIME: f32 = 99.0;
