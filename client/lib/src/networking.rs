use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    input::{gamepad::GamepadEvent, keyboard::KeyboardInput},
    prelude::*,
    utils::HashMap,
};
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use characters::WAGResources;
use input_parsing::{InputParser, PadStream};
use player_state::PlayerState;
use strum::IntoEnumIterator;
use wag_core::{
    Characters, Clock, Controllers, Facing, GameState, LocalCharacter, LocalController,
    OnlineState, RollbackSchedule, WagInputButton, WagInputEvent,
};

use crate::{
    damage::{Defense, HitboxSpawner},
    movement::PlayerVelocity,
    player_state_management::MoveBuffer,
};

type Config = bevy_ggrs::GgrsConfig<u16, PeerId>;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WagInputEvent>()
            .add_systems(OnEnter(GameState::Online(OnlineState::Lobby)), setup_socket)
            .add_systems(
                FixedUpdate,
                wait_for_players.run_if(in_state(GameState::Online(OnlineState::Lobby))),
            )
            .add_systems(ReadInputs, read_local_inputs)
            .init_schedule(RollbackSchedule)
            .edit_schedule(RollbackSchedule, |schedule| {
                schedule.set_build_settings(ScheduleBuildSettings {
                    ambiguity_detection: LogLevel::Error,
                    hierarchy_detection: LogLevel::Error,
                    ..default()
                });
            })
            .add_systems(
                GgrsSchedule,
                (generate_online_input_streams, run_rollback_schedule)
                    .chain()
                    .run_if(|session: Option<Res<bevy_ggrs::Session<Config>>>| session.is_some()),
            )
            .add_systems(
                FixedUpdate,
                (generate_offline_input_streams, run_rollback_schedule)
                    .chain()
                    .run_if(|session: Option<Res<bevy_ggrs::Session<Config>>>| session.is_none()),
            )
            .add_plugins(GgrsPlugin::<Config>::default())
            // Probably an incomplete list of things to roll back
            .rollback_resource_with_copy::<Clock>()
            .rollback_component_with_clone::<PlayerState>()
            .rollback_component_with_clone::<PadStream>()
            .rollback_component_with_clone::<InputParser>()
            .rollback_component_with_clone::<WAGResources>()
            .rollback_component_with_clone::<PlayerVelocity>()
            .rollback_component_with_clone::<MoveBuffer>()
            .rollback_component_with_copy::<HitboxSpawner>()
            .rollback_component_with_copy::<Defense>()
            .rollback_component_with_copy::<Facing>()
            .rollback_component_with_copy::<Transform>();
    }
}

fn run_rollback_schedule(world: &mut World) {
    world.run_schedule(RollbackSchedule);
}

fn setup_socket(mut commands: Commands) {
    let room_url = "ws://wag.tunk.org:3536/wag?next=2";
    info!("connecting to matchbox server: {room_url}");
    let sock = WebRtcSocketBuilder::new(room_url)
        .add_reliable_channel()
        .add_ggrs_channel()
        .build();
    commands.insert_resource(MatchboxSocket::from(sock));
}

#[derive(Debug, Default)]
enum ConnectionState {
    #[default]
    WaitingToEstablish,
    CharacterSync,
    StartSession,
}

