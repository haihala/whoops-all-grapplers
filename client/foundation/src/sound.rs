use bevy::{prelude::*, utils::HashMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Resource)]
pub struct Sounds {
    pub handles: HashMap<Sound, Vec<Handle<AudioSource>>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Reflect, EnumIter)]
pub enum Sound {
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
    FemaleShagamu,
    FemaleKiritsu,
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
    Matches,
    PaperCrumple,

    // Music
    Motivation,
    AnimeBeginnings,
    WaitingMusic,
}

impl Sound {
    pub fn paths() -> HashMap<Sound, Vec<String>> {
        Self::iter()
            .flat_map(|sfx| {
                if matches!(sfx, Sound::Number(_)) {
                    (1..=20).map(Sound::Number).collect()
                } else {
                    vec![sfx]
                }
            })
            .map(|sfx| (sfx, sfx.asset_path()))
            .collect()
    }

    fn asset_path(&self) -> Vec<String> {
        match self {
            Sound::Whoosh => vec!["sound_effects/whoosh.ogg".to_string()],
            Sound::Clash => Self::clips("clink", 2),
            Sound::Block => vec!["sound_effects/block.ogg".to_string()],
            Sound::Hit => Self::clips("hit", 3),
            Sound::Silence => vec![],
            Sound::GlassClink => Self::clips("glass", 8),
            Sound::PotLidGong => Self::clips("pot-lid", 4),
            Sound::PlasticCupFlick => Self::clips("plastic-cup-flick", 23),
            Sound::PlasticCupTap => Self::clips("plastic-cup-tap", 20),
            Sound::CheekSlap => Self::clips("cheek-slap", 20),
            Sound::FemaleExhale => Self::clips("female-exhale", 9),
            Sound::FemaleHiYah => Self::clips("female-hi-yah", 10),
            Sound::FemaleLoYah => Self::clips("female-lo-yah", 5),
            Sound::FemaleShagamu => Self::clips("female-shagamu", 4),
            Sound::FemaleKiritsu => Self::clips("female-kiritsu", 4),
            Sound::FemaleKyatchi => Self::clips("female-kyatchi", 6),
            Sound::FemaleNoooo => Self::clips("female-noooo", 3),
            Sound::FemaleOw => Self::clips("female-ow", 8),
            Sound::FemaleGutPunch => Self::clips("female-gut-punch", 8),
            Sound::BottleBonk => Self::clips("bottle-bonk", 12),
            Sound::PastaPat => Self::clips("pasta-pat", 11),
            Sound::Number(n) => vec![format!("sound_effects/number-{:0>2}.ogg", n)],
            Sound::AnnouncerRound => Self::clips("announcer-round", 3),
            Sound::AnnouncerFight => Self::clips("announcer-fight", 3),
            Sound::AnnouncerWins => Self::clips("announcer-wins", 5),
            Sound::AnnouncerPlayer => Self::clips("announcer-player", 5),
            Sound::AnnouncerDraw => Self::clips("announcer-draw", 2),
            Sound::KnifeChopstickDrag => Self::clips("knife-dragging-on-chopstick", 7),
            Sound::HangingKnifeFlick => Self::clips("hanging-knife-flick", 4),
            Sound::Matches => Self::clips("matches", 5),
            Sound::PaperCrumple => Self::clips("crumpled-paper", 3),
            Sound::Motivation => vec!["music/motivation-258263.mp3".to_string()],
            Sound::AnimeBeginnings => vec!["music/anime-beginings-139797.mp3".to_string()],
            Sound::WaitingMusic => {
                vec!["music/waiting-music-116216.mp3".to_string()]
            }
        }
    }

    fn clips(base_file_name: &'static str, amount: usize) -> Vec<String> {
        (1..=amount)
            .map(|int| format!("sound_effects/{}-{:0>2}.ogg", base_file_name, int))
            .collect()
    }

    pub fn volume(self) -> f32 {
        match self {
            Sound::FemaleExhale => 0.4,
            Sound::PlasticCupFlick => 0.1,
            Sound::PotLidGong => 0.6,
            Sound::Number(_) => 0.7,
            Sound::FemaleOw => 2.0,
            Sound::PaperCrumple => 0.5,
            Sound::Matches => 1.3,

            Sound::Motivation | Sound::AnimeBeginnings | Sound::WaitingMusic => 0.3,

            // Music
            _ => 1.0,
        }
    }

    pub fn is_announcer(&self) -> bool {
        matches!(
            self,
            Sound::AnnouncerDraw
                | Sound::AnnouncerWins
                | Sound::AnnouncerPlayer
                | Sound::AnnouncerRound
                | Sound::AnnouncerFight
        )
    }

    pub fn is_music(&self) -> bool {
        matches!(
            self,
            Sound::AnimeBeginnings | Sound::WaitingMusic | Sound::Motivation
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

#[derive(Debug, PartialEq, Clone, Copy, Default, Reflect, Event)]
pub struct SoundRequest {
    pub sound: Sound,
    // Bevy doesn't make seeking or getting the current seek position easy
    //pub seek: f32,
}

impl From<Sound> for SoundRequest {
    fn from(sound: Sound) -> Self {
        Self { sound }
    }
}
