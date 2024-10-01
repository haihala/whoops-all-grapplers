#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

const PI = 3.14159265359;

// Controls
const zigzags = 3.0;
const sharpness = 0.8;
const max_dip = 0.1;
const max_bump = 0.075;
const thickness = 0.1;
const smoothing = 0.15;
const color_ratio = 0.8;
const wave_thickness = 3.0;
const wave_speed = 18.0;

@group(2) @binding(0) var<uniform> inner_color: vec4<f32>;
@group(2) @binding(1) var<uniform> outer_color: vec4<f32>;
@group(2) @binding(2) var<uniform> start_time: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time + 10000;
    let cycle = (time - start_time);

    let shake = pow((1 - cycle), 5.0) * 0.1;

    let scale = (zigzags - (1 - sharpness));
    let x = mesh.uv.x * scale + shake * sin(250 * time);
    let y = mesh.uv.y * scale + shake * cos(170 * time);
    let segment = i32(x);

    let dip = max_dip * zigzags;
    let bump = max_bump * zigzags;

    var shape = 1.0;
    if (x - f32(segment)) < sharpness { // Downwards part of the segment
        // Segment start (match previous segment end)
        //y = x-bump
        let bump_term = -bump * (1.0 - (x - f32(segment)) / sharpness);

        // Segment discontinuity
        // y = x+dip;
        let dip_term = dip * (x - f32(segment)) / sharpness;
        shape = dip_term + bump_term;
    } else {    // Upwards part of the segment
        // Segment discontinuity start
        // y = x+dip;
        let dip_term = dip * (1.0 - (x - f32(segment) - sharpness) / (1 - sharpness));

        // Segment end (match next segment start)
        //y = x-bump
        let bump_term = -bump * (x - f32(segment) - sharpness) / (1 - sharpness);

        shape = dip_term + bump_term;
    }

    let norm_term = ilerp(0.0, smoothing, mesh.uv.y * (1 - mesh.uv.y));
    let flash = max(0.0, 1 - cycle * 2.0);
    let wave_target = sqrt(2.0) * cycle * wave_speed;
    let diag_pos = mesh.uv.x + mesh.uv.y;
    let wave = wave_thickness * clamp((1 - pow(wave_target - diag_pos, 2.0)), 1 / wave_thickness, 1.0);
    let threshold = norm_term * thickness * flash * wave;

    let field = length(x - y + shape);

    var color = vec4(0.0);

    if field < color_ratio * threshold {
        color = inner_color;
    } else if field < threshold {
        color = outer_color;
    }

    return color;
}

fn ilerp(floor: f32, ceil: f32, val: f32) -> f32 {
    return (val - floor) / (ceil - floor);
}

