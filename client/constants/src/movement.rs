// Jumping
// How high should a neutral jump be
const PLAYER_JUMP_HEIGHT: f32 = 2.0;
// How long should a neutral jump take
const PLAYER_JUMP_DURATION: f32 = 1.0;

// Below this are calculated values, tweak above
const PLAYER_JUMP_DURATION_HALVED: f32 = PLAYER_JUMP_DURATION / 2.0;
const PLAYER_JUMP_DURATION_HALVED_SQUARED: f32 =
    PLAYER_JUMP_DURATION_HALVED * PLAYER_JUMP_DURATION_HALVED;

/*
// Math for gravity
x = x0 + v0*t + 1/2*a*t^2

From the apex down
x0 = jump height,
x = 0
v0 = 0

0 = -h + 1/2*a*t^2
1/2*a*t^2 = h
a = 2*h/t^2
*/
pub const PLAYER_GRAVITY_FORCE: f32 =
    2.0 * PLAYER_JUMP_HEIGHT / PLAYER_JUMP_DURATION_HALVED_SQUARED;
pub const PLAYER_GRAVITY_PER_FRAME: f32 = PLAYER_GRAVITY_FORCE / crate::FPS;

/*
Math for initial jump velocity
x = x0 + v0*t + 1/2*a*t^2
From start to end

x0 = 0
x = 0
t and a = known, solve v0

0 = v0*t + 1/2*a*t^2
v0 = -1/2*a*t
*/
pub const NEUTRAL_JUMP_Y: f32 = 0.5 * PLAYER_GRAVITY_FORCE * PLAYER_JUMP_DURATION;

// Values are for an angle of 60 degrees up from horizontal
const DIAGONAL_JUMP_ANGLE_SIN: f32 = 0.866;
const DIAGONAL_JUMP_ANGLE_COS: f32 = 0.5;

pub const DIAGONAL_JUMP_X: f32 = NEUTRAL_JUMP_Y * DIAGONAL_JUMP_ANGLE_COS;
pub const DIAGONAL_JUMP_Y: f32 = NEUTRAL_JUMP_Y * DIAGONAL_JUMP_ANGLE_SIN;

const SUPERJUMP_HEIGHT_MULTIPLIER: f32 = 1.3;

pub const NEUTRAL_SUPERJUMP_Y: f32 = SUPERJUMP_HEIGHT_MULTIPLIER * NEUTRAL_JUMP_Y;
pub const DIAGONAL_SUPERJUMP_X: f32 = SUPERJUMP_HEIGHT_MULTIPLIER * DIAGONAL_JUMP_X;
pub const DIAGONAL_SUPERJUMP_Y: f32 = SUPERJUMP_HEIGHT_MULTIPLIER * DIAGONAL_JUMP_Y;
