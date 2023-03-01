use bevy::prelude::*;

use characters::{Action, Inventory};
use player_state::PlayerState;
use wag_core::{Clock, GameState, Player, RoundLog, RoundResult, ROUND_MONEY, VICTORY_BONUS};

#[derive(Reflect, Component, Clone, Copy)]
pub struct Health {
    value: usize,

    // As this is also stored elsewhere (StatusEffect), could maybe be removed from here in the future
    // TODO: Think about it
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

    pub fn reset(&mut self, max_health: i32) {
        self.max = max_health as usize;
        self.value = max_health as usize;
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
    clock: Res<Clock>,
    mut round_log: ResMut<RoundLog>,
    mut players: Query<(&Health, &Player, &mut Inventory)>,
    mut state: ResMut<State<GameState>>,
) {
    if *state.current() != GameState::Combat {
        return;
    }

    let round_over = players
        .iter()
        .filter_map(
            |(health, player, _)| {
                if health.value > 0 {
                    Some(player)
                } else {
                    None
                }
            },
        )
        .count()
        != 2
        || clock.done();

    if round_over {
        let mut ordered_healths = (&mut players).into_iter().collect::<Vec<_>>();

        ordered_healths.sort_by(|(a, _, _), (b, _, _)| {
            a.get_percentage()
                .partial_cmp(&b.get_percentage())
                .unwrap()
                .reverse()
        });

        assert!(ordered_healths.len() == 2);
        let [(winner_health, winner, winner_inventory), (loser_health, loser, loser_inventory)] = &mut ordered_healths[..] else {
            panic!("Couldn't unpack players");
        };

        let result = if winner_health.get_percentage() == loser_health.get_percentage() {
            // Tie
            RoundResult { winner: None }
        } else {
            winner_inventory.money += VICTORY_BONUS;

            RoundResult {
                winner: Some(**winner),
            }
        };

        round_log.add(result);

        winner_inventory.money += ROUND_MONEY + round_log.loss_bonus(**winner);
        loser_inventory.money += ROUND_MONEY + round_log.loss_bonus(**loser);

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
