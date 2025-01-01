use bevy::{
    input::gamepad::gamepad_event_processing_system, prelude::*, utils::HashMap, window::WindowMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use characters::{ActionEvent, FlashRequest, GaugeType, Gauges, Hitbox, Hurtboxes, Inventory};
use foundation::{
    Area, Characters, Clock, Controllers, Dev, Facing, GameState, InputDevice, LocalCharacter,
    LocalController, LocalState, MatchState, OnlineState, Player, Players, RollbackSchedule, Sound,
    SoundRequest, Stats, StatusCondition, StatusFlag, SystemStep, WagArgs, GI_PARRY_FLASH_COLOR,
    KEYBOARD_MAGIC_CONSTANT,
};
use input_parsing::{InputParser, ParrotStream};
use strum::IntoEnumIterator;

use crate::{
    movement::{ObjectVelocity, PlayerVelocity, Pushbox},
    networking,
    player_state_management::MoveBuffer,
};

mod box_visualization;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .register_type::<Player>()
            // TODO FIXME Recursive type definition problem
            // .register_type::<PlayerState>()
            .register_type::<Clock>()
            .register_type::<PlayerVelocity>()
            .register_type::<ObjectVelocity>()
            .register_type::<Pushbox>()
            .register_type::<Area>()
            .register_type::<Hurtboxes>()
            .register_type::<Hitbox>()
            .register_type::<MoveBuffer>()
            .register_type::<Inventory>()
            .register_type::<Facing>()
            .register_type::<Stats>()
            .register_type::<InputParser>()
            .register_type::<ParrotStream>()
            .add_systems(Startup, setup_gizmos)
            // This needs access to gamepads, which don't in startup
            // It also needs to happen before any other gameplay systems
            // Too early, dev-local breaks. Too late and dev-synctest breaks.
            .add_systems(PreUpdate, skip_menus.after(gamepad_event_processing_system))
            .add_systems(
                RollbackSchedule,
                (
                    audio_test_system,
                    shader_test_system,
                    fullscreen_toggle,
                    pause_toggle,
                    kill_system,
                    reset_bind,
                )
                    .chain()
                    .in_set(SystemStep::DevTools),
            )
            .add_systems(
                Update,
                (
                    box_visualization::visualize_hitboxes,
                    box_visualization::visualize_hurtboxes,
                    box_visualization::visualize_pushboxes,
                    box_visualization::visualize_generic_areas,
                ),
            );
    }
}

fn setup_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.depth_bias = -1.0;
}

fn skip_menus(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_match_state: ResMut<NextState<MatchState>>,
    args: Res<WagArgs>,
    pad_query: Query<Entity, With<Gamepad>>,
) {
    let Some(dev_mode) = args.dev else {
        panic!("In dev plugin but not in dev mode")
    };

    if *game_state.get() != GameState::MainMenu {
        return;
    }

    info!("Skipping menus");

    let pads: HashMap<_, _> = pad_query
        .iter()
        .map(InputDevice::Controller)
        .enumerate()
        .chain([(KEYBOARD_MAGIC_CONSTANT, InputDevice::Keyboard)])
        .collect();

    match dev_mode {
        Dev::Online {
            local_controller,
            local_character,
        } => {
            next_game_state.set(GameState::Online(OnlineState::Lobby));
            networking::setup_socket(&mut commands);
            commands.insert_resource(LocalController(pads[&local_controller]));
            commands.insert_resource(LocalCharacter(local_character));
        }
        Dev::Local {
            pad1,
            pad2,
            character1,
            character2,
            starting_money: _,
        } => {
            next_game_state.set(GameState::Local(LocalState::Match));
            next_match_state.set(MatchState::Loading);
            commands.insert_resource(Controllers {
                p1: pads[&pad1],
                p2: pads[&pad2],
            });

            commands.insert_resource(Characters {
                p1: character1,
                p2: character2,
            })
        }
        Dev::Synctest {
            local_controller,
            local_character,
        } => {
            next_game_state.set(GameState::Synctest);
            next_match_state.set(MatchState::Loading);
            commands.insert_resource(LocalController(pads[&local_controller]));
            commands.insert_resource(LocalCharacter(local_character));
            commands.insert_resource(Controllers {
                p1: InputDevice::Online(0),
                p2: InputDevice::Online(1),
            });
            commands.insert_resource(Characters {
                p1: local_character,
                p2: local_character,
            });
            commands.run_system_cached(networking::start_synctest_session);
        }
    }
}

fn shader_test_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    players: Res<Players>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        info!("Playing shader flash");
        for player in Player::iter() {
            commands.trigger_targets(
                ActionEvent::Flash(FlashRequest {
                    color: GI_PARRY_FLASH_COLOR,
                    speed: 10.0,
                    ..default()
                }),
                players.get(player),
            );
        }
    }

    if keys.just_pressed(KeyCode::Digit5) {
        info!("Playing weaken shader");
        for player in Player::iter() {
            commands.trigger_targets(
                ActionEvent::Condition(StatusCondition {
                    flag: StatusFlag::Weaken,
                    expiration: Some(60),
                    ..default()
                }),
                players.get(player),
            );
        }
    }
}

fn audio_test_system(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Digit2) {
        info!("Playing whoosh audio");
        commands.trigger(SoundRequest::from(Sound::Whoosh));
    }
}

fn fullscreen_toggle(keys: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if keys.just_pressed(KeyCode::Digit3) {
        let mut win = windows.get_single_mut().unwrap();
        info!("Fullscreen toggle");

        win.mode = match win.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
            _ => win.mode,
        }
    }
}

fn pause_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
    clock: Res<Clock>,
    mut local_frame: Local<Option<usize>>,
) {
    if keys.just_pressed(KeyCode::Digit4) {
        if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            if time.relative_speed() == 0.0 {
                info!("Frame step");

                *local_frame = Some(clock.frame);
                time.set_relative_speed(1.0);
            }
        } else {
            info!("Pause toggle");
            let new_speed = 1.0 - time.relative_speed();
            time.set_relative_speed(new_speed);
        }
    }

    if let Some(frame) = *local_frame {
        if clock.frame > frame {
            time.set_relative_speed(0.0);
            *local_frame = None;
        }
    }
}

fn kill_system(keys: Res<ButtonInput<KeyCode>>, mut players: Query<(&Player, &mut Gauges)>) {
    let kill_p1 = keys.just_released(KeyCode::Digit6);
    let kill_p2 = keys.just_released(KeyCode::Digit7);

    if kill_p1 || kill_p2 {
        for (player, mut res) in &mut players {
            if (kill_p1 && *player == Player::One) || (kill_p2 && *player == Player::Two) {
                res.get_mut(GaugeType::Health).unwrap().drain(99999);
            }
        }
    }
}

fn reset_bind(
    keys: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Gauges>,
    mut clock: ResMut<Clock>,
) {
    let reset_pressed = keys.just_released(KeyCode::Digit0);

    if reset_pressed {
        for mut res in &mut players {
            res.get_mut(GaugeType::Health).unwrap().gain(99999);
        }

        clock.reset();
    }
}
