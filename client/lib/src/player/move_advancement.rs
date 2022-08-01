use bevy::prelude::*;
use characters::Situation;
use time::Clock;

use super::PlayerQuery;

pub(super) fn move_advancement(clock: Res<Clock>, mut query: Query<PlayerQuery>) {
    for mut player in query.iter_mut() {
        let situation = Situation {
            inventory: &player.inventory,
            history: player
                .state
                .get_move_history()
                .map(|history| history.to_owned()),
            grounded: player.state.is_grounded(),
            resources: &player.resources,
            parser: &player.input_parser,
            current_frame: clock.frame,
        };

        player.state.proceed_move(situation);

        // Recover from the move if it's over
        if player.state.current_move_fully_handled().unwrap() {
            player.state.recover(clock.frame)
        }
    }
}
