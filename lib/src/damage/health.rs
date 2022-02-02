use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use time::{Clock, GameState, RoundResult};
use types::Player;

#[derive(Inspectable, Component, Clone, Copy)]
pub struct Health {
    ratio: f32,
    value: i32,
    max: i32,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            ratio: 1.0,
            value: 100,
            max: 100,
        }
    }
}
impl Health {
    pub fn get_ratio(&self) -> f32 {
        self.ratio
    }

    pub fn reset(&mut self) {
        *self = Health::default();
    }

    pub fn apply_damage(&mut self, amount: i32) {
        self.value -= amount;
        self.ratio = self.value as f32 / self.max as f32;
    }
}

pub fn check_dead(
    mut commands: Commands,
    clock: Res<Clock>,
    query: Query<(&Health, &Player)>,
    mut state: ResMut<State<GameState>>,
) {
    let living_players: Vec<Player> = query
        .iter()
        .filter_map(|(health, player)| {
            if health.value > 0 {
                Some(player.to_owned())
            } else {
                None
            }
        })
        .collect();

    if living_players.len() != 2 || clock.time_out() {
        commands.insert_resource(if living_players.len() == 1 {
            RoundResult {
                winner: Some(living_players[0]),
            }
        } else {
            RoundResult { winner: None }
        });

        // FIXME: This gave an error while I was fixing other stuff, may or may not actually be broken, likely related to round ending by time if it is.
        // thread 'Compute Task Pool (0)' panicked at 'called `Result::unwrap()` on an `Err` value: AlreadyInState', lib\src\damage\health.rs:63:41
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        state.set(GameState::PostRound).unwrap();
    }
}