fn wait_for_players(
    mut commands: Commands,
    mut connection_state: Local<ConnectionState>,
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
    local_character: Res<LocalCharacter>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match &mut *connection_state {
        ConnectionState::WaitingToEstablish => {
            // Check for new connections
            socket.update_peers();
            let players = socket.players();

            if players.len() >= 2 {
                *connection_state = ConnectionState::CharacterSync;
            }
        }
        ConnectionState::CharacterSync => {
            let (peer_index, peer) = socket
                .players()
                .into_iter()
                .enumerate()
                .find_map(|(i, p)| match p {
                    ggrs::PlayerType::Remote(peer) => Some((i, peer)),
                    _ => None,
                })
                .unwrap();

            socket
                .channel_mut(0)
                .send(Box::new([local_character.0.into()]), peer);

            let contents = loop {
                let data = socket.channel_mut(0).receive();
                if data.is_empty() {
                    continue;
                }

                break data[0].1.clone();
            };

            // First to join is index 0 -> player 1
            let chars = if peer_index == 0 {
                Characters {
                    p1: contents[0].into(),
                    p2: local_character.0,
                }
            } else if peer_index == 1 {
                Characters {
                    p1: local_character.0,
                    p2: contents[0].into(),
                }
            } else {
                // I'm assuming only valid indices are 0 and 1
                // I think this will break if spectating is introduced
                dbg!(peer_index);
                panic!("Peer index is not 0 or 1");
            };

            commands.insert_resource(chars);
            commands.insert_resource(Controllers { p1: 0, p2: 1 });
            *connection_state = ConnectionState::StartSession;
        }
        ConnectionState::StartSession => {
            let mut session_builder = ggrs::SessionBuilder::<Config>::new()
                .with_num_players(2)
                .with_input_delay(2);

            for (i, player) in socket.players().into_iter().enumerate() {
                session_builder = session_builder
                    .add_player(player, i)
                    .expect("failed to add player");
            }

            // move the channel out of the socket (required because GGRS takes ownership of it)
            let channel = socket.take_channel(1).unwrap();

            // start the GGRS session
            let ggrs_session = session_builder
                .start_p2p_session(channel)
                .expect("failed to start session");

            commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));

            next_state.set(GameState::Online(OnlineState::Loading));
        }
    };
}

fn read_local_inputs(
    mut commands: Commands,
    gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    keyboard_keys: Res<ButtonInput<KeyCode>>,
    maybe_controller: Option<Res<LocalController>>,
    local_players: Res<LocalPlayers>,
    local_controller: Res<LocalController>,
) {
    let Some(controller) = maybe_controller else {
        return;
    };

    let gamepad = Gamepad { id: controller.0 };

    let mut inputs = HashMap::new();

    // There is only ever one, but the value can be 1 or 0
    for handle in &local_players.0 {
        let mut input = 0u16;

        // TODO: Analog stick
        for (shift, wag_button) in WagInputButton::iter().enumerate() {
            if gamepad_buttons.pressed(GamepadButton {
                gamepad,
                button_type: wag_button.to_gamepad_button_type(),
            }) {
                input |= 1 << shift;
            }
        }

        // Keyboard
        if local_controller.0 == 69 {
            for (shift, wag_button) in WagInputButton::iter().enumerate() {
                if keyboard_keys.pressed(wag_button.to_keycode()) {
                    input |= 1 << shift;
                }
            }
        }

        inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(inputs));
}

fn generate_offline_input_streams(
    mut writer: EventWriter<WagInputEvent>,
    mut gamepad_events: EventReader<GamepadEvent>,
    mut keyboard_events: EventReader<KeyboardInput>,
) {
    // TODO: Analog input
    for event in gamepad_events.read() {
        if let GamepadEvent::Button(btn_ev) = event {
            let Some(button) = WagInputButton::from_gamepad_button_type(btn_ev.button_type) else {
                dbg!("Discarded input", btn_ev);
                continue;
            };

            writer.send(WagInputEvent {
                pressed: btn_ev.value > 0.5,
                player_handle: btn_ev.gamepad.id,
                button,
            });
        }
    }

    for event in keyboard_events.read() {
        let Some(button) = WagInputButton::from_key(event.key_code) else {
            dbg!("Pressed non-mapped key", event.key_code);
            continue;
        };

        writer.send(WagInputEvent {
            pressed: event.state.is_pressed(),
            player_handle: 69, // Hehe special id for keyboard
            button,
        });
    }
}

fn generate_online_input_streams(
    mut writer: EventWriter<WagInputEvent>,
    inputs: Res<PlayerInputs<Config>>,
    mut input_states: Local<HashMap<usize, u16>>,
) {
    for (player_handle, (input, _)) in inputs.iter().enumerate() {
        let Some(old_state) = input_states.get(&player_handle) else {
            input_states.insert(player_handle, 0);
            continue;
        };

        for (shift, button_type) in WagInputButton::iter().enumerate() {
            let was_pressed = ((old_state >> shift) & 1) == 1;
            let is_pressed = ((input >> shift) & 1) == 1;

            if was_pressed != is_pressed {
                writer.send(WagInputEvent {
                    player_handle,
                    pressed: is_pressed,
                    button: button_type,
                });
            }
        }

        input_states.insert(player_handle, *input);
    }
}
