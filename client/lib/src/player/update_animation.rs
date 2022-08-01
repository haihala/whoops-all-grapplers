use bevy::prelude::*;
use characters::{Action, Character};
use player_state::PlayerState;
use types::Facing;

use crate::assets::AnimationHelper;

#[allow(clippy::type_complexity)]
pub fn update_animation(
    mut query: Query<
        (&Character, &mut PlayerState, &Facing, &mut AnimationHelper),
        Or<(Changed<PlayerState>, Changed<Facing>)>,
    >,
) {
    for (character, mut state, facing, mut helper) in query.iter_mut() {
        if let Some(generic) = state.get_generic_animation(*facing) {
            helper.play(
                character
                    .generic_animations
                    .get(&generic)
                    .unwrap()
                    .to_owned(),
            );
        } else if let Some(move_animation) = state
            .drain_unprocessed_actions(|action| matches!(action, Action::Animation(_)))
            .into_iter()
            .map(|animation| {
                if let Action::Animation(ani) = animation {
                    ani
                } else {
                    panic!("Should never go here");
                }
            })
            .last()
        {
            helper.play(move_animation);
        } else {
            dbg!("No generic animation nor is a move ongoing?");
        }
    }
}
