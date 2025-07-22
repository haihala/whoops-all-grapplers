use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use foundation::{
    BLOCK_EFFECT_BASE_COLOR, BLOCK_EFFECT_EDGE_COLOR, CLASH_SPARK_BASE_COLOR,
    CLASH_SPARK_EDGE_COLOR, HIT_SPARK_BASE_COLOR, HIT_SPARK_EDGE_COLOR, HIT_SPARK_MID_COLOR,
    JACKPOT_HIGH_POINT_PERCENTAGE, JACKPOT_TOTAL_DURATION, LIGHTNING_BOLT_INNER_COLOR,
    LIGHTNING_BOLT_OUTER_COLOR, OPENER_INNER_COLOR, PEBBLE_BORDER_COLOR, PEBBLE_INNER_COLOR,
    SPARK_BURST_BORDER_COLOR, SPARK_BURST_INNER_COLOR, SPEED_LINES_BASE_COLOR,
    SPEED_LINES_EDGE_COLOR,
};

pub trait AnimatedMaterial {
    fn bump_frame(&mut self);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LightningBoltMaterial {
    #[uniform(0)]
    inner_color: LinearRgba,
    #[uniform(1)]
    outer_color: LinearRgba,
    #[uniform(2)]
    pub frame: u32,
    #[uniform(3)]
    mirror: i32, // Bools not supported
}

impl LightningBoltMaterial {
    pub fn new(mirror: bool) -> Self {
        Self {
            inner_color: LIGHTNING_BOLT_INNER_COLOR.into(),
            outer_color: LIGHTNING_BOLT_OUTER_COLOR.into(),
            frame: 0,
            mirror: mirror as i32,
        }
    }
}

impl AnimatedMaterial for LightningBoltMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}

impl Material for LightningBoltMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/lightning_bolt.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct FocalPointLinesMaterial {
    #[uniform(0)]
    pub frame: u32,
}

impl AnimatedMaterial for FocalPointLinesMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Material for FocalPointLinesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/focal_point_lines.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LineFieldMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    edge_color: LinearRgba,
    #[uniform(2)]
    speed: f32,
    #[uniform(3)]
    line_thickness: f32,
    #[uniform(4)]
    layer_count: i32,
    #[uniform(5)]
    frame: u32,
    #[uniform(6)]
    duration: f32,
    #[uniform(7)]
    mirror: f32,
}
impl AnimatedMaterial for LineFieldMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl LineFieldMaterial {
    pub fn new(mirror: bool) -> Self {
        Self {
            mirror: mirror as i32 as f32 * -2.0 + 1.0,
            base_color: SPEED_LINES_BASE_COLOR.into(),
            edge_color: SPEED_LINES_EDGE_COLOR.into(),
            speed: 1.0,
            line_thickness: 0.2,
            layer_count: 5,
            duration: 0.2,
            frame: 0,
        }
    }
}

impl Material for LineFieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/lines.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HitSparkMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    mid_color: LinearRgba,
    #[uniform(2)]
    edge_color: LinearRgba,
    #[uniform(3)]
    pub frame: u32,
}
impl AnimatedMaterial for HitSparkMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Default for HitSparkMaterial {
    fn default() -> Self {
        Self {
            edge_color: HIT_SPARK_EDGE_COLOR.into(),
            mid_color: HIT_SPARK_MID_COLOR.into(),
            base_color: HIT_SPARK_BASE_COLOR.into(),
            frame: 0,
        }
    }
}

impl Material for HitSparkMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hit_spark.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlockEffectMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    edge_color: LinearRgba,
    #[uniform(2)]
    speed: f32,
    #[uniform(3)]
    pub frame: u32,
}
impl AnimatedMaterial for BlockEffectMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Default for BlockEffectMaterial {
    fn default() -> Self {
        Self {
            edge_color: BLOCK_EFFECT_EDGE_COLOR.into(),
            base_color: BLOCK_EFFECT_BASE_COLOR.into(),
            speed: 1.5,
            frame: 0,
        }
    }
}

impl Material for BlockEffectMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/block_effect.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ClashSparkMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    edge_color: LinearRgba,
    #[uniform(2)]
    speed: f32,
    #[uniform(3)]
    pub frame: u32,
}
impl AnimatedMaterial for ClashSparkMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Default for ClashSparkMaterial {
    fn default() -> Self {
        Self {
            frame: 0,
            edge_color: CLASH_SPARK_EDGE_COLOR.into(),
            base_color: CLASH_SPARK_BASE_COLOR.into(),
            speed: 1.2,
        }
    }
}

