use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use core::Player;
use time::{Clock, GameState, RoundResult};

#[derive(Inspectable, Component, Clone, Copy)]
pub struct Health {
    value: i32,
    max: i32,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            value: 100,
            max: 100,
        }
    }
}
impl Health {
    pub fn get_percentage(&self) -> f32 {
        (self.value as f32 / self.max as f32) * 100.0
    }

    pub fn reset(&mut self) {
        *self = Health::default();
    }

    pub fn apply_damage(&mut self, amount: i32) {
        self.value -= amount;
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

    let round_over = living_players.len() != 2 || clock.time_out();

    if *state.current() == GameState::Combat && round_over {
        commands.insert_resource(if living_players.len() == 1 {
            RoundResult {
                winner: Some(living_players[0]),
            }
        } else {
            RoundResult { winner: None }
        });

        state.set(GameState::Shop).unwrap();
    }
}
