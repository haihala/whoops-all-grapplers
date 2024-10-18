use bevy::{prelude::*, utils::HashMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect, EnumIter)]
pub enum SoundEffect {
    Whoosh,
    Clash,
    Block, // Unused
    Hit,
    #[default]
    Silence,
    GlassClink,
    PotLidGong,
    PlasticCupFlick,
    PlasticCupTap,
    CheekSlap,
    FemaleExhale,
    BottleBonk,
    PastaPat,
}

impl SoundEffect {
    pub fn paths() -> HashMap<SoundEffect, Vec<String>> {
        Self::iter().map(|sfx| (sfx, sfx.asset_paths())).collect()
    }

    fn asset_paths(&self) -> Vec<String> {
        match self {
            SoundEffect::Whoosh => vec!["sound_effects/whoosh.ogg".to_string()],
            SoundEffect::Clash => Self::clips("clink", 2),
            SoundEffect::Block => vec!["sound_effects/block.ogg".to_string()],
            SoundEffect::Hit => Self::clips("hit", 3),
            SoundEffect::Silence => vec![],
            SoundEffect::GlassClink => Self::clips("glass", 8),
            SoundEffect::PotLidGong => Self::clips("pot-lid", 4),
            SoundEffect::PlasticCupFlick => Self::clips("plastic-cup-flick", 23),
            SoundEffect::PlasticCupTap => Self::clips("plastic-cup-tap", 20),
            SoundEffect::CheekSlap => Self::clips("cheek-slap", 20),
            SoundEffect::FemaleExhale => Self::clips("female-exhale", 16),
            SoundEffect::BottleBonk => Self::clips("bottle-bonk", 12),
            SoundEffect::PastaPat => Self::clips("pasta-pat", 11),
        }
    }

    fn clips(base_file_name: &'static str, amount: usize) -> Vec<String> {
        (1..=amount)
            .map(|int| format!("sound_effects/{}-{:0>2}.ogg", base_file_name, int))
            .collect()
    }

    pub fn volume(self) -> f32 {
        match self {
            SoundEffect::FemaleExhale => 0.1,
            SoundEffect::PlasticCupFlick => 0.1,
            SoundEffect::PotLidGong => 0.8,
            _ => 1.0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
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
    MidFlash,
    WaveDiagonal,
    WaveFlat,
}
impl VisualEffect {
    pub fn mesh_size(&self) -> Rectangle {
        match self {
            VisualEffect::Clash => Rectangle::new(1.5, 1.5),
            VisualEffect::Block => Rectangle::new(1.1, 2.0),
            VisualEffect::Hit => Rectangle::new(1.1, 1.1),
            VisualEffect::ThrowTech | VisualEffect::ThrowTarget => Rectangle::new(2.0, 2.0),
            VisualEffect::Pebbles | VisualEffect::Sparks => Rectangle::new(1.8, 1.8),
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
