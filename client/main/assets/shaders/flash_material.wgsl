#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, Vertex, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

@group(2) @binding(100)
var<uniform> flash_color: vec4<f32>;
@group(2) @binding(101)
var<uniform> flash_speed: f32;
@group(2) @binding(102)
var<uniform> flash_depth: f32;
@group(2) @binding(103)
var<uniform> flash_duration: f32;
@group(2) @binding(104)
var<uniform> flash_start: f32;
@group(2) @binding(105)
var<uniform> weaken_end: f32;
@group(2) @binding(106)
var<uniform> weaken_color: vec4f;

#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_view_bindings::{globals, view},
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    let flash_age = (globals.time - flash_start);
    let age_damp = step(flash_age, flash_duration);
    let norm = dot(in.world_normal, normalize(view.world_position.xyz - in.world_position.xyz));

    let depth = step(norm, abs(cos(flash_age * flash_speed)));
#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // we can optionally modify the final result here
#endif

    if globals.time < weaken_end {
        out.color = mix(out.color, weaken_color, 0.98);
    }

    out.color = mix(
        out.color,
        flash_color,
        depth * age_damp,
    );

    return out;
}
