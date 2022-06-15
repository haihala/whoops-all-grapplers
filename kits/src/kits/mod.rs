mod equipment;
mod helpers;
mod kit;
mod ryan;

pub use kit::Kit;
pub use ryan::ryan_kit;

use equipment::get_equipment_move;
use helpers::{dash, jump};

pub fn all_kits() -> Vec<Kit> {
    vec![ryan_kit()]
}
