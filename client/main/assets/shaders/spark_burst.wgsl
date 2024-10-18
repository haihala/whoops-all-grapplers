#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

const PI = 3.14159265359;
const cycle_duration = 3.0;
const speed = 1.0;

@group(2) @binding(0) var<uniform> start_time: f32;
@group(2) @binding(1) var<uniform> inner_color: vec4<f32>;
@group(2) @binding(2) var<uniform> border_color: vec4<f32>;
@group(2) @binding(3) var<uniform> mirror: f32;

const gravity = 8.0;
const land_speed_loss = 0.5;
const elasticity = 0.4;

const min_angle = PI / 6;
const max_angle = PI * 0.3;

const min_velocity = 1.8;
const max_velocity = 5.0;
const size_vel_influence = 0.5;
const deceleration_x = 0.2;

const min_size = 0.02;
const max_size = 0.04;
const shrink_speed = 0.08;

const min_start = 0.0;
const max_start = 0.2;

const border = 0.2;

const seed = 420;
const amount = 40;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time - start_time;
    let cycle = min(1.0, speed * fract(time / cycle_duration));
    let coords = (mesh.uv - vec2(mirror, 1.0)) * vec2((1.0 - 2.0 * mirror), -1.0);

    var field = 0.0;

    for (var i = seed; i < seed + amount; i++) {
        let size_rand = rand(i, 2.345);
        let size = max(0.0, size_rand * (max_size - min_size) + min_size - shrink_speed * cycle);
        let size_offset = vec2(0.0, size);
        let front_rock = ball(i, cycle + 0.01, size_rand) + size_offset;
        let back_rock = ball(i, cycle, size_rand) + size_offset;
        let norm = normalize(back_rock - front_rock);

        let front_dist = 0.5 * length(coords - front_rock);
        let back_dist = 0.5 * length(coords - (front_rock + 0.07 * norm * size_rand));
        let dist = front_dist + back_dist;

        field += step(dist, size);
    }

    if field <= 0.0 {
        return vec4(0.0);
    }

    let color = inner_color.xyz * field + border_color.xyz * (1 - field);
    return vec4(color, 1.0);
}

fn ball(i: i32, t: f32, size_rand: f32) -> vec2f {
    let angle = rand(i, 1.234) * (max_angle - min_angle) + min_angle;
    let size_influence = max(0.0, 1.0 - size_vel_influence * pow(size_rand, 10.0));
    let velocity = (rand(i, 3.456) * (max_velocity - min_velocity) + min_velocity) * size_influence;
    let start_x = rand(i, 4.567) * (max_start - min_start) + min_start;
    let start_y = rand(i, 5.678) * (max_start - min_start) + min_start;

    let launch = vec2(cos(angle), sin(angle)) * velocity;

    let rock_x = start_x + launch.x * t;
    let rock_y = launch.y * t - gravity * pow(t, 2.0) / 2.0 + start_y;

    let rock_pos = vec2(rock_x, rock_y);
    return rock_pos;
}

fn rand(index: i32, mul: f32) -> f32 {
    let x = f32(index) - mul;
    let y = f32(index) * mul;
    return fract(sin(x * 12.9898 + y * 78.233) * 43758.5453);
}