impl Material for ClashSparkMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/clash_spark.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
pub struct BlankMaterial {} // needs to be this type of struct for material
impl AnimatedMaterial for BlankMaterial {
    fn bump_frame(&mut self) {}
}
impl Material for BlankMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/blank.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RingRippleMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub edge_color: LinearRgba,
    #[uniform(2)]
    pub duration: f32,
    #[uniform(3)]
    pub ring_thickness: f32,
    #[uniform(4)]
    pub frame: u32,
    #[uniform(5)]
    pub rings: i32,
    #[uniform(6)]
    pub offset: f32,
}
impl Default for RingRippleMaterial {
    fn default() -> Self {
        Self {
            frame: 0,
            base_color: LinearRgba::default(),
            edge_color: LinearRgba::default(),
            duration: 0.7,
            ring_thickness: 0.05,
            rings: 1,
            offset: 0.08,
        }
    }
}
impl AnimatedMaterial for RingRippleMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Material for RingRippleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ring_ripple.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct IconFlashMaterial {
    #[uniform(0)]
    pub frame: u32,
    #[texture(1)]
    #[sampler(2)]
    array_texture: Handle<Image>,
}
impl IconFlashMaterial {
    pub fn new(icon: Handle<Image>) -> Self {
        Self {
            frame: 0,
            array_texture: icon,
        }
    }
}
impl AnimatedMaterial for IconFlashMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Material for IconFlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/icon_flash.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct SmokeBombMaterial {
    #[uniform(0)]
    pub frame: u32,
}
impl AnimatedMaterial for SmokeBombMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Material for SmokeBombMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/smoke_bomb.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SparkBurstMaterial {
    #[uniform(0)]
    pub frame: u32,
    #[uniform(1)]
    inner_color: LinearRgba,
    #[uniform(2)]
    border_color: LinearRgba,
    #[uniform(3)]
    mirror: f32,
}
impl AnimatedMaterial for SparkBurstMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl SparkBurstMaterial {
    pub fn new(mirror: bool) -> Self {
        Self {
            frame: 0,
            inner_color: SPARK_BURST_INNER_COLOR.into(),
            border_color: SPARK_BURST_BORDER_COLOR.into(),
            mirror: mirror as i32 as f32,
        }
    }
}

impl Material for SparkBurstMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/spark_burst.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PebbleMaterial {
    #[uniform(0)]
    pub frame: u32,
    #[uniform(1)]
    inner_color: LinearRgba,
    #[uniform(2)]
    border_color: LinearRgba,
    #[uniform(3)]
    mirror: f32,
}
impl AnimatedMaterial for PebbleMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl PebbleMaterial {
    pub fn new(mirror: bool) -> Self {
        Self {
            frame: 0,
            inner_color: PEBBLE_INNER_COLOR.into(),
            border_color: PEBBLE_BORDER_COLOR.into(),
            mirror: mirror as i32 as f32,
        }
    }
}

impl Material for PebbleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/pebbles.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MidFlashMaterial {
    #[uniform(0)]
    pub frame: u32,
    #[uniform(1)]
    inner_color: LinearRgba,
    #[uniform(2)]
    outer_color: LinearRgba,
}
impl MidFlashMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            frame: 0,
            inner_color: OPENER_INNER_COLOR.into(),
            outer_color: color.into(),
        }
    }
}
impl AnimatedMaterial for MidFlashMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Material for MidFlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mid_flash.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DiagonalWaveMaterial {
    #[uniform(0)]
    pub frame: u32,
    #[uniform(1)]
    color: LinearRgba,
    #[uniform(2)]
    mirror: i32, // Bools not supported
}
impl DiagonalWaveMaterial {
    pub fn new(color: Color, mirror: bool) -> Self {
        Self {
            frame: 0,
            color: color.into(),
            mirror: mirror as i32,
        }
    }
}

impl AnimatedMaterial for DiagonalWaveMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Material for DiagonalWaveMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/diagonal_wave.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FlatWaveMaterial {
    #[uniform(0)]
    pub frame: u32,
    #[uniform(1)]
    color: LinearRgba,
    #[uniform(2)]
    mirror: i32, // Bools not supported
}
impl FlatWaveMaterial {
    pub fn new(color: Color, mirror: bool) -> Self {
        Self {
            frame: 0,
            color: color.into(),
            mirror: mirror as i32,
        }
    }
}
impl AnimatedMaterial for FlatWaveMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Material for FlatWaveMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/flat_wave.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct JackpotRingMaterial {
    #[uniform(0)]
    pub frame: u32,
    #[uniform(1)]
    pub duration: f32,
    #[uniform(2)]
    pub peak: f32,
}
impl AnimatedMaterial for JackpotRingMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Default for JackpotRingMaterial {
    fn default() -> Self {
        Self {
            frame: 0,
            duration: JACKPOT_TOTAL_DURATION,
            peak: JACKPOT_HIGH_POINT_PERCENTAGE,
        }
    }
}

impl Material for JackpotRingMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/jackpot_ring.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/jackpot_ring.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct BezierSwooshMaterial {
    #[uniform(0)]
    pub control_points: [Vec4; 16],
    #[uniform(1)]
    pub curves: u32,
    #[uniform(2)]
    pub duration: u32,
    #[uniform(3)]
    pub overlap: f32,
    #[uniform(4)]
    pub frame: u32,
    #[uniform(5)]
    pub primary_color: LinearRgba,
    #[uniform(6)]
    pub secondary_color: LinearRgba,
}
impl AnimatedMaterial for BezierSwooshMaterial {
    fn bump_frame(&mut self) {
        self.frame += 1;
    }
}
impl Material for BezierSwooshMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/bezier_swoosh.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

// Extended Flash Material
pub type ExtendedFlashMaterial = ExtendedMaterial<StandardMaterial, FlashMaterial>;

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
pub struct FlashMaterial {
    // Start at a high binding number to ensure bindings don't conflict
    // with the base material
    #[uniform(100)]
    pub color: LinearRgba,
    #[uniform(101)]
    pub speed: f32,
    #[uniform(102)]
    pub depth: f32, // How far into the flash to go? 1 means go full monochrome color, 0 means no change
    #[uniform(103)]
    pub duration: f32,
    #[uniform(104)]
    pub frame: u32,
    #[uniform(105)]
    pub color_shift_end: f32,
    #[uniform(106)]
    pub color_shift: LinearRgba,
    #[uniform(107)]
    pub angle_mult: f32,
}
impl MaterialExtension for FlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/flash_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/flash_material.wgsl".into()
    }
}
impl AnimatedMaterial for ExtendedFlashMaterial {
    fn bump_frame(&mut self) {
        self.extension.frame += 1;
    }
}
