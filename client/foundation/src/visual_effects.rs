use bevy::prelude::*;

use crate::Icon;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum VisualEffect {
    #[default]
    Blank,
    Clash,
    Block,
    Hit,
    SpeedLines,
    ThrowTarget,
    Lightning,
    Pebbles,
    Sparks,
    OpenerSpark(Color),
    WaveDiagonal(Color),
    WaveFlat(Color),
    RingPulse(RingPulse),
    SmokeBomb,
    Icon(Icon),
    JackpotRing,
    Smear(Smear),
}
impl VisualEffect {
    pub fn mesh_size(&self) -> Rectangle {
        match self {
            VisualEffect::Clash => Rectangle::new(1.5, 1.5),
            VisualEffect::Block => Rectangle::new(1.1, 2.0),
            VisualEffect::Hit => Rectangle::new(1.1, 1.1),
            VisualEffect::RingPulse(_) | VisualEffect::ThrowTarget => Rectangle::new(2.0, 2.0),
            VisualEffect::Pebbles | VisualEffect::Sparks => Rectangle::new(1.8, 1.8),
            VisualEffect::SmokeBomb => Rectangle::new(3.0, 3.0),
            VisualEffect::JackpotRing => Rectangle::new(2.0, 3.0),
            _ => Rectangle::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RingPulse {
    pub base_color: Color,
    pub edge_color: Color,
    pub rings: i32,
    pub duration: f32,
    pub thickness: f32,
    pub offset: f32,
}

impl Default for RingPulse {
    fn default() -> Self {
        Self {
            base_color: Default::default(),
            edge_color: Default::default(),
            rings: 1,
            duration: 1.0,
            thickness: 0.05,
            offset: 0.08,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Smear {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub control_points: Vec<Vec3>,
    pub duration: usize,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct VfxRequest {
    pub effect: VisualEffect,
    pub tf: Transform,
    pub mirror: bool,
}

impl From<VisualEffect> for VfxRequest {
    fn from(effect: VisualEffect) -> Self {
        Self {
            effect,
            ..default()
        }
    }
}
