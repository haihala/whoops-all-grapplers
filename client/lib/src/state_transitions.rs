use bevy::{app::AppExit, asset::LoadState, prelude::*};

use characters::{Character, Inventory, ResourceType, WAGResources};
use input_parsing::InputParser;
use wag_core::{
    Clock, GameState, Joints, Player, RoundLog, RoundResult, POST_ROUND_DURATION,
    PRE_ROUND_DURATION, ROUNDS_TO_WIN, ROUND_MONEY, VICTORY_BONUS,
};

use crate::{assets::AssetsLoading, ui::Notifications};

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                end_loading.run_if(in_state(GameState::Loading)),
                end_claiming.run_if(in_state(GameState::ClaimingControllers)),
                end_combat.run_if(in_state(GameState::Combat)),
                transition_after_timer,
            ),
        );
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
    mut players: Query<(&WAGResources, &Player, &mut Inventory, &Character)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let round_over = players
        .iter()
        .filter_map(|(properties, player, _, _)| {
            if properties.get(ResourceType::Health).unwrap().is_empty() {
                None
            } else {
                Some(player)
            }
        })
        .count()
        != 2
        || clock.done();

    if !round_over {
        return;
    };

    let mut ordered_healths = (&mut players).into_iter().collect::<Vec<_>>();

    // TODO: There has to be a cleaner way This whole function could use a once-over. Many names seem outdated due to refactors elsewhere
    ordered_healths.sort_by_key(|(res, _, _, _)| {
        -(res
            .get(ResourceType::Health)
            .unwrap()
            .get_percentage()
            .round() as i32) // f32 doesn't implement ord, so sort doesn't work
    });

    assert!(ordered_healths.len() == 2);
    let [(winner_props, winner, winner_inventory, winner_character), (loser_props, loser, loser_inventory, loser_character)] =
        &mut ordered_healths[..]
    else {
        panic!("Couldn't unpack players");
    };

    for player in [Player::One, Player::Two] {
        notifications.add(player, format!("Round payout: ${}", ROUND_MONEY));

        let meter_money = if player == **winner {
            let meter_money = winner_props.get(ResourceType::Meter).unwrap().current;
            winner_inventory.money += meter_money as usize;
            meter_money
        } else {
            let meter_money = loser_props.get(ResourceType::Meter).unwrap().current;
            loser_inventory.money += meter_money as usize;
            meter_money
        };

        notifications.add(player, format!("Meter payout: ${}", meter_money));
    }

    winner_inventory.remove_one_round_consumables(winner_character);
    loser_inventory.remove_one_round_consumables(loser_character);

    winner_inventory.money += ROUND_MONEY;
    loser_inventory.money += ROUND_MONEY;

    let result = if winner_props
        .get(ResourceType::Health)
        .unwrap()
        .get_percentage()
        == loser_props
            .get(ResourceType::Health)
            .unwrap()
            .get_percentage()
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

    next_state.set(GameState::PostRound);
    commands.insert_resource(TransitionTimer {
        timer: Timer::from_seconds(POST_ROUND_DURATION, TimerMode::Once),
        exit_game: round_log.wins(**winner) >= ROUNDS_TO_WIN,
    })
}

fn transition_after_timer(
    mut commands: Commands,
    timer_resource: Option<ResMut<TransitionTimer>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(mut transition) = timer_resource {
        transition.timer.tick(time.delta());

        if transition.timer.finished() {
            if transition.exit_game {
                exit.send(AppExit);
            } else {
                next_state.set(game_state.get().next());
                commands.remove_resource::<TransitionTimer>()
            }
        }
    }
}

fn end_loading(
    players: Query<&Joints>,
    loading_assets: Res<AssetsLoading>,
    server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let joints_loaded = players.iter().all(|joints| !joints.nodes.is_empty());
    let some_assets_loading = !loading_assets.0.is_empty();
    let all_assets_loaded = loading_assets
        .0
        .iter()
        .all(|h| server.get_load_state(h.id()) == Some(LoadState::Loaded));

    if joints_loaded && some_assets_loading && all_assets_loaded {
        dbg!("Done loading assets");
        next_state.set(GameState::ClaimingControllers);
    }
}

fn end_claiming(
    mut commands: Commands,
    parsers: Query<&InputParser>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if parsers.iter().all(|parser| parser.is_ready()) {
        next_state.set(GameState::PreRound);
        commands.insert_resource(TransitionTimer::from(Timer::from_seconds(
            PRE_ROUND_DURATION,
            TimerMode::Once,
        )))
    }
}
