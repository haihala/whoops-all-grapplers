// I've understood this to be the distance, beyond which the camera stops drawing stuff
pub const CAMERA_FAR_DISTANCE: f32 = 10000.0;
pub const CAMERA_HEIGHT: f32 = 2.0;

// World units (meters) for how high and how wide the viewport should be.
// The eventual value will be 2x this, since the pixels go from -1 to 1 on both axis
pub const VIEWPORT_WIDTH: f32 = 5.0;

pub const BACKGROUND_POSITION: (f32, f32, f32) = (0.0, 5.0, 0.0);
pub const BACKGROUND_SCALE: (f32, f32, f32) = (30.0, 20.0, 1.0);

pub const PLAYER_SPRITE_WIDTH: f32 = 0.80;
pub const PLAYER_SPRITE_HEIGHT: f32 = 1.80;

// TODO: This is wrong. The fps is simply put way more
// However the numbers seem to match which was weird.
// const FRAMES_PER_SECOND: f32 = 60.0; // f32 here to avoid casting elsewhere

pub const GROUND_PLANE_HEIGHT: f32 = 0.0;
pub const ARENA_WIDTH: f32 = 10.0;

pub const PLAYER_WALK_SPEED: f32 = 3.0;
pub const PLAYER_INITIAL_RUN_SPEED: f32 = 5.0;
const PLAYER_ACCELERATION_TIME: f32 = 1.0;
pub const PLAYER_TOP_SPEED: f32 = 10.0;

const PLAYER_RUN_SPEED_DELTA: f32 = PLAYER_TOP_SPEED - PLAYER_INITIAL_RUN_SPEED;

pub const REVERSE_DRAG_MULTIPLIER: f32 = 2.0; // Drag multiplier when pressing in the other direction
const PLAYER_DECELERATION_TIME: f32 = 0.7;
const AIR_DRAG_MULTIPLIER: f32 = 0.0;

pub const PLAYER_ACCELERATION: f32 = PLAYER_RUN_SPEED_DELTA / PLAYER_ACCELERATION_TIME;
pub const GROUND_DRAG: f32 = PLAYER_TOP_SPEED / PLAYER_DECELERATION_TIME;
pub const AIR_DRAG: f32 = GROUND_DRAG * AIR_DRAG_MULTIPLIER;

// Tweak these
const PLAYER_JUMP_HEIGHT: f32 = 2.0;
const PLAYER_JUMP_DURATION: f32 = 1.0;

// Helper
const PLAYER_JUMP_DURATION_HALVED: f32 = PLAYER_JUMP_DURATION / 2.0;
const PLAYER_JUMP_DURATION_HALVED_SQUARED: f32 =
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
pub const PLAYER_GRAVITY: f32 = 2.0 * PLAYER_JUMP_HEIGHT / PLAYER_JUMP_DURATION_HALVED_SQUARED;

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
pub const PLAYER_JUMP_VELOCITY: f32 = PLAYER_JUMP_HEIGHT / PLAYER_JUMP_DURATION_HALVED
    + 0.5 * PLAYER_GRAVITY * PLAYER_JUMP_DURATION_HALVED;

pub const PLAYER_JUMP_VECTOR: (f32, f32, f32) = (0.0, PLAYER_JUMP_VELOCITY, 0.0);

// float.sin() can't be used in const.
// Angle is 60 degrees
const DIAGONAL_JUMP_ANGLE_SIN: f32 = 0.866;
const DIAGONAL_JUMP_ANGLE_COS: f32 = 0.5;

const VERTICAL_JUMP_PART: f32 = PLAYER_JUMP_VELOCITY * DIAGONAL_JUMP_ANGLE_SIN;
const HORIZONTAL_JUMP_PART: f32 = PLAYER_JUMP_VELOCITY * DIAGONAL_JUMP_ANGLE_COS;

pub const PLAYER_LEFT_JUMP_VECTOR: (f32, f32, f32) =
    (-HORIZONTAL_JUMP_PART, VERTICAL_JUMP_PART, 0.0);
pub const PLAYER_RIGHT_JUMP_VECTOR: (f32, f32, f32) =
    (HORIZONTAL_JUMP_PART, VERTICAL_JUMP_PART, 0.0);

pub const ROUND_TIME: f32 = 99.0;

const HEALTH_BAR_TIMER_WIDTH: f32 = 10.0;
const HEALTH_BAR_EDGE_PADDING: f32 = 5.0;
pub const HEALTH_BAR_ANCHOR: f32 = 50.0 + (HEALTH_BAR_TIMER_WIDTH / 2.0);
pub const RESOURCE_BAR_WIDTH: f32 =
    (100.0 - HEALTH_BAR_TIMER_WIDTH - HEALTH_BAR_EDGE_PADDING) / 2.0;
