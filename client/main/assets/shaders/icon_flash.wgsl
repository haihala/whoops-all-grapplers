#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};


// https://bevyengine.org/examples/shaders/array-texture/
@group(2) @binding(0) var<uniform> start_time: f32;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let color = textureSample(texture, texture_sampler, mesh.uv);
    let opacity = max(0.0, 1.0 - pow(2 * (globals.time - start_time), 5.0));
    return vec4(color.xyz, color.a * opacity);
}

