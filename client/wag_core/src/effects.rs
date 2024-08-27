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
            SoundEffect::GlassClink => Self::clips("glass", 10),
            SoundEffect::PotLidGong => Self::clips("pot-lid", 4),
            SoundEffect::PlasticCupFlick => Self::clips("plastic-cup-flick", 23),
            SoundEffect::PlasticCupTap => Self::clips("plastic-cup-tap", 20),
            SoundEffect::CheekSlap => Self::clips("cheek-slap", 20),
            SoundEffect::FemaleExhale => Self::clips("female-exhale", 16),
            SoundEffect::BottleBonk => Self::clips("bottle-bonk", 12),
        }
    }

    fn clips(base_file_name: &'static str, amount: usize) -> Vec<String> {
        (1..=amount)
            .map(|int| format!("sound_effects/{}-{:0>2}.ogg", base_file_name, int))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum VisualEffect {
    Clash,
    Block,
    Hit,
    ThrowTech,
}
