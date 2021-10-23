// Basic ground movement
pub const WALK_SPEED: f32 = 3.0;
pub const PLAYER_TOP_SPEED: f32 = 10.0;
pub const MINIMUM_RUN_SPEED: f32 = 5.0;
const ACCELERATION_TIME: f32 = 1.0;

const ACCELERATION_DELTA: f32 = PLAYER_TOP_SPEED - MINIMUM_RUN_SPEED;
pub const PLAYER_ACCELERATION: f32 = ACCELERATION_DELTA / ACCELERATION_TIME / crate::FPS;

// Dashing
const DASH_START_DURATION_SECONDS: f32 = 0.2;
const DASH_RECOVERY_DURATION_SECONDS: f32 = 0.5;
const DASH_DISTANCE: f32 = 4.0;
const DASH_START_DISTANCE_FRACTION: f32 = 0.6;

const SHIFT_DURING_DASH_START: f32 = DASH_DISTANCE * DASH_START_DISTANCE_FRACTION;
const SHIFT_DURING_DASH_RECOVERY: f32 = DASH_DISTANCE * (1.0 - DASH_START_DISTANCE_FRACTION);

pub const DASH_START_SPEED: f32 = SHIFT_DURING_DASH_START / DASH_START_DURATION_SECONDS;
pub const DASH_RECOVERY_SPEED: f32 = SHIFT_DURING_DASH_RECOVERY / DASH_RECOVERY_DURATION_SECONDS;
pub const DASH_START_FRAMES: usize = (DASH_START_DURATION_SECONDS * crate::FPS) as usize;
pub const DASH_WHOLE_FRAMES: usize =
    ((DASH_START_DURATION_SECONDS + DASH_RECOVERY_DURATION_SECONDS) * crate::FPS) as usize;

// Pushing
pub const PUSHING_DEAD_ZONE: f32 = 0.2;

// Drag
const DECELERATION_TIME: f32 = 0.7;

pub const DRAG: f32 = PLAYER_TOP_SPEED / DECELERATION_TIME / crate::FPS;

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
pub const JUMP_VELOCITY: f32 = 0.5 * PLAYER_GRAVITY_FORCE * PLAYER_JUMP_DURATION;
pub const NEUTRAL_JUMP_VECTOR: (f32, f32, f32) = (0.0, JUMP_VELOCITY, 0.0);

// Values are for an angle of 60 degrees up from horizontal
const DIAGONAL_JUMP_ANGLE_SIN: f32 = 0.866;
const DIAGONAL_JUMP_ANGLE_COS: f32 = 0.5;

pub const DIAGONAL_JUMP_VECTOR: (f32, f32, f32) = (
    JUMP_VELOCITY * DIAGONAL_JUMP_ANGLE_COS,
    JUMP_VELOCITY * DIAGONAL_JUMP_ANGLE_SIN,
    0.0,
);
