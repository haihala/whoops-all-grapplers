#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

const PI = 3.14159265359;
const cycle_duration = 2.0;
const speed = 5.0;

const angle_sharpness = 1.5;

@group(2) @binding(0) var<uniform> start_time: f32;
@group(2) @binding(1) var<uniform> color: vec4f;
@group(2) @binding(2) var<uniform> mirror: i32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time - start_time;
    let cycle = min(1.0, speed * fract(time / cycle_duration));

    var x = mesh.uv.x;
    if mirror == 1 {
        x = 1 - mesh.uv.x;
    }

    let angle = atan2(x, mesh.uv.y);                // 0 to -PI/2
    let norm_angle = 1 - 2 * abs((angle * 2.0 / PI) - 0.5); // 0 to 1
    let angle_falloff = pow(norm_angle, angle_sharpness);
    let time_falloff = 1 - pow(cycle, 5.0);
    let falloff = angle_falloff * time_falloff;

    let dist = length(vec2(x, mesh.uv.y));
    let wave = pow(1 - abs(dist - (ease(cycle * 0.4) * 0.15 + 0.8)), 40.0);
    let field = falloff * wave;

    return vec4(color.xyz, field);
}

fn ease(i: f32) -> f32 {
    return 1 - pow(1 - i, 5.0);
}

