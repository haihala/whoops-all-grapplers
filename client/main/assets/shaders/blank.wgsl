#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4(1.0);
}

