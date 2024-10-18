#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

const PI = 3.14159265359;
const cycle_duration = 2.0;
const speed = 2.0;

const edge_sharpness = 2.0;

@group(2) @binding(0) var<uniform> start_time: f32;
@group(2) @binding(1) var<uniform> color: vec4f;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time - start_time;
    let cycle = min(1.0, speed * fract(time / cycle_duration));

    let edge_falloff = pow(1 - abs(mesh.uv.y * 2 - 1.0), edge_sharpness);
    let time_falloff = 1 - pow(cycle, 5.0);
    let falloff = edge_falloff * time_falloff;

    let dist = length(mesh.uv - vec2(0.0, 0.5));
    let wave = pow(1 - abs(dist - (ease(cycle * 0.4) * 0.15 + 0.6)), 40.0);
    let field = falloff * wave;

    return vec4(color.xyz, field);
}

fn ease(i: f32) -> f32 {
    return 1 - pow(1 - i, 5.0);
}

