#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> edge_color: vec4<f32>;
@group(2) @binding(2) var<uniform> speed: f32;
@group(2) @binding(3) var<uniform> start_time: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Coordinate relative to middle
    let centered = 2 * (mesh.uv - 0.5);
    let angle = atan2(centered.x, centered.y);
    let time = globals.time - start_time;

    let range = length(centered);
    let ring = fract(speed * time);

    let sins = 0.25 * sin(angle + 15 * time);
    var field = 2 * (pow(1 - abs(ring - range), 5.0) - 0.5) - (2 * range) + sins;
    field = clamp(field, -0.1, 1.0);
    let color = (1 - field) * edge_color.xyz + (field) * base_color.xyz;

    return vec4(color, step(0.0, field));
}

