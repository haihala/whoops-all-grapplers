use bevy::prelude::*;
use characters::Character;
use player_state::PlayerState;
use types::Facing;

use crate::assets::AnimationHelper;

#[allow(clippy::type_complexity)]
pub fn update_animation(
    mut query: Query<
        (&Character, &PlayerState, &Facing, &mut AnimationHelper),
        Or<(Changed<PlayerState>, Changed<Facing>)>,
    >,
) {
    for (character, state, facing, mut helper) in query.iter_mut() {
        if let Some(generic) = state.get_generic_animation(*facing) {
            helper.play(
                character
                    .generic_animations
                    .get(&generic)
                    .unwrap()
                    .to_owned(),
            );
        } else if let Some(ongoing_move) = state.get_move_state() {
            let move_data = character.get_move(ongoing_move.move_id);
            helper.play(move_data.get_animation(ongoing_move));
        } else {
            dbg!("How did we get here?");
        }
    }
}
