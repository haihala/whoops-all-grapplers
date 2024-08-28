#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};


@group(2) @binding(0) var<uniform> start_time: f32;

const PI = 3.14159265359;
const offset = PI * 2 / 3;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time - start_time;
    let centered = 2 * (mesh.uv - 0.5);
    let angle = atan2(centered.x, centered.y);
    let dist = length(centered);

    let flower = 1 - step(sin(angle * 12 + time), 0.98);
    // TODO: Maybe speed this up
    let focal_distance = 1 - pow(time, 4.0);
    let ring = smoothstep(0.0, 1.0, pow(1 - abs(dist - focal_distance), 2.0));
    let core_mask = smoothstep(0.0, 0.1, dist);
    let outer_mask = 1 - step(1.0, dist);

    let field = smoothstep(0.1, 1.0, flower * ring * core_mask * outer_mask);

    return vec4(field);
}


