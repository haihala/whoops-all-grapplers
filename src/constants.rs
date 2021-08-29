pub static PLAYER_SPRITE_WIDTH: f32 = 10.0;
pub static PLAYER_SPRITE_HEIGHT: f32 = 15.0;

pub static INPUT_BUFFER_LENGTH: usize = 60;
pub static RECENT_INPUT_FRAMES: usize = 5;

pub static GROUND_PLANE_HEIGHT: f32 = -1.0;
pub static ARENA_WIDTH: f32 = 600.0;

static PLAYER_ACCELERATION_TIME: f32 = 1.0;
static PLAYER_DECELERATION_TIME: f32 = 1.0;
static AIR_DRAG_MULTIPLIER: f32 = 0.5;

pub static PLAYER_TOP_SPEED: f32 = 200.0;

pub static PLAYER_ACCELERATION: f32 = PLAYER_TOP_SPEED / PLAYER_ACCELERATION_TIME;
pub static GROUND_DRAG: f32 = PLAYER_TOP_SPEED / PLAYER_DECELERATION_TIME;
pub static AIR_DRAG: f32 = GROUND_DRAG * AIR_DRAG_MULTIPLIER;

// Tweak these
static PLAYER_JUMP_HEIGHT: f32 = 200.0;
static PLAYER_JUMP_DURATION: f32 = 1.0;

// Helper
static PLAYER_JUMP_DURATION_HALVED: f32 = PLAYER_JUMP_DURATION / 2.0;
static PLAYER_JUMP_DURATION_HALVED_SQUARED: f32 =
    PLAYER_JUMP_DURATION_HALVED * PLAYER_JUMP_DURATION_HALVED;

/*
x = x0 + v0*t + 1/2*a*t^2

From the apex down
x0 = jump height,
x = 0
v0 = 0

0 = -h + 1/2*a*t^2
1/2*a*t^2 = h
a = 2*h/t^2
*/
pub static PLAYER_GRAVITY: f32 = 2.0 * PLAYER_JUMP_HEIGHT / PLAYER_JUMP_DURATION_HALVED_SQUARED;

/*
x = x0 + v0*t + 1/2*a*t^2

From start to apex
x0 = 0
x = h

h = v0*t + 1/2*a*t^2
H - 1/2*a*t^2 = v0*t
(h - 1/2*a*t^2)/t = v0

v0 = (h - 1/2*a*t^2)/t
v0 = h/t - 1/2*a*t
*/
pub static PLAYER_JUMP_VELOCITY: f32 = PLAYER_JUMP_HEIGHT / PLAYER_JUMP_DURATION_HALVED
    + 0.5 * PLAYER_GRAVITY * PLAYER_JUMP_DURATION_HALVED;

// [src\character\ryan.rs:40] crate::constants::PLAYER_GRAVITY = 0.000625
// [src\character\ryan.rs:41] crate::constants::PLAYER_JUMP_VELOCITY = 0.0003125
