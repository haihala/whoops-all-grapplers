#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> mid_color: vec4<f32>;
@group(2) @binding(2) var<uniform> edge_color: vec4<f32>;
@group(2) @binding(3) var<uniform> start_time: f32;

const PI = 3.14159265359;
const offset = PI * 2 / 3;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Coordinate relative to middle
    let centered = 2 * (mesh.uv - 0.5);
    let time = globals.time - start_time;

    let angle = atan2(centered.x, centered.y) + 0.1 * time;
    let base_wave = wave(angle, 1.0, 1.0, 7);
    let secondary_wave = wave(angle + 3 * time, 0.0, 3.0, 5);
    let tertiary_wave = wave(angle, 0.0, 1.0, 1);
    let wave_field = pow(0.9 * base_wave + 0.1 * secondary_wave + 1.0 * tertiary_wave, 2.0);

    let range = length(centered) / sqrt(2.0);
    let radius_field = pow(1 - range, 5.0);

    let field = (pow(0.2 * wave_field, 3.0) + 10.0 * radius_field) * pow(1 - range, 5.0);
    var color = vec3(0.0);
    if field > 5.0 {
        color = base_color.xyz;
    } else if field > 1.5 {
        color = mid_color.xyz;
    } else {
        color = edge_color.xyz;
    }

    return vec4(color, step(0.9, field));
}

fn wave(input: f32, start: f32, increment: f32, loops: i32) -> f32 {
    var total = 0.0;
    var num = start;
    for (var i: i32 = 0; i < loops; i++) {
        num += increment;
        total += 1 - abs(sin(num * (num + input)));
    }
    return total;
}

