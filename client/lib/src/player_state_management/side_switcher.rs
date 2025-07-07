use bevy::prelude::*;
use foundation::{CharacterFacing, Players};
use player_state::PlayerState;

use crate::event_spreading::FlipVisuals;

pub fn sideswitcher(
    players: Res<Players>,
    mut query: Query<(&Transform, &mut CharacterFacing, &PlayerState)>,
) {
    if let Ok([(tf1, mut facing1, state1), (tf2, mut facing2, state2)]) =
        query.get_many_mut([players.one, players.two])
    {
        let p1_flipped = tf1.translation.x > tf2.translation.x;
        if facing1.absolute.to_flipped() != p1_flipped {
            facing1.absolute.set_flipped(p1_flipped);
            facing2.absolute.set_flipped(!p1_flipped);
        }

        if state1.can_update_visual_facing() {
            facing1.visual = facing1.absolute;
        }

        if state2.can_update_visual_facing() {
            facing2.visual = facing2.absolute;
        }
    }
}

pub fn flip_visuals(trigger: Trigger<FlipVisuals>, mut query: Query<&mut CharacterFacing>) {
    let mut facing = query.get_mut(trigger.target()).unwrap();
    facing.visual = facing.visual.opposite();
}
