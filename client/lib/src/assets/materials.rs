use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use characters::FlashRequest;
use wag_core::{
    BLOCK_EFFECT_BASE_COLOR, BLOCK_EFFECT_EDGE_COLOR, CLASH_SPARK_BASE_COLOR,
    CLASH_SPARK_EDGE_COLOR, HIT_SPARK_BASE_COLOR, HIT_SPARK_EDGE_COLOR, HIT_SPARK_MID_COLOR,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HitSparkMaterial {
    #[uniform(0)]
    base_color: LinearRgba,
    #[uniform(1)]
    mid_color: LinearRgba,
    #[uniform(2)]
    edge_color: LinearRgba,
    #[uniform(3)]
    start_time: f32,
}
impl HitSparkMaterial {
    pub(crate) fn reset(&mut self, time: f32) {
        self.start_time = time;
    }
}

impl Default for HitSparkMaterial {
    fn default() -> Self {
        Self {
            edge_color: HIT_SPARK_EDGE_COLOR.into(),
            mid_color: HIT_SPARK_MID_COLOR.into(),
            base_color: HIT_SPARK_BASE_COLOR.into(),
            start_time: 0.0,
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
    start_time: f32,
}
impl BlockEffectMaterial {
    pub(crate) fn reset(&mut self, time: f32) {
        self.start_time = time;
    }
}
impl Default for BlockEffectMaterial {
    fn default() -> Self {
        Self {
            edge_color: BLOCK_EFFECT_EDGE_COLOR.into(),
            base_color: BLOCK_EFFECT_BASE_COLOR.into(),
            speed: 1.5,
            start_time: 0.0,
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
    start_time: f32,
}
impl ClashSparkMaterial {
    pub(crate) fn reset(&mut self, time: f32) {
        self.start_time = time;
    }
}

impl Default for ClashSparkMaterial {
    fn default() -> Self {
        Self {
            edge_color: CLASH_SPARK_EDGE_COLOR.into(),
            base_color: CLASH_SPARK_BASE_COLOR.into(),
            speed: 1.2,
            start_time: 0.0,
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

// Extended Flash Material
pub type ExtendedFlashMaterial = ExtendedMaterial<StandardMaterial, FlashMaterial>;

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
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
    pub start_time: f32,
}
impl MaterialExtension for FlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/flash_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/flash_material.wgsl".into()
    }
}
impl FlashMaterial {
    pub fn from_request(request: FlashRequest, time: f32) -> Self {
        Self {
            color: request.color.into(),
            speed: request.speed,
            depth: request.depth,
            duration: request.duration,
            start_time: time,
        }
    }
}
