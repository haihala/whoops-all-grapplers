#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

const PI = 3.14159265359;
const cycle_duration = 3.0;
const speed = 1.0;

@group(2) @binding(0) var<uniform> start_time: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time - start_time;
    let cycle = min(1.0, speed * fract(time / cycle_duration));
    let centered = vec2(2.0, -2.0) * (mesh.uv - 0.5);

    let bottom_left = bubbles(centered, 10, cycle, 0.2, 0.3, 0.02, vec2(0.4), vec2(0.0, -0.7), vec2(-0.8, -0.7));
    let bottom_right = bubbles(centered, 10, cycle, 0.2, 0.3, 0.02, vec2(0.3), vec2(0.0, -0.7), vec2(0.8, -0.7));
    let pillar = bubbles(centered, 12, 1.5 * cycle, 0.3, 0.5, 0.02, vec2(0.3), vec2(0.0, -0.5), vec2(0.0, 0.9));
    let base = bubbles(centered, 12, 1.5 * cycle, 0.3, 0.5, 0.02, vec2(1.0), vec2(0.0, -0.5), vec2(0.0, 0.1));

    let combined = bottom_left + bottom_right + pillar + base;
    let field = clamp(0.0, 1.0, combined.x + combined.y);

    let color = vec4(field);

    return color;
}


fn bubbles(centered: vec2f, bubbles: i32, cycle: f32, min_size: f32, max_size: f32, edge: f32, spread: vec2f, start_pos: vec2f, end_pos: vec2f) -> vec2f {
    // Ease out quint
    let fade = 1 - pow(1 - cycle, 5.0);

    var out = vec2(0.0);
    for (var i = 0; i < bubbles; i++) {
        let fi = f32(i);
        let fb = f32(bubbles);
        let part = fi / fb;
        let rand = pow(sin(420.69 * part), 2.0);

        let size = mix(min_size, max_size, part) - max_size * fade;

        let ang = part * 2 * PI;
        let shift_x = cos(ang) * spread.x;
        let shift_y = sin(ang) * spread.y;

        let dest = (end_pos + vec2(shift_x, shift_y)) * rand;
        let current_pos = mix(start_pos, dest, fade);

        let dist = length(current_pos - centered);

        if dist < size - edge * fade {
            out.x = 1.0;
        } else if dist < size {
            out.y = 0.5;
        }
    }
    return out;
}


