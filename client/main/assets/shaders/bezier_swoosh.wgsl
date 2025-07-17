#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI, easeInQuint, remap};

// Z controls relative thickness
@group(2) @binding(0) var<uniform> control_points: array<vec3f, 16>;
@group(2) @binding(1) var<uniform> curve_count: u32;
@group(2) @binding(2) var<uniform> duration: f32;
@group(2) @binding(3) var<uniform> start_time: f32;
@group(2) @binding(4) var<uniform> primary_color: vec4f;
@group(2) @binding(5) var<uniform> secondary_color: vec4f;

const sections_per_curve_per_unit: u32 = 40;
const t_length = 0.3;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time - start_time;
    let cycle = remap(
        clamp(time / duration, 0.0, 1.0),
        0.0,
        1.0,
        0.0,
        1.0-t_length,
    );

    let coord = (mesh.uv - 0.5) * vec2(2.0, -2.0);
    let curve = calc_curve(
        coord, 
        cycle,
        t_length,
    );

    if !curve.on_curve {
        return vec4(0.0);
    }

    return swoosh(curve.uv, cycle);
}

fn swoosh(uv: vec2f, t: f32) -> vec4f {
    let coord = (uv-0.5)*2;

    let edge_mask = 1-easeInQuint(abs(coord.x));

    let corner_fade_x = 1-cos(coord.x * PI / 2.0);
    let corner_fade_y = abs(coord.y);
    let corner_mask = 1-(corner_fade_x*corner_fade_y);

    var stripes = 0.0;
    let layers = 3;
    let base_frequency = 0.3;
    let frequency_power_base = 3.0;

    for (var i = 1; i <= layers; i++) {
        let l = f32(i);

        let main_freq = abs(coord.x) * base_frequency * pow(frequency_power_base, l);
        stripes += cos(main_freq);
    }

    // Some variation over time
    stripes += sin(coord.x * 3.0 + t * PI * 5.0);

    let wave_y = cos(abs(coord.y) * PI/2.0);
    let wave_mask = clamp(stripes, 0.0, 1.0) * wave_y;

    let mask = edge_mask * corner_mask * wave_mask;

    let throughline_weight = clamp(1.3-abs(coord.x), 0.0, 1.0);
    let color = mix(secondary_color, primary_color, throughline_weight);

    return vec4(mask * color);
}

struct CurveHit {
    uv: vec2f,
    dist: f32,
    on_curve: bool,
}

fn calc_curve(coord: vec2f, uv_map_start: f32, uv_map_length: f32) -> CurveHit {
    let curve_length = min(uv_map_length, 1.0 - uv_map_start);
    let section_count = u32(
        max(
            2.0,    // In order to not get rounded ends, we need at least two segments
            f32(sections_per_curve_per_unit) * curve_length * f32(curve_count)
        )
    );
    let sections = f32(section_count);
    let section_len = curve_length/sections;

    var output: CurveHit;

    for (var int_i: u32 = 0; int_i < section_count; int_i++) {
        let i = f32(int_i);
        let t_start = uv_map_start + i * section_len;
        let t_end = t_start + section_len;
        let in_first_half = i < (sections/2.0);

        let start_bez = bezier(t_start);
        let end_bez = bezier(t_end);
        let start = start_bez.xy;
        let end = end_bez.xy;

        let projection = project(coord, start, end);
        let thickness = mix(start_bez.z, end_bez.z, projection) * 0.01;    // Multiply for nicer numbers

        // Aight so story time
        // Trying to deduce if a point is on the curve or not was a pain
        //
        // Approach 1: Project to line, if projection is not between start and end, cull it
        // Problem: When curving, there are gaps on the outer edge
        //
        // Approach 2: Distance field
        // Problem: Rounded edges, uvs outside of the [0, 1] range
        //
        // Approach 3 (active): Project + distance to segment end
        // On the first half, allow going over
        // On the last half, allow undershooting
        // This has hard ends and leaves no gaps

        var off_segment_proj_plus = false;
        if in_first_half {
            // Allow going over, but not under
            off_segment_proj_plus = projection < 0.0 || (projection > 1.0 && length(coord-end) > end_bez.z*0.01);
        } else {
            // Allow falling short, but not going over
            off_segment_proj_plus = projection > 1.0 || (projection < 0.0 && length(coord-start) > start_bez.z*0.01);
        }

        if off_segment_proj_plus {
            continue;
        }

        let clamp_proj = clamp(projection, 0.0, 1.0);
        let meeting_point = mix(start, end, clamp_proj);
        let dist = length(meeting_point - coord);

        if dist < thickness {
            let norm_dist = dist/thickness;
            let uvs = vec2(
                (calc_side(coord, start, end) / thickness + 1) / 2,
                (i + clamp_proj) / sections
            );

            if !output.on_curve || output.dist > norm_dist {
                output.uv = uvs;
                output.dist = norm_dist;
            }

            output.on_curve = true;
        }
    }

    return output;
}

fn bezier(full_t: f32) -> vec3f {
    let t_per_set = 1.0 / f32(curve_count);
    let set_index = floor(full_t / t_per_set);
    let offset = 3 * i32(set_index);
    let t = (full_t - (t_per_set * set_index)) / t_per_set;

    let a = control_points[offset];
    let b = control_points[offset+1];
    let c = control_points[offset+2];
    let d = control_points[offset+3];

    let a_part = a * (-pow(t, 3.0) + 3.0*pow(t, 2.0) - 3.0*t+1.0);
    let b_part = b * (3.0 * pow(t,3.0) - 6*pow(t, 2.0) + 3*t);
    let c_part = c * (-3.0*pow(t,3.0) + 3*pow(t,2.0));
    let d_part = d * pow(t, 3.0);
    return a_part + b_part + c_part + d_part;
}

// From Sebastian
fn calc_side(point: vec2f, start: vec2f, end: vec2f) -> f32 {
    let line = end - start;
    let offset = point - start;
    return (line.x * offset.y - line.y * offset.x) / length(line);
}

fn project(point: vec2f, start: vec2f, end: vec2f) -> f32 {
    let rel_point = point-start;
    let line = end-start;   // Line we are projecting on
    return dot(rel_point, line) / dot(line, line); // How far along the line we are
}

