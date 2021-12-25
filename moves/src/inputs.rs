use crate::ryan::*;
use crate::universal::*;

use bevy::utils::HashMap;
use types::MoveType;

pub fn ryan_inputs() -> HashMap<MoveType, &'static str> {
    vec![(HADOUKEN, "236f"), (PUNCH, "f"), (COMMAND_PUNCH, "6f")]
        .into_iter()
        .chain(universal_inputs())
        .collect()
}

fn universal_inputs() -> std::vec::IntoIter<(MoveType, &'static str)> {
    vec![(DASH_FORWARD, "656"), (DASH_BACK, "454")].into_iter()
}
