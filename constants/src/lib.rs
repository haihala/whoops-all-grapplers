mod movement;
pub use movement::*;

// Player size
pub const PLAYER_SPRITE_WIDTH: f32 = 0.80;
pub const PLAYER_SPRITE_STANDING_HEIGHT: f32 = 1.80;
pub const PLAYER_SPRITE_CROUCHING_HEIGHT: f32 = PLAYER_SPRITE_STANDING_HEIGHT / 2.0;

// FPS
pub const FPS: f32 = 60.0;
