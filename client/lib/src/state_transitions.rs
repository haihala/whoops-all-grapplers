use bevy::{app::AppExit, prelude::*};

use characters::{Inventory, Properties};
use input_parsing::InputParser;
use wag_core::{
    Clock, GameState, Player, RoundLog, RoundResult, POST_ROUND_DURATION, PRE_ROUND_DURATION,
    ROUNDS_TO_WIN, ROUND_MONEY, VICTORY_BONUS,
};

use crate::ui::Notifications;

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(end_loading.with_run_criteria(State::on_update(GameState::Loading)))
            .add_system(end_combat.with_run_criteria(State::on_update(GameState::Combat)))
            .add_system(transition_after_timer);
    }
}

#[derive(Debug, Resource)]
pub struct TransitionTimer {
    timer: Timer,
    exit_game: bool,
}
impl From<Timer> for TransitionTimer {
    fn from(timer: Timer) -> Self {
        Self {
            timer,
            exit_game: false,
        }
    }
}

pub fn end_combat(
    mut commands: Commands,
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    mut round_log: ResMut<RoundLog>,
    mut players: Query<(&Properties, &Player, &mut Inventory)>,
    mut state: ResMut<State<GameState>>,
) {
    let round_over = players
        .iter()
        .filter_map(|(properties, player, _)| {
            if properties.health.is_empty() {
                None
            } else {
                Some(player)
            }
        })
        .count()
        != 2
        || clock.done();

    if round_over {
        let mut ordered_healths = (&mut players).into_iter().collect::<Vec<_>>();

        ordered_healths.sort_by(|(a, _, _), (b, _, _)| {
            a.health
                .get_percentage()
                .partial_cmp(&b.health.get_percentage())
                .unwrap()
                .reverse()
        });

        assert!(ordered_healths.len() == 2);
        let [(winner_props, winner, winner_inventory), (loser_props, loser, loser_inventory)] = &mut ordered_healths[..] else {
            panic!("Couldn't unpack players");
        };

        for player in [Player::One, Player::Two] {
            notifications.add(player, format!("Round payout: ${}", ROUND_MONEY));
        }

        winner_inventory.money += ROUND_MONEY;
        loser_inventory.money += ROUND_MONEY;

        let result = if winner_props.health.get_percentage() == loser_props.health.get_percentage()
        {
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
        commands.insert_resource(TransitionTimer {
            timer: Timer::from_seconds(POST_ROUND_DURATION, TimerMode::Once),
            exit_game: round_log.wins(**winner) >= ROUNDS_TO_WIN,
        })
    }
}

fn transition_after_timer(
    mut commands: Commands,
    timer_resource: Option<ResMut<TransitionTimer>>,
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(mut transition) = timer_resource {
        transition.timer.tick(time.delta());

        if transition.timer.finished() {
            if transition.exit_game {
                exit.send(AppExit);
            } else {
                let next_state = game_state.current().next();
                game_state.set(next_state).unwrap();
                commands.remove_resource::<TransitionTimer>()
            }
        }
    }
}

fn end_loading(
    mut commands: Commands,
    parsers: Query<&InputParser>,
    mut game_state: ResMut<State<GameState>>,
) {
    if parsers.iter().all(|parser| parser.is_ready()) {
        game_state.set(GameState::PreRound).unwrap();
        commands.insert_resource(TransitionTimer::from(Timer::from_seconds(
            PRE_ROUND_DURATION,
            TimerMode::Once,
        )))
    }
}
