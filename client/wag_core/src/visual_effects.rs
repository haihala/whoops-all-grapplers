use bevy::prelude::*;

use crate::Icon;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum VisualEffect {
    #[default]
    Blank,
    Clash,
    Block,
    Hit,
    ThrowTech,
    SpeedLines,
    ThrowTarget,
    Lightning,
    Pebbles,
    Sparks,
    OpenerSpark(Color),
    WaveDiagonal(Color),
    WaveFlat(Color),
    SmokeBomb,
    Icon(Icon),
}
impl VisualEffect {
    pub fn mesh_size(&self) -> Rectangle {
        match self {
            VisualEffect::Clash => Rectangle::new(1.5, 1.5),
            VisualEffect::Block => Rectangle::new(1.1, 2.0),
            VisualEffect::Hit => Rectangle::new(1.1, 1.1),
            VisualEffect::ThrowTech | VisualEffect::ThrowTarget => Rectangle::new(2.0, 2.0),
            VisualEffect::Pebbles | VisualEffect::Sparks => Rectangle::new(1.8, 1.8),
            VisualEffect::SmokeBomb => Rectangle::new(3.0, 3.0),
            _ => Rectangle::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
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
