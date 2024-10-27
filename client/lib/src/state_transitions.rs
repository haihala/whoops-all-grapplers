use std::time::Duration;

use bevy::{asset::LoadState, prelude::*, state::state::FreelyMutableState};

use characters::{Character, Inventory, ResourceType, WAGResources};
use input_parsing::InputParser;
use wag_core::{
    Clock, GameResult, GameState, InCharacterSelect, InMatch, MatchState, Player, RollbackSchedule,
    RoundLog, RoundResult, VoiceLine, WAGStage, POST_ROUND_DURATION, ROUNDS_TO_WIN, ROUND_MONEY,
    VICTORY_BONUS,
};

use crate::{
    assets::{Announcer, AssetsLoading, PlayerModelHook},
    event_spreading::{PlaySound, StartHitstop},
    ui::Notifications,
};

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<MatchState>()
            .add_computed_state::<InMatch>()
            .add_computed_state::<InCharacterSelect>()
            .add_systems(
                RollbackSchedule,
                (
                    end_loading.run_if(in_state(MatchState::Loading)),
                    end_combat.run_if(in_state(MatchState::Combat)),
                    clear_between_states.run_if(state_changed::<GameState>),
                    transition_after_timer::<GameState>,
                    transition_after_timer::<MatchState>,
                )
                    .chain()
                    .in_set(WAGStage::StateTransitions),
            );
    }
}

#[derive(Debug, Resource)]
pub struct TransitionTimer<T: States> {
    pub timer: Timer,
    pub state: T,
}

pub fn end_combat(
    mut commands: Commands,
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    mut announcer: ResMut<Announcer>,
    mut round_log: ResMut<RoundLog>,
    mut players: Query<(&WAGResources, &Player, &mut Inventory, &Character)>,
    mut next_match_state: ResMut<NextState<MatchState>>,
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
        announcer.tie();
        RoundResult { winner: None }
    } else {
        notifications.add(**winner, format!("Victory bonus: ${}", VICTORY_BONUS));
        winner_inventory.money += VICTORY_BONUS;

        let loss_bonus = round_log.loss_bonus(**loser);
        if loss_bonus > 0 {
            notifications.add(**loser, format!("Jobber bonus: ${}", loss_bonus));
            loser_inventory.money += loss_bonus;
        }

        commands.trigger(PlaySound(loser_character.get_voiceline(VoiceLine::Defeat)));

        announcer.round_win(**winner);
        commands.trigger(StartHitstop(Duration::from_secs_f32(POST_ROUND_DURATION)));
        RoundResult {
            winner: Some(**winner),
        }
    };

    round_log.add(result);

    let game_over = round_log.wins(**winner) >= ROUNDS_TO_WIN;

    if game_over {
        commands.insert_resource(GameResult { winner: **winner });
    }

    next_match_state.set(MatchState::PostRound);
    commands.insert_resource(TransitionTimer {
        timer: Timer::from_seconds(POST_ROUND_DURATION, TimerMode::Once),
        state: if game_over {
            MatchState::EndScreen
        } else {
            MatchState::Shop
        },
    });
}

fn transition_after_timer<T: FreelyMutableState>(
    mut commands: Commands,
    timer_resource: Option<ResMut<TransitionTimer<T>>>,
    mut next_state: ResMut<NextState<T>>,
) {
    if let Some(mut transition) = timer_resource {
        transition
            .timer
            .tick(Duration::from_millis((1000.0 / wag_core::FPS) as u64));

        if transition.timer.finished() {
            next_state.set(transition.state.clone());
            commands.remove_resource::<TransitionTimer<T>>()
        }
    }
}

fn end_loading(
    ready_players: Query<&Player>,
    hooked_children: Query<&PlayerModelHook>,
    loading_assets: Res<AssetsLoading>,
    server: Res<AssetServer>,
    mut next_match_state: ResMut<NextState<MatchState>>,
) {
    let two_players = ready_players.iter().count() == 2;
    let hooks_ran = hooked_children.iter().count() == 0;
    let asset_loads_started = !loading_assets.0.is_empty();
    let all_assets_loaded = loading_assets
        .0
        .iter()
        .all(|h| server.get_load_state(h.id()) == Some(LoadState::Loaded));

    if two_players && hooks_ran && asset_loads_started && all_assets_loaded {
        println!("Done loading assets");
        next_match_state.set(MatchState::PostLoad);
    }
}

fn clear_between_states(mut players: Query<&mut InputParser>) {
    for mut parser in &mut players.iter_mut() {
        parser.clear();
    }
}
