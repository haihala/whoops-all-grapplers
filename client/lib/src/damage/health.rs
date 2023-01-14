use bevy::prelude::*;

use characters::Action;
use player_state::PlayerState;
use time::{Clock, GameState, RoundResult};
use wag_core::Player;

#[derive(Reflect, Component, Clone, Copy)]
pub struct Health {
    value: usize,
    max: usize,
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

    pub fn apply_damage(&mut self, amount: usize) {
        if amount > self.value {
            // Prevent underflow
            self.value = 0;
        } else {
            self.value -= amount;
        }
    }
}

pub fn check_dead(
    mut commands: Commands,
    clock: Res<Clock>,
    query: Query<(&Health, &Player)>,
    mut state: ResMut<State<GameState>>,
) {
    if *state.current() != GameState::Combat {
        return;
    }

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

    if round_over {
        commands.insert_resource(if living_players.len() == 1 {
            RoundResult {
                winner: Some(living_players[0]),
            }
        } else {
            RoundResult { winner: None }
        });

        state.set(GameState::PostRound).unwrap();
    }
}

pub(super) fn take_damage(mut query: Query<(&mut PlayerState, &mut Health)>) {
    for (mut state, mut health) in &mut query {
        for amount in state.drain_matching_actions(|action| {
            if let Action::TakeDamage(amount) = action {
                Some(*amount)
            } else {
                None
            }
        }) {
            health.apply_damage(amount);
        }
    }
}
