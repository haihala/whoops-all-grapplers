use crate::ryan::*;
use crate::universal::*;
use bevy::utils::HashMap;
use types::StickPosition;
use types::{GameButton, MoveType, Normal, Special};

pub fn ryan_normals() -> HashMap<MoveType, Normal> {
    vec![
        (
            PUNCH,
            Normal {
                button: GameButton::Fast,
                stick: None,
            },
        ),
        (
            COMMAND_PUNCH,
            Normal {
                button: GameButton::Fast,
                stick: Some(StickPosition::E),
            },
        ),
    ]
    .into_iter()
    .collect()
}

pub fn ryan_specials() -> HashMap<MoveType, Special> {
    vec![(
        HADOUKEN,
        Special {
            motion: vec![2, 3, 6].into(),
            button: Some(GameButton::Fast),
        },
    )]
    .into_iter()
    .chain(universal_specials())
    .collect()
}

fn universal_specials() -> std::vec::IntoIter<(MoveType, Special)> {
    vec![
        (
            DASH_FORWARD,
            Special {
                motion: (vec![6, 5, 6], vec![7, 4, 1]).into(),
                button: None,
            },
        ),
        (
            DASH_BACK,
            Special {
                motion: (vec![4, 5, 4], vec![9, 6, 3]).into(),
                button: None,
            },
        ),
    ]
    .into_iter()
}
