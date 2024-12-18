use std::hash::{Hash, Hasher};

use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    utils::HashMap,
};
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use characters::{Attack, Gauges, Hitbox, Hurtboxes, Inventory};
use foundation::{
    Area, CharacterClock, CharacterFacing, Characters, Clock, Combo, Controllers, GameState,
    Hitstop, InputDevice, InputState, InputStream, LocalCharacter, LocalController, MatchState,
    NetworkInputButton, OnlineState, OwnedInput, Owner, Pickup, Player, RollbackSchedule, Stats,
    WagArgs, STICK_DEAD_ZONE,
};
use input_parsing::{InputParser, ParrotStream};
use player_state::PlayerState;
use strum::IntoEnumIterator;

use crate::{
    assets::{AnimationHelper, CharacterShake, ExtendedFlashMaterial},
    camera::{ChildCameraEffects, RootCameraEffects},
    damage::{HitTracker, HitboxSpawner, LifetimeFlags, ProjectileMarker},
    entity_management::DespawnMarker,
    movement::{Follow, ObjectVelocity, PlayerVelocity, Pushbox, Walls},
    player_state_management::MoveBuffer,
};

type Config = bevy_ggrs::GgrsConfig<u16, PeerId>;

pub struct NetworkPlugin;

fn session_exists(session: Option<Res<bevy_ggrs::Session<Config>>>) -> bool {
    session.is_some()
}
fn no_session_exists(session: Option<Res<bevy_ggrs::Session<Config>>>) -> bool {
    session.is_none()
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputStream>()
            .add_systems(OnEnter(GameState::Online(OnlineState::Lobby)), setup_socket)
            .add_systems(OnExit(GameState::Online(OnlineState::Match)), teardown)
            .add_systems(
                FixedUpdate,
                wait_for_players.run_if(in_state(GameState::Online(OnlineState::Lobby))),
            )
            .add_systems(
                FixedUpdate,
                start_synctest_session.run_if(in_synctest_postload),
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
                (
                    generate_online_input_streams,
                    run_rollback_schedule,
                    handle_ggrs_events,
                    clear_input_stream,
                )
                    .chain()
                    .run_if(session_exists),
            )
            .add_systems(
                FixedUpdate,
                (
                    generate_offline_input_streams,
                    run_rollback_schedule,
                    clear_input_stream,
                )
                    .chain()
                    .run_if(no_session_exists),
            )
            .add_plugins(GgrsPlugin::<Config>::default())
            // Probably an incomplete list of things to roll back
            // Resources
            .rollback_resource_with_clone::<InputStream>()
            .rollback_resource_with_copy::<Clock>()
            .rollback_resource_with_copy::<Walls>()
            // Player components
            .rollback_component_with_clone::<Gauges>()
            .rollback_component_with_clone::<Hurtboxes>()
            .rollback_component_with_clone::<InputParser>()
            .rollback_component_with_clone::<Inventory>()
            .rollback_component_with_clone::<MoveBuffer>()
            .rollback_component_with_clone::<ParrotStream>()
            .rollback_component_with_clone::<PlayerState>()
            .rollback_component_with_clone::<PlayerVelocity>()
            .rollback_component_with_copy::<AnimationHelper>()
            .rollback_component_with_copy::<CharacterClock>()
            .rollback_component_with_copy::<CharacterFacing>()
            .rollback_component_with_copy::<CharacterShake>()
            .rollback_component_with_copy::<Combo>()
            .rollback_component_with_copy::<HitboxSpawner>()
            .rollback_component_with_copy::<Hitstop>()
            .rollback_component_with_copy::<Player>()
            .rollback_component_with_copy::<Pushbox>()
            .rollback_component_with_copy::<Stats>()
            // Hitboxes
            .rollback_component_with_clone::<Attack>()
            .rollback_component_with_copy::<DespawnMarker>()
            .rollback_component_with_copy::<Follow>()
            .rollback_component_with_copy::<HitTracker>()
            .rollback_component_with_copy::<Hitbox>()
            .rollback_component_with_copy::<LifetimeFlags>()
            .rollback_component_with_copy::<ObjectVelocity>()
            .rollback_component_with_copy::<Owner>()
            .rollback_component_with_copy::<ProjectileMarker>()
            // Pickups
            .rollback_component_with_copy::<Area>()
            .rollback_component_with_copy::<Pickup>()
            // Camera
            .rollback_component_with_clone::<ChildCameraEffects>()
            .rollback_component_with_clone::<RootCameraEffects>()
            // Bevy inbuilts
            .rollback_component_with_clone::<Name>()
            .rollback_component_with_clone::<StateScoped<GameState>>()
            .rollback_component_with_clone::<StateScoped<MatchState>>()
            .rollback_component_with_copy::<GlobalTransform>()
            .rollback_component_with_copy::<InheritedVisibility>()
            .rollback_component_with_copy::<Transform>()
            .rollback_component_with_copy::<ViewVisibility>()
            .rollback_component_with_copy::<Visibility>()
            .rollback_component_with_reflect::<AnimationPlayer>()
            .rollback_component_with_reflect::<AnimationTransitions>()
            .rollback_component_with_reflect::<Mesh3d>()
            .rollback_component_with_reflect::<MeshMaterial3d<StandardMaterial>>()
            .rollback_component_with_reflect::<MeshMaterial3d<ExtendedFlashMaterial>>()
            .rollback_component_with_reflect::<SceneRoot>()
            // Checksums
            .checksum_component::<Transform>(tf_hasher)
            .checksum_resource::<Clock>(clock_hasher)
            .checksum_component_with_hash::<PlayerState>();
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

fn teardown(mut commands: Commands) {
    commands.remove_resource::<MatchboxSocket<MultipleChannels>>();
    commands.remove_resource::<bevy_ggrs::Session<Config>>();
    commands.remove_resource::<bevy_ggrs::LocalInputs<Config>>();

    commands.remove_resource::<Characters>();
    commands.remove_resource::<Controllers>();
    commands.remove_resource::<LocalCharacter>();
    commands.remove_resource::<LocalController>();
}

#[derive(Debug, Default)]
enum ConnectionState {
    #[default]
    WaitingToEstablish,
    CharacterSync,
    StartSession,
}

#[allow(clippy::too_many_arguments)]
fn wait_for_players(
    mut commands: Commands,
    mut connection_state: Local<ConnectionState>,
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
    local_character: Res<LocalCharacter>,
    local_controls: Res<LocalController>,
    args: Res<WagArgs>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_match_state: ResMut<NextState<MatchState>>,
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
            let (chars, controllers) = if peer_index == 0 {
                (
                    Characters {
                        p1: contents[0].into(),
                        p2: local_character.0,
                    },
                    Controllers {
                        p1: InputDevice::Online(0),
                        p2: local_controls.0,
                    },
                )
            } else if peer_index == 1 {
                (
                    Characters {
                        p1: local_character.0,
                        p2: contents[0].into(),
                    },
                    Controllers {
                        p1: local_controls.0,
                        p2: InputDevice::Online(1),
                    },
                )
            } else {
                // I'm assuming only valid indices are 0 and 1
                // I think this will break if spectating is introduced
                debug!(peer_index);
                panic!("Peer index is not 0 or 1");
            };

            commands.insert_resource(chars);
            commands.insert_resource(controllers);
            *connection_state = ConnectionState::StartSession;
        }
        ConnectionState::StartSession => {
            let mut session_builder = ggrs::SessionBuilder::<Config>::new()
                .with_num_players(2)
                .with_desync_detection_mode(ggrs::DesyncDetection::On { interval: 1 })
                .with_input_delay(args.input_delay);

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

            next_game_state.set(GameState::Online(OnlineState::Match));
            next_match_state.set(MatchState::Loading);
        }
    };
}

