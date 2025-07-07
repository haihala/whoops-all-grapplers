use bevy::{asset::LoadState, prelude::*, state::state::FreelyMutableState};

use characters::{Character, GaugeType, Gauges, Inventory};
use foundation::{
    Clock, GameResult, GameState, InCharacterSelect, InMatch, MatchState, Player, RollbackSchedule,
    RoundLog, RoundResult, Sound, SoundRequest, SystemStep, VoiceLine, BASE_ROUND_MONEY, FPS,
    MAX_COMBAT_DURATION, POST_ROUND_DURATION, PRE_ROUND_DURATION, ROUNDS_TO_WIN,
    ROUND_MONEY_BUILDUP, VICTORY_BONUS,
};
use input_parsing::InputParser;

use crate::{
    assets::{AnimationHelper, Announcer, AssetsLoading, Music, PlayerModelHook},
    ui::{self, Notifications},
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
                    .in_set(SystemStep::StateTransitions),
            );
    }
}

#[derive(Debug, Resource)]
pub struct TransitionTimer<T: States> {
    pub frame: usize,
    pub state: T,
}

#[allow(clippy::too_many_arguments)]
pub fn end_combat(
    mut commands: Commands,
    clock: Res<Clock>,
    mut notifications: ResMut<Notifications>,
    mut announcer: ResMut<Announcer>,
    mut round_log: ResMut<RoundLog>,
    mut players: Query<(&Gauges, &Player, &mut Inventory, &Character)>,
    mut next_match_state: ResMut<NextState<MatchState>>,
    mut music: ResMut<Music>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    let player_dead = players
        .iter()
        .any(|(properties, _, _, _)| properties.get(GaugeType::Health).unwrap().is_empty());

    let time_out = clock.relative_frame() as f32 / FPS >= MAX_COMBAT_DURATION;
    let round_over = player_dead || time_out;

    if !round_over {
        return;
    };

    let mut ordered_healths = (&mut players).into_iter().collect::<Vec<_>>();

    // TODO: There has to be a cleaner way This whole function could use a once-over. Many names seem outdated due to refactors elsewhere
    ordered_healths.sort_by_key(|(res, _, _, _)| {
        -(res.get(GaugeType::Health).unwrap().get_percentage().round() as i32) // f32 doesn't implement ord, so sort doesn't work
    });

    debug_assert!(ordered_healths.len() == 2);
    let [(winner_props, winner, winner_inventory, winner_character), (loser_props, _, loser_inventory, loser_character)] =
        &mut ordered_healths[..]
    else {
        panic!("Couldn't unpack players");
    };

    let round_money = BASE_ROUND_MONEY + ROUND_MONEY_BUILDUP * round_log.rounds_played();

    for player in [Player::One, Player::Two] {
        notifications.add(player, format!("Round payout: ${round_money}"));

        let meter_money = if player == **winner {
            let meter_money = winner_props.get(GaugeType::Meter).unwrap().current;
            winner_inventory.money += meter_money as usize;
            meter_money
        } else {
            let meter_money = loser_props.get(GaugeType::Meter).unwrap().current;
            loser_inventory.money += meter_money as usize;
            meter_money
        };

        notifications.add(player, format!("Meter payout: ${meter_money}"));
    }

    winner_inventory.remove_one_round_consumables(winner_character);
    loser_inventory.remove_one_round_consumables(loser_character);

    winner_inventory.money += round_money;
    loser_inventory.money += round_money;

    let result = if winner_props
        .get(GaugeType::Health)
        .unwrap()
        .get_percentage()
        == loser_props.get(GaugeType::Health).unwrap().get_percentage()
    {
        // Tie
        announcer.tie();
        RoundResult { winner: None }
    } else {
        notifications.add(**winner, format!("Victory bonus: ${VICTORY_BONUS}"));
        winner_inventory.money += VICTORY_BONUS;

        commands.trigger(SoundRequest::from(
            loser_character.get_voiceline(VoiceLine::Defeat),
        ));

        announcer.round_win(**winner);
        RoundResult {
            winner: Some(**winner),
        }
    };

    round_log.add(result);

    let game_over = round_log.wins(**winner) >= ROUNDS_TO_WIN;

    let next_state = if game_over {
        commands.insert_resource(GameResult { winner: **winner });
        music.pop();

        MatchState::EndScreen
    } else {
        music.push(Sound::WaitingMusic);

        MatchState::Shop
    };

    next_match_state.set(MatchState::PostRound);

    for mut anim_player in &mut animation_players {
        anim_player.pause_all();
    }

    commands.insert_resource(TransitionTimer {
        frame: clock.frame + (FPS * POST_ROUND_DURATION) as usize,
        state: next_state,
    });
}

fn transition_after_timer<T: FreelyMutableState>(
    mut commands: Commands,
    timer_resource: Option<Res<TransitionTimer<T>>>,
    mut next_state: ResMut<NextState<T>>,
    clock: Res<Clock>,
) {
    if let Some(transition) = timer_resource {
        if transition.frame <= clock.frame {
            next_state.set(transition.state.clone());
            commands.remove_resource::<TransitionTimer<T>>()
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn end_loading(
    ready_players: Query<&AnimationHelper>,
    hooked_children: Query<&PlayerModelHook>,
    loading_assets: Res<AssetsLoading>,
    server: Res<AssetServer>,
    mut next_match_state: ResMut<NextState<MatchState>>,
    mut commands: Commands,
    mut announcer: ResMut<Announcer>,
    clock: Res<Clock>,
) {
    let two_players = ready_players.iter().count() == 2;
    let hooks_ran = hooked_children.iter().count() == 0;
    let asset_loads_started = !loading_assets.0.is_empty();
    let all_assets_loaded = loading_assets.0.iter().all(|h| {
        server
            .get_load_state(h.id())
            .map(|inner| matches!(inner, LoadState::Loaded))
            .unwrap_or_default()
    });

    if two_players && hooks_ran && asset_loads_started && all_assets_loaded {
        info!("Done loading assets");

        announcer.round_start(1);
        commands.run_system_cached(ui::setup_shop);
        commands.run_system_cached(ui::setup_combat_hud);

        next_match_state.set(MatchState::PreRound);
        commands.insert_resource(TransitionTimer {
            frame: clock.frame + (FPS * PRE_ROUND_DURATION) as usize,
            state: MatchState::Combat,
        });
    }
}

fn clear_between_states(mut players: Query<&mut InputParser>) {
    for mut parser in &mut players.iter_mut() {
        parser.clear();
    }
}
