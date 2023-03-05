use bevy::prelude::*;
use characters::Situation;
use wag_core::Clock;

use super::PlayerQuery;

pub(super) fn move_advancement(clock: Res<Clock>, mut query: Query<PlayerQuery>) {
    for mut player in &mut query {
        let situation = Situation {
            inventory: &player.inventory,
            history: player
                .state
                .get_move_history()
                .map(|history| history.to_owned()),
            grounded: player.state.is_grounded(),
            properties: &player.properties,
            parser: &player.input_parser,
            current_frame: clock.frame,
            conditions: &player.state.get_conditions().to_owned(), // There is probably a smarter way to do this, but I can't really be bothered to think of it at this moment
        };

        player.state.proceed_move(situation);

        // Recover from the move if it's over
        if player.state.current_move_fully_handled() == Some(true) {
            player.state.recover(clock.frame)
        }
    }
}
