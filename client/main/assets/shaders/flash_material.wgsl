#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_view_bindings::{globals, view},
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
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


@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var flash_age = (globals.time - flash_start);
    var age_damp = step(flash_age, flash_duration);
    var norm = dot(in.world_normal, normalize(view.world_position.xyz - in.world_position.xyz));
    var norm_damp = pow(1-norm, 4.0);

    var ratio = flash_depth * pow(cos(globals.time * flash_speed), 2.0);
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

    out.color = mix(
        out.color,
        flash_color,
        norm_damp * age_damp * ratio
    );


    return out;
}
