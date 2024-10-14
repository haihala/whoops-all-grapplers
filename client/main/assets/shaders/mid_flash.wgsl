#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

const PI = 3.14159265359;
const cycle_duration = 2.0;
const anim_duration = 0.5;


@group(2) @binding(0) var<uniform> start_time: f32;
@group(2) @binding(1) var<uniform> inner_color: vec4<f32>;
@group(2) @binding(2) var<uniform> outer_color: vec4<f32>;

const main_thickness = 0.1;

const secondary_start = vec2(0.01, 0.015);
const secondary_end = vec2(0.6, 0.9);
const secondary_movement = vec2(0.1, 0.2);
const secondary_thickness = 0.1;

const edge = 0.03;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time - start_time;
    let cycle = min(1.0, cycle_duration * fract(time / cycle_duration) / anim_duration);

    let a = pow(cycle + 0.3, 5.0) ;
    let b = 1.1 - pow(1 - cycle, 5.0);
    let t = (pow(1 - abs(0.2 - cycle), 4.0)) * main_thickness;
    let main = diamond(vec2(a), vec2(b), t, mesh.uv);
    let color_main = diamond(vec2(a), vec2(b), t - edge, mesh.uv);

    var secondary = 0.0;
    var color_secondary = 0.0;
    for (var i = 1; i < 3; i++) {
        // Secondaries start at max and fade out
        let fi = f32(i) * 2.0;
        let fiv = vec2(fi);
        let thickness = max(0.0, (1 - 8 * pow(cycle, 2.0))) * secondary_thickness;

        let movement = cycle * secondary_movement;

        let start_top = movement + secondary_start * fi;
        let start_bot = movement.yx + secondary_start.yx * fi;

        let end_top = cycle * movement + pow(secondary_end, fiv);
        let end_bot = cycle * movement.yx + pow(secondary_end.yx, fiv);

        secondary += diamond(start_top, end_top, thickness, mesh.uv);
        secondary += diamond(start_bot, end_bot, thickness, mesh.uv);
        color_secondary += diamond(start_top, end_top, thickness - edge, mesh.uv);
        color_secondary += diamond(start_bot, end_bot, thickness - edge, mesh.uv);
    }

    let main_field = max(main, secondary);
    let color_field = max(color_main, color_secondary);

    let color = inner_color * color_field + outer_color * (1 - color_field);
    return vec4(color.xyz, main_field);
}


fn diamond(cp1: vec2<f32>, cp2: vec2<f32>, max_width: f32, point: vec2<f32>) -> f32 {
    if length(cp1 - cp2) < max_width {
        // This prevents the whole thing flashing white
        return 0.0;
    }

    // cp stands for control point
    // https://stackoverflow.com/questions/64330618/finding-the-projection-of-a-point-onto-a-line
    let ab = cp2 - cp1;
    let ac = point - cp1;
    let dab = dot(ab, ab);
    let ad = ab * dot(ab, ac) / dab;

    let proj = cp1 + ad;
    let norm = dot(ad, ab) / dab;
    let threshold = 2 * (0.5 - abs(0.5 - norm)) * max_width;
    let dist = length(proj - point);
    return step(dist, threshold);
}

