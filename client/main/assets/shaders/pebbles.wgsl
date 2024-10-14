#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

const PI = 3.14159265359;
const cycle_duration = 3.0;
const speed = 1.0;

@group(2) @binding(0) var<uniform> start_time: f32;
@group(2) @binding(1) var<uniform> inner_color: vec4<f32>;
@group(2) @binding(2) var<uniform> border_color: vec4<f32>;
@group(2) @binding(3) var<uniform> mirror: f32;

const gravity = 9.0;
const friction = 0.8;
const land_speed_loss = 0.5;
const elasticity = 0.4;

const min_angle = PI / 8;
const max_angle = PI * 0.35;

const min_velocity = 1.0;
const max_velocity = 2.5;
const size_vel_influence = 0.5;

const min_size = 0.007;
const max_size = 0.025;

const min_start = 0.0;
const max_start = 0.1;

const border = 0.5;

const seed = 420;
const amount = 30;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time - start_time;
    let cycle = min(1.0, speed * fract(time / cycle_duration));
    let coords = (mesh.uv - vec2(mirror, 1.0)) * vec2((1.0 - 2.0 * mirror), -1.0);

    var field = vec3(0.0);

    for (var i = seed; i < seed + amount; i++) {
        let angle = rand(i, 1.234) * (max_angle - min_angle) + min_angle;
        let size_rand = rand(i, 2.345);
        let size = size_rand * (max_size - min_size) + min_size;
        let size_influence = max(0.0, 1.0 - size_vel_influence * pow(size_rand, 10.0));
        let velocity = (rand(i, 3.456) * (max_velocity - min_velocity) + min_velocity) * size_influence;
        let start_x = rand(i, 4.567) * (max_start - min_start) + min_start;
        let start_y = rand(i, 5.678) * (max_start - min_start) + min_start;

        let launch = vec2(cos(angle), sin(angle)) * velocity;

        // First bounce time
        let disc = sqrt(pow(launch.y, 2.0) + 2 * start_y * gravity);
        let t0 = (launch.y + disc) / gravity;

        var rock_y = 0.0;
        var rock_x = 0.0;

        rock_x = start_x + launch.x * min(cycle, t0);
        if cycle < t0 {
            // On first bounce
            rock_y = launch.y * cycle - gravity * pow(cycle, 2.0) / 2.0 + start_y;
        } else {
            // On subsequent bounces
            let impact_velocity = gravity * t0 - launch.y;
            var total_t = t0;
            for (var bounce = 1; bounce < 4; bounce++) {
                let bounce_vel = pow(elasticity, f32(bounce)) * impact_velocity;
                let tb = 2 * bounce_vel / gravity;
                let bounce_t = cycle - total_t;

                if bounce_t > tb {
                    // This bounce has been passed
                    rock_x += launch.x * pow(land_speed_loss, f32(bounce)) * tb;
                    total_t += tb;
                } else {
                    // Yet to complete this bounce
                    rock_y = bounce_vel * bounce_t - gravity * pow(bounce_t, 2.0) / 2.0;
                    rock_x += launch.x * pow(land_speed_loss, f32(bounce)) * bounce_t;
                    break;
                }
            }
        }

        let rock_pos = vec2(rock_x, rock_y + size);
        let dist = length(coords - rock_pos);
        if field.x == 0 {
            field.y += step(abs(dist - size), size * border);
        }
        field.x += step(dist, size);
    }

    if field.x <= 0.01 {
        return vec4(0.0);
    }

    let fade = 1 - pow(cycle, 5.0);
    if field.y > 0.0 {
        return vec4(border_color.xyz, fade);
    }

    return vec4(inner_color.xyz, fade);
}

fn rand(index: i32, mul: f32) -> f32 {
    let x = f32(index) - mul;
    let y = f32(index) * mul;
    return fract(sin(x * 12.9898 + y * 78.233) * 43758.5453);
}

