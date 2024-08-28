#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> edge_color: vec4<f32>;
@group(2) @binding(2) var<uniform> speed: f32;
@group(2) @binding(3) var<uniform> angle: f32;
@group(2) @binding(4) var<uniform> line_thickness: f32;
@group(2) @binding(5) var<uniform> layer_count: i32;
@group(2) @binding(6) var<uniform> start_time: f32;
@group(2) @binding(7) var<uniform> duration: f32;

const PI = 3.14159265359;
const offset = PI * 2 / 3;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = (globals.time - start_time);

    let wt = 1 - (time / duration);
    // Ease-in-quint
    let wave = pow(wt, 5.0);
    let centered = 2 * (mesh.uv - 0.5);

    var layers = 0.0;
    for (var i = 0; i < layer_count; i++) {
        let t = speed * time * f32(i) + 1.0 / f32(i);
        layers += clamp(layer(t, angle, centered), 0.0, 1.0);
    }
    layers *= wave;

    let falloff = clamp(1.0 - length(centered), 0.0, 1.0);
    let color = lerp(clamp(layers, 0.0, 1.0), base_color, edge_color);
    return vec4(color.xyz, layers * falloff);
}

fn layer(time: f32, ang: f32, coords: vec2<f32>) -> f32 {
    // Creates the lines that go along the wanted direction
    let angled = (coords.x * cos(ang) + coords.y * sin(ang));
    let stripes = step(abs(sin(angled / line_thickness)), 0.5);

    // Produces a rolling effect
    let coangled = coords.x * cos(ang - PI / 2) + coords.y * sin(ang - PI / 2);
    let roller = sin(coangled + 2 * PI * fract(time) - PI / 2);

    // Masks out some lines
    let lanes = step(abs(sin((5 + floor(time) % 3) * angled + 10 * floor(time))), line_thickness);
    return stripes * lanes * roller;
}

fn lerp(t: f32, c1: vec4<f32>, c2: vec4<f32>) -> vec4<f32> {
    return t * c1 + (1 - t) * c2;
}