fn in_synctest_postload(
    game_state: Res<State<GameState>>,
    match_state: Res<State<MatchState>>,
) -> bool {
    *game_state.get() == GameState::Synctest && *match_state.get() == MatchState::PostLoad
}

fn start_synctest_session(mut commands: Commands, args: Res<WagArgs>, mut started: Local<bool>) {
    if *started {
        return;
    }

    *started = true;

    info!("Starting synctest session");
    let num_players = 2;

    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(args.input_delay);

    for i in 0..num_players {
        session_builder = session_builder
            .add_player(ggrs::PlayerType::Local, i)
            .expect("failed to add player");
    }

    let ggrs_session = session_builder
        .start_synctest_session()
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::SyncTest(ggrs_session));
}

fn read_local_inputs(
    mut commands: Commands,
    keyboard_keys: Res<ButtonInput<KeyCode>>,
    maybe_controller: Option<Res<LocalController>>,
    local_players: Res<LocalPlayers>,
    pad_query: Query<&Gamepad>,
) {
    let Some(local_controls) = maybe_controller else {
        return;
    };

    let mut inputs = HashMap::new();

    // There is only ever one, but the value can be 1 or 0
    let handle = local_players.0.first().unwrap();
    let mut input = 0u16;

    match local_controls.0 {
        InputDevice::Controller(entity) => {
            let gamepad = pad_query.get(entity).unwrap();
            for (shift, wag_button) in NetworkInputButton::iter().enumerate() {
                if gamepad.pressed(wag_button.to_gamepad_button_type()) {
                    input |= 1 << shift;
                }
            }

            if let (Some(stick_x), Some(stick_y)) = (
                gamepad.get(GamepadAxis::LeftStickX),
                gamepad.get(GamepadAxis::LeftStickY),
            ) {
                let (up, down) = if stick_y.abs() < STICK_DEAD_ZONE {
                    (0, 0)
                } else if stick_y < 0.0 {
                    (0, 1)
                } else {
                    (1, 0)
                };

                let (left, right) = if stick_x.abs() < STICK_DEAD_ZONE {
                    (0, 0)
                } else if stick_x < 0.0 {
                    (1, 0)
                } else {
                    (0, 1)
                };

                // You may end up with opposing cardinals pressed with stick+dpad
                input |= up;
                input |= down << 1;
                input |= left << 2;
                input |= right << 3;
            }
        }
        InputDevice::Keyboard => {
            for (shift, wag_button) in NetworkInputButton::iter().enumerate() {
                if keyboard_keys.pressed(wag_button.to_keycode()) {
                    input |= 1 << shift;
                }
            }
        }
        InputDevice::Online(_) => {
            error!("We should never have online input devices here");
            panic!()
        }
    }

    inputs.insert(*handle, input);
    inputs.insert(1 - handle, 0);

    commands.insert_resource(LocalInputs::<Config>(inputs));
}

