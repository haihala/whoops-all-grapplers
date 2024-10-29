use bevy::prelude::*;
use wag_core::ActionId;

use crate::ToHit;

#[derive(Component, Default, Clone, Debug, PartialEq)]
pub struct Attack {
    pub to_hit: ToHit,
    pub on_hit: usize,
    pub action_id: ActionId,
}

// Options
// - Include dyn_clone
// - Make a semi-dynamic system similar to move requirements
// - Stick with the current system, retarget projectiles some other way
// - Sneak hitbox spawnings out some other way (not as an action event)
//      - Lookup system, where on hit effects are separated from events to their own list
//      - Events refer to on hit effects via index
