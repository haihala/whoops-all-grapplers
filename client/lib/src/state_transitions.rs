use bevy::prelude::*;

use characters::Inventory;
use wag_core::{Clock, GameState, Player, RoundLog, RoundResult, ROUND_MONEY, VICTORY_BONUS};

use crate::{damage::Health, ui::Notifications};

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(end_combat.with_run_criteria(State::on_update(GameState::Combat)));
    }
}

pub fn end_combat(
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    mut round_log: ResMut<RoundLog>,
    mut players: Query<(&Health, &Player, &mut Inventory)>,
    mut state: ResMut<State<GameState>>,
) {
    let round_over = players
        .iter()
        .filter_map(|(health, player, _)| {
            if health.get_percentage() > 0.0 {
                Some(player)
            } else {
                None
            }
        })
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

        for player in [Player::One, Player::Two] {
            notifications.add(player, format!("Round payout: ${}", ROUND_MONEY));
        }

        winner_inventory.money += ROUND_MONEY;
        loser_inventory.money += ROUND_MONEY;

        let result = if winner_health.get_percentage() == loser_health.get_percentage() {
            // Tie
            RoundResult { winner: None }
        } else {
            notifications.add(**winner, format!("Victory bonus: ${}", VICTORY_BONUS));
            winner_inventory.money += VICTORY_BONUS;

            let loss_bonus = round_log.loss_bonus(**loser);
            if loss_bonus > 0 {
                notifications.add(**loser, format!("Jobber bonus: ${}", loss_bonus));
                loser_inventory.money += loss_bonus;
            }

            RoundResult {
                winner: Some(**winner),
            }
        };

        round_log.add(result);

        state.set(GameState::PostRound).unwrap();
    }
}
