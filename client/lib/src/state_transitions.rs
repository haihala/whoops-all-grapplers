use std::time::Duration;

use bevy::{asset::LoadState, prelude::*};

use characters::{Character, Inventory, ResourceType, WAGResources};
use input_parsing::InputParser;
use wag_core::{
    Clock, GameResult, GameState, InCharacterSelect, InCombat, InEndScreen, InLoadingScreen,
    InMatch, InMatchSetup, InMenu, Joints, LocalState, MatchState, OnlineState, Player,
    RollbackSchedule, RoundLog, RoundResult, SynctestState, WAGStage, POST_ROUND_DURATION,
    ROUNDS_TO_WIN, ROUND_MONEY, VICTORY_BONUS,
};

use crate::{assets::AssetsLoading, ui::Notifications};

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_computed_state::<InMatch>()
            .add_computed_state::<InMenu>()
            .add_computed_state::<InCombat>()
            .add_computed_state::<InEndScreen>()
            .add_computed_state::<InLoadingScreen>()
            .add_computed_state::<InMatchSetup>()
            .add_computed_state::<InCharacterSelect>()
            .add_computed_state::<MatchState>()
            .add_systems(
                RollbackSchedule,
                (
                    end_loading.run_if(in_state(InLoadingScreen)),
                    end_combat.run_if(in_state(InCombat)),
                    clear_between_states.run_if(state_changed::<GameState>),
                    transition_after_timer,
                )
                    .chain()
                    .in_set(WAGStage::StateTransitions),
            );
    }
}

#[derive(Debug, Resource)]
pub struct TransitionTimer {
    pub timer: Timer,
    pub state: GameState,
}

pub fn end_combat(
    mut commands: Commands,
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    mut round_log: ResMut<RoundLog>,
    mut players: Query<(&WAGResources, &Player, &mut Inventory, &Character)>,
    game_state: Res<State<GameState>>,
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

    let game_over = round_log.wins(**winner) >= ROUNDS_TO_WIN;

    if game_over {
        commands.insert_resource(GameResult { winner: **winner });
    }

    let (next, after) = match **game_state {
        GameState::Local(_) => (
            GameState::Local(LocalState::Match(MatchState::PostRound)),
            GameState::Local(if game_over {
                LocalState::EndScreen
            } else {
                LocalState::Match(MatchState::Shop)
            }),
        ),
        GameState::Online(_) => (
            GameState::Online(OnlineState::Match(MatchState::PostRound)),
            GameState::Online(if game_over {
                OnlineState::EndScreen
            } else {
                OnlineState::Match(MatchState::Shop)
            }),
        ),
        GameState::Synctest(_) => (
            GameState::Synctest(SynctestState::Match(MatchState::PostRound)),
            GameState::Synctest(if game_over {
                SynctestState::EndScreen
            } else {
                SynctestState::Match(MatchState::Shop)
            }),
        ),
        _ => panic!("Out of match transitions!"),
    };
    next_state.set(next);

    commands.insert_resource(TransitionTimer {
        timer: Timer::from_seconds(POST_ROUND_DURATION, TimerMode::Once),
        state: after,
    })
}

fn transition_after_timer(
    mut commands: Commands,
    timer_resource: Option<ResMut<TransitionTimer>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(mut transition) = timer_resource {
        transition
            .timer
            .tick(Duration::from_millis((1000.0 / wag_core::FPS) as u64));

        if transition.timer.finished() {
            next_state.set(transition.state);
            commands.remove_resource::<TransitionTimer>()
        }
    }
}

fn end_loading(
    players: Query<&Joints>,
    loading_assets: Res<AssetsLoading>,
    server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    mut latch: Local<bool>,
) {
    let two_players = players.iter().count() == 2;
    let joints_loaded = players.iter().all(|joints| !joints.nodes.is_empty());
    let some_assets_loading = !loading_assets.0.is_empty();
    let all_assets_loaded = loading_assets
        .0
        .iter()
        .all(|h| server.get_load_state(h.id()) == Some(LoadState::Loaded));

    if two_players && joints_loaded && some_assets_loading && all_assets_loaded {
        if *latch {
            println!("Done loading assets");
            next_state.set(match *current_state.get() {
                GameState::Local(_) => GameState::Local(LocalState::SetupMatch),
                GameState::Online(_) => GameState::Online(OnlineState::SetupMatch),
                GameState::Synctest(_) => GameState::Synctest(SynctestState::SetupMatch),
                _ => panic!("Loading while not in a loading situation"),
            });
        } else {
            *latch = true;
        }
    }
}

fn clear_between_states(mut players: Query<&mut InputParser>) {
    for mut parser in &mut players.iter_mut() {
        parser.clear();
    }
}
