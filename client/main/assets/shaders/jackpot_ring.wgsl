#import bevy_pbr::forward_io::{Vertex};
#import bevy_pbr::mesh_view_bindings::{globals, view};
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

#import "shaders/helpers.wgsl"::{easeInQuint, easeOutElastic, easeOutQuint, easeInCirc, PI, TAU, remap}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) uv_b: vec2<f32>,
    @location(4) world_tangent: vec4<f32>,
    @location(5) color: vec4<f32>,
    @location(6) @interpolate(flat) instance_index: u32,
    @location(7) height: f32,
}

const color_border = 0.1;   // Percentage, how close to peak change to mid-high colors

const width_at_peak = 0.1;  // Relative
const steepness = 6.0;

const low_color = vec3(0.2, 0.29, 0.17);        // Dull green
const mid_color= vec3(0.2235, 1.0, 0.0784);     // Neon green
const high_color = vec3(1.0);
const line_color = vec3(1.0);
const line_outline_color = vec3(0.0);
const color_fade = 2.0;
const line_inner_thickness = 0.002;              // Without outline
const line_outer_thickness = 0.01;              // Including outline

@group(2) @binding(0) var<uniform> start_time: f32;
@group(2) @binding(1) var<uniform> duration: f32;
@group(2) @binding(2) var<uniform> peak: f32;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let time = (globals.time - start_time) / duration;

    // Goes up and down, always in range [0, 1], 1 at peak
    var t = 1.0;
    var width = 1.0;
    if time < peak {
        t = pow(time / peak, steepness);
        width = remap(1-t, 0.0, 1.0, width_at_peak, 1.0);
    } else {
        t = pow((peak+1.0-time), steepness);
        width = easeInQuint(remap(time, peak, 1.0, 1.0, 0.0))* width_at_peak;
    }

    var height = vec3(0.0);
    if vertex.position.y > 0.0 {
        // The cylinder is drawn so that the bottom is -1 and top is 1
        height.y = abs(vertex.position.y) * (2.0*t - 1.0);
    }

    let radius = length(vertex.position.xz);
    let offset = vertex.normal * remap(width, 0.0, 1.0, -radius, 0.0) + height;

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position + offset, 1.0),
    );

    out.world_normal = vertex.normal;
    out.instance_index = vertex.instance_index;
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    if (abs(mesh.world_normal.y) == 1.0) {
        return vec4(0.0);
    }

    let time = (globals.time - start_time) / duration;

    if (time >= 1.0) {
        return vec4(0.0);
    }

    let accel = 10.0 * (pow(time+0.5, 4.0));
    let rev_accel = -40.0 * time;

    let main_wave = 0.1*sin(mesh.uv.x*2.0*TAU + accel);
    let secondary_wave = 0.05*sin(mesh.uv.x*8*TAU + rev_accel);
    let time_wave = 0.2*sin(8.0*TAU*time);
    let baseline = 0.5 * time;

    let height = main_wave + secondary_wave + time_wave + baseline;
    let on_line = abs(mesh.uv.y - height) < line_inner_thickness;
    let on_outline = abs(mesh.uv.y - height) < line_outer_thickness;
    if (on_line) {
        return vec4(line_color, 1.0);
    } if (on_outline) {
        return vec4(line_outline_color, 1.0);
    } else if (mesh.uv.y < height) {
        let alpha = color_fade*(height - mesh.uv.y);

        let dist = abs(peak - time);

        if dist < color_border {
            let normdist = (1.0 - dist / color_border);
            let color = mix(mid_color, high_color, normdist);
            return vec4(color, alpha);
        } else {
            let max_dist = max(peak, 1.0-peak);
            let normdist = easeInQuint(1.0 - remap(dist, color_border, max_dist, 0.0, 1.0));
            let color = mix(low_color, mid_color, normdist);
            return vec4(color, alpha);
        }
    } else {
        return vec4(0.0);
    }
}
