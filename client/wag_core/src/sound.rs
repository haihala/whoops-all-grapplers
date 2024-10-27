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
    FemaleHiYah,
    FemaleLoYah,
    FemaleKyatchi,
    FemaleNoooo,
    FemaleOw,
    FemaleGutPunch,
    BottleBonk,
    PastaPat,
    Number(usize),
    AnnouncerFight,
    AnnouncerRound,
    AnnouncerWins,
    AnnouncerDraw,
    AnnouncerPlayer,
    KnifeChopstickDrag,
    HangingKnifeFlick,
}

impl SoundEffect {
    pub fn paths() -> HashMap<SoundEffect, Vec<String>> {
        Self::iter()
            .flat_map(|sfx| {
                if matches!(sfx, SoundEffect::Number(_)) {
                    (1..=20).map(SoundEffect::Number).collect()
                } else {
                    vec![sfx]
                }
            })
            .map(|sfx| (sfx, sfx.asset_path()))
            .collect()
    }

    fn asset_path(&self) -> Vec<String> {
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
            SoundEffect::FemaleExhale => Self::clips("female-exhale", 9),
            SoundEffect::FemaleHiYah => Self::clips("female-hi-yah", 10),
            SoundEffect::FemaleLoYah => Self::clips("female-lo-yah", 5),
            SoundEffect::FemaleKyatchi => Self::clips("female-kyatchi", 6),
            SoundEffect::FemaleNoooo => Self::clips("female-noooo", 3),
            SoundEffect::FemaleOw => Self::clips("female-ow", 8),
            SoundEffect::FemaleGutPunch => Self::clips("female-gut-punch", 8),
            SoundEffect::BottleBonk => Self::clips("bottle-bonk", 12),
            SoundEffect::PastaPat => Self::clips("pasta-pat", 11),
            SoundEffect::Number(n) => vec![format!("sound_effects/number-{:0>2}.ogg", n)],
            SoundEffect::AnnouncerRound => Self::clips("announcer-round", 3),
            SoundEffect::AnnouncerFight => Self::clips("announcer-fight", 3),
            SoundEffect::AnnouncerWins => Self::clips("announcer-wins", 5),
            SoundEffect::AnnouncerPlayer => Self::clips("announcer-player", 5),
            SoundEffect::AnnouncerDraw => Self::clips("announcer-draw", 2),
            SoundEffect::KnifeChopstickDrag => Self::clips("knife-dragging-on-chopstick", 7),
            SoundEffect::HangingKnifeFlick => Self::clips("hanging-knife-flick", 4),
        }
    }

    fn clips(base_file_name: &'static str, amount: usize) -> Vec<String> {
        (1..=amount)
            .map(|int| format!("sound_effects/{}-{:0>2}.ogg", base_file_name, int))
            .collect()
    }

    pub fn volume(self) -> f32 {
        match self {
            SoundEffect::FemaleExhale => 0.4,
            SoundEffect::PlasticCupFlick => 0.1,
            SoundEffect::PotLidGong => 0.6,
            SoundEffect::Number(_) => 0.7,
            SoundEffect::FemaleOw => 2.0,
            _ => 1.0,
        }
    }

    pub fn is_announcer(&self) -> bool {
        matches!(
            self,
            SoundEffect::AnnouncerDraw
                | SoundEffect::AnnouncerWins
                | SoundEffect::AnnouncerPlayer
                | SoundEffect::AnnouncerRound
                | SoundEffect::AnnouncerFight
        )
    }
}

pub const BIG_HIT_THRESHOLD: i32 = 30;
pub const SMALL_HIT_THRESHOLD: i32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoiceLine {
    Defeat,
    SmallHit,
    BigHit,
}
