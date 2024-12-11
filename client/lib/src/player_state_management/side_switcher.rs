use bevy::prelude::*;
use foundation::{Facing, Players};

pub fn sideswitcher(players: Res<Players>, mut query: Query<(&Transform, &mut Facing)>) {
    if let Ok([(tf1, mut facing1), (tf2, mut facing2)]) =
        query.get_many_mut([players.one, players.two])
    {
        let p1_flipped = tf1.translation.x > tf2.translation.x;
        if facing1.to_flipped() != p1_flipped {
            facing1.set_flipped(p1_flipped);
            facing2.set_flipped(!p1_flipped);
        }
    }
}
