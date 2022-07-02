use bevy::prelude::*;

mod primary_state;

mod player_state;
pub use crate::player_state::PlayerState;

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, _app: &mut App) {}
}
