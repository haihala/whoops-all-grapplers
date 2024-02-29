use bevy::prelude::*;
use bitt::{Asserter, PlaybackTestGear, PlaybackTestingOptions};
use wag_core::{GameState, WagArgs};
use whoops_all_grapplers_lib::WAGLib;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WAGLib::with_args(WagArgs::default()),
            PlaybackTestGear::new(
                "sanity-check".into(),
                PlaybackTestingOptions { ..default() },
            ),
        ))
        .add_systems(Update, state_cycled)
        .run();
}

fn state_cycled(
    state: Res<State<GameState>>,
    mut asserter: ResMut<Asserter>,
    mut game_has_started: Local<bool>,
) {
    if state.get() == &GameState::PreRound && *game_has_started {
        asserter.pass();
    } else if state.get() == &GameState::Combat {
        *game_has_started = true;
    }
}
