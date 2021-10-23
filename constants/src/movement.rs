// Ground movement
pub const PLAYER_WALK_SPEED: f32 = 3.0;
pub const PLAYER_DASH_SPEED: f32 = 10.0;
pub const PLAYER_TOP_SPEED: f32 = 10.0;
pub const PLAYER_INITIAL_RUN_SPEED: f32 = 5.0;
const PLAYER_ACCELERATION_TIME: f32 = 1.0;

const PLAYER_ACCELERATION_DELTA: f32 = PLAYER_TOP_SPEED - PLAYER_INITIAL_RUN_SPEED;
pub const PLAYER_ACCELERATION: f32 =
    PLAYER_ACCELERATION_DELTA / PLAYER_ACCELERATION_TIME / crate::FPS;

// Drag
pub const REVERSE_DRAG_MULTIPLIER: f32 = 2.0; // Drag multiplier when pressing in the other direction
const PLAYER_DECELERATION_TIME: f32 = 0.7;
const AIR_DRAG_MULTIPLIER: f32 = 0.0;
pub const GROUND_DRAG: f32 = PLAYER_TOP_SPEED / PLAYER_DECELERATION_TIME / crate::FPS;
pub const AIR_DRAG: f32 = GROUND_DRAG * AIR_DRAG_MULTIPLIER;

// Jumping
const PLAYER_JUMP_HEIGHT: f32 = 2.0;
const PLAYER_JUMP_DURATION: f32 = 1.0;

// Below this are calculated jumping values
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
pub const PLAYER_JUMP_VELOCITY: f32 = 0.5 * PLAYER_GRAVITY_FORCE * PLAYER_JUMP_DURATION;
pub const PLAYER_JUMP_VECTOR: (f32, f32, f32) = (0.0, PLAYER_JUMP_VELOCITY, 0.0); // Neutral jump

// float.sin() can't be used in const.
// Values are for an angle of 60 degrees up from horizontal
const DIAGONAL_JUMP_ANGLE_SIN: f32 = 0.866;
const DIAGONAL_JUMP_ANGLE_COS: f32 = 0.5;

const VERTICAL_JUMP_PART: f32 = PLAYER_JUMP_VELOCITY * DIAGONAL_JUMP_ANGLE_SIN;
const HORIZONTAL_JUMP_PART: f32 = PLAYER_JUMP_VELOCITY * DIAGONAL_JUMP_ANGLE_COS;

// Diagonal jumps
pub const PLAYER_LEFT_JUMP_VECTOR: (f32, f32, f32) =
    (-HORIZONTAL_JUMP_PART, VERTICAL_JUMP_PART, 0.0);
pub const PLAYER_RIGHT_JUMP_VECTOR: (f32, f32, f32) =
    (HORIZONTAL_JUMP_PART, VERTICAL_JUMP_PART, 0.0);