fn generate_offline_input_streams(
    mut stream: ResMut<InputStream>,
    keys: Res<ButtonInput<KeyCode>>,
    pad_query: Query<(Entity, &Gamepad)>,
) {
    let mut new_states = HashMap::<InputDevice, InputState>::new();

    for (entity, pad) in &pad_query {
        let mut state = InputState::default();
        let mut stick: IVec2 = default();

        // Buttons
        for nb in NetworkInputButton::iter() {
            if pad.pressed(nb.to_gamepad_button_type()) {
                match nb {
                    // Dpad
                    NetworkInputButton::Up => {
                        stick.y += 1;
                    }
                    NetworkInputButton::Down => {
                        stick.y -= 1;
                    }
                    NetworkInputButton::Left => {
                        stick.x -= 1;
                    }
                    NetworkInputButton::Right => {
                        stick.x += 1;
                    }
                    // Other buttons
                    _ => {
                        // This filters out unused buttons
                        if let Ok(btn) = nb.try_into() {
                            state.pressed.insert(btn);
                        }
                    }
                }
            }
        }

        // Analog stick (Not sure why they are options)
        let analog_x = pad.get(GamepadAxis::LeftStickX).unwrap();
        let analog_y = pad.get(GamepadAxis::LeftStickY).unwrap();

        if analog_x.abs() > STICK_DEAD_ZONE {
            stick.x += analog_x.signum() as i32;
        };

        if analog_y.abs() > STICK_DEAD_ZONE {
            stick.y += analog_y.signum() as i32;
        };

        // Clamps values from -1 to 1 (in cases where dpad and analog stick press the same way)
        stick.x = stick.x.signum();
        stick.y = stick.y.signum();
        state.stick_position = stick.into();

        new_states.insert(InputDevice::Controller(entity), state);
    }

    // Add keyboard
    let mut kb_state = InputState::default();
    let mut kb_stick: IVec2 = default();

    // Buttons
    for nb in NetworkInputButton::iter() {
        if keys.pressed(nb.to_keycode()) {
            match nb {
                // Dpad
                NetworkInputButton::Up => {
                    kb_stick.y += 1;
                }
                NetworkInputButton::Down => {
                    kb_stick.y -= 1;
                }
                NetworkInputButton::Left => {
                    kb_stick.x -= 1;
                }
                NetworkInputButton::Right => {
                    kb_stick.x += 1;
                }
                // Other buttons
                _ => {
                    // This filters out unused buttons
                    if let Ok(btn) = nb.try_into() {
                        kb_state.pressed.insert(btn);
                    }
                }
            }
        }
    }

    kb_state.stick_position = kb_stick.into();

    new_states.insert(InputDevice::Keyboard, kb_state);

    // Compare to previous state
    for (pad, new_state) in new_states.iter() {
        let old_state = stream.input_states.entry(*pad).or_default();
        for event in old_state.changes_to(new_state) {
            // Send events if differences noted
            stream.events.push(OwnedInput {
                event,
                player_handle: *pad,
            });
        }
        // Save new state
        stream.input_states.insert(*pad, new_state.clone());
    }
}

