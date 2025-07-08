#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> edge_color: vec4<f32>;
@group(2) @binding(2) var<uniform> duration: f32;
@group(2) @binding(3) var<uniform> ring_thickness: f32;
@group(2) @binding(4) var<uniform> start_time: f32;
@group(2) @binding(5) var<uniform> rings: i32;
@group(2) @binding(6) var<uniform> offset: f32;


#import "shaders/helpers.wgsl"::{PI, easeOutQuint}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let centered = 2 * (mesh.uv - 0.5);
    let normdist = length(centered);
    let time = (globals.time - start_time) /  duration;  // Normalized time

    let half_ring = ring_thickness / 2.0;

    var color = vec4(0.0);

    for (var i: i32 = 0; i < rings; i++) {
        let phase = easeOutQuint(clamp((time - f32(i) * offset) / (1.0 - f32(rings-1) * offset), 0.0, 1.0));
        let dist = abs(phase - normdist);

        var alpha = 0.0;
        if dist < half_ring && phase <= 1.0 {
            let distance_fade = 1 - normdist;
            // Technically, this should be 1.0, but the 1.2 makes for a less sharp
            // falloff and the mid color is more pronounced
            let sdf_fade = 1.2 - (dist / half_ring);
            alpha = distance_fade * sdf_fade;
        }

        color += alpha * mix(edge_color, base_color, dist / half_ring);
    }

    return color;
}
 
