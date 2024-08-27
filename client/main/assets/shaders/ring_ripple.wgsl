#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> edge_color: vec4<f32>;
@group(2) @binding(2) var<uniform> duration: f32;
@group(2) @binding(3) var<uniform> ring_thickness: f32;
@group(2) @binding(4) var<uniform> start_time: f32;

const PI = 3.14159265359;
const offset = PI * 2 / 3;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Coordinate relative to middle
    let centered = 2 * (mesh.uv - 0.5);
    let time = easeOutQuint((globals.time - start_time) / duration);

    let normdist = length(centered);

    let half_ring = ring_thickness / 2.0;
    let ring_midpoint = normdist + half_ring;

    let t = abs(time - ring_midpoint);
    var alpha = 0.0;
    if t < half_ring {
        let distance_fade = 1 - time;
        // Technically, this should be 1.0, but the 1.2 makes for a less sharp
        // falloff and the mid color is more pronounced
        let sdf_fade = 1.2 - (t / half_ring);
        alpha = distance_fade * sdf_fade;
    } else {
        alpha = 0.0;
    }
    var color = lerp(t / half_ring, edge_color, base_color);
    color.a *= alpha;
    return color;
}

fn lerp(t: f32, c1: vec4<f32>, c2: vec4<f32>) -> vec4<f32> {
    return t * c1 + (1 - t) * c2;
}

fn easeOutQuint(x: f32) -> f32 {
    return 1 - pow(1 - x, 5.0);
}