fn clear_input_stream(mut stream: ResMut<InputStream>) {
    // This is scheduled to run at the end of each rollback loop
    stream.events.clear();
}

fn generate_online_input_streams(
    mut writer: ResMut<InputStream>,
    inputs: Res<PlayerInputs<Config>>,
) {
    let mut new_states = HashMap::<InputDevice, InputState>::new();

    for (player_index, (index, _)) in inputs.iter().enumerate() {
        let mut new_state = InputState::default();
        let mut new_stick: IVec2 = default();

        for (shift, nb) in NetworkInputButton::iter().enumerate() {
            let is_pressed = ((index >> shift) & 1) == 1;

            if is_pressed {
                match nb {
                    // Dpad
                    NetworkInputButton::Up => {
                        new_stick.y += 1;
                    }
                    NetworkInputButton::Down => {
                        new_stick.y -= 1;
                    }
                    NetworkInputButton::Left => {
                        new_stick.x -= 1;
                    }
                    NetworkInputButton::Right => {
                        new_stick.x += 1;
                    }
                    // Other buttons
                    _ => {
                        // This filters out unused buttons
                        if let Ok(btn) = nb.try_into() {
                            new_state.pressed.insert(btn);
                        }
                    }
                }
            }
        }

        new_state.stick_position = new_stick.into();

        new_states.insert(InputDevice::Online(player_index), new_state);
    }

    // Compare to previous state
    for (handle, new_state) in new_states.iter() {
        let old_state = writer.input_states.entry(*handle).or_default();
        for event in old_state.changes_to(new_state) {
            // Send events if differences noted
            writer.events.push(OwnedInput {
                event,
                player_handle: *handle,
            });
        }
        // Save new state
        writer.input_states.insert(*handle, new_state.clone());
    }
}

fn handle_ggrs_events(
    mut sesh: ResMut<Session<Config>>,
    mut next_main_state: ResMut<NextState<GameState>>,
    mut next_match_state: ResMut<NextState<MatchState>>,
) {
    let Session::P2P(s) = sesh.as_mut() else {
        return;
    };

    for event in s.events() {
        match event {
            ggrs::GgrsEvent::Disconnected { addr: _ }
            | ggrs::GgrsEvent::DesyncDetected {
                frame: _,
                local_checksum: _,
                remote_checksum: _,
                addr: _,
            } => {
                warn!("GGRS event: {:?}, transitioning to main menu", event);
                next_main_state.set(GameState::MainMenu);
                next_match_state.set(MatchState::None);
                // TODO: Add like a notification for the user or something
            }
            ggrs::GgrsEvent::NetworkInterrupted {
                addr: _,
                disconnect_timeout: _,
            }
            | ggrs::GgrsEvent::NetworkResumed { addr: _ }
            | ggrs::GgrsEvent::WaitRecommendation { skip_frames: _ } => {
                debug!("Unhandled GGRS event: {:?}", event);
            }
            _ => {}
        }
    }
}

fn tf_hasher(transform: &Transform) -> u64 {
    let mut hasher = checksum_hasher();

    debug_assert!(
        transform.is_finite(),
        "Hashing is not stable for NaN f32 values."
    );

    transform.translation.x.to_bits().hash(&mut hasher);
    transform.translation.y.to_bits().hash(&mut hasher);
    transform.translation.z.to_bits().hash(&mut hasher);

    transform.rotation.x.to_bits().hash(&mut hasher);
    transform.rotation.y.to_bits().hash(&mut hasher);
    transform.rotation.z.to_bits().hash(&mut hasher);
    transform.rotation.w.to_bits().hash(&mut hasher);

    transform.scale.x.to_bits().hash(&mut hasher);
    transform.scale.y.to_bits().hash(&mut hasher);
    transform.scale.z.to_bits().hash(&mut hasher);

    hasher.finish()
}

fn clock_hasher(clock: &Clock) -> u64 {
    let mut hasher = checksum_hasher();

    clock.frame.hash(&mut hasher);

    hasher.finish()
}
