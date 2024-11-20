use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use characters::{
    ActionEvent, FlashRequest, Hitbox, Hurtboxes, Inventory, ResourceType, WAGResources,
};
use input_parsing::{InputParser, PadStream, ParrotStream};
use strum::IntoEnumIterator;
use wag_core::{
    Area, Characters, Clock, Controllers, Dev, Facing, GameState, LocalCharacter, LocalController,
    LocalState, MatchState, OnlineState, Player, Players, SoundEffect, Stats, StatusCondition,
    StatusFlag, WagArgs, GI_PARRY_FLASH_COLOR,
};

use crate::{
    event_spreading::PlaySound,
    movement::{ObjectVelocity, PlayerVelocity, Pushbox},
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
            .register_type::<PadStream>()
            .register_type::<ParrotStream>()
            .add_systems(Startup, setup_gizmos)
            .add_systems(PostStartup, skip_menus)
            .add_systems(
                Update,
                (
                    audio_test_system,
                    shader_test_system,
                    fullscreen_toggle,
                    pause_toggle,
                    kill_system,
                    box_visualization::visualize_hitboxes,
                    box_visualization::visualize_hurtboxes,
                    box_visualization::visualize_pushboxes,
                )
                    .chain(),
            );
    }
}

fn setup_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.depth_bias = -1.0;
}

fn skip_menus(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_match_state: ResMut<NextState<MatchState>>,
    args: Res<WagArgs>,
) {
    let Some(dev_mode) = args.dev else {
        panic!("In dev plugin but not in dev mode")
    };

    match dev_mode {
        Dev::Online {
            local_controller,
            local_character,
        } => {
            next_game_state.set(GameState::Online(OnlineState::Lobby));
            commands.insert_resource(LocalController(local_controller));
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
            commands.insert_resource(Controllers { p1: pad1, p2: pad2 });

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
            commands.insert_resource(LocalController(local_controller));
            commands.insert_resource(LocalCharacter(local_character));
            commands.insert_resource(Controllers::default());
            commands.insert_resource(Characters {
                p1: local_character,
                p2: local_character,
            });
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
        commands.trigger(PlaySound(SoundEffect::Whoosh));
    }
}

fn fullscreen_toggle(keys: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if keys.just_pressed(KeyCode::Digit3) {
        let mut win = windows.get_single_mut().unwrap();
        info!("Fullscreen toggle");

        win.mode = match win.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            WindowMode::BorderlessFullscreen => WindowMode::Windowed,
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

fn kill_system(keys: Res<ButtonInput<KeyCode>>, mut players: Query<(&Player, &mut WAGResources)>) {
    let kill_p1 = keys.just_released(KeyCode::Digit6);
    let kill_p2 = keys.just_released(KeyCode::Digit7);

    if kill_p1 || kill_p2 {
        for (player, mut res) in &mut players {
            if (kill_p1 && *player == Player::One) || (kill_p2 && *player == Player::Two) {
                res.get_mut(ResourceType::Health).unwrap().drain(99999);
            }
        }
    }
}
