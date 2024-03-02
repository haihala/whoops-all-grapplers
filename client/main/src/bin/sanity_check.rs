use bevy::prelude::*;
use bitt::{PlaybackTestGear, PlaybackTestingOptions, TestWrangler};
use wag_core::{GameState, WagArgs};
use whoops_all_grapplers_lib::WAGLib;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WAGLib::with_args(WagArgs::default()),
            PlaybackTestGear::new(
                "sanity-check".into(),
                PlaybackTestingOptions {
                    manual_start: true,
                    ..default()
                },
            ),
        ))
        .add_systems(Update, state_cycled)
        .run();
}

fn state_cycled(
    state: Res<State<GameState>>,
    mut wrangler: ResMut<TestWrangler>,
    mut game_has_started: Local<bool>,
) {
    if state.get() == &GameState::ClaimingControllers {
        wrangler.start();
    } else if state.get() == &GameState::PreRound && *game_has_started {
        wrangler.pass();
    } else if state.get() == &GameState::Combat {
        *game_has_started = true;
    }
}
