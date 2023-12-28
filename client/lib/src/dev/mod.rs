use std::vec;

use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use characters::{ActionEvent, FlashRequest, Hitbox, Hurtbox, Inventory};
use player_state::PlayerState;
use wag_core::{
    Clock, GameState, Joints, Player, SoundEffect, Stats, WAGStage, GI_PARRY_FLASH_COLOR,
};

use crate::{
    assets::Sounds,
    physics::{ConstantVelocity, PlayerVelocity, Pushbox},
    player::MoveBuffer,
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
            .register_type::<ConstantVelocity>()
            .register_type::<Pushbox>()
            .register_type::<Hurtbox>()
            .register_type::<Hitbox>()
            .register_type::<MoveBuffer>()
            .register_type::<Inventory>()
            .register_type::<Joints>()
            .register_type::<Stats>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    audio_test_system,
                    shader_test_system,
                    fullscreen_toggle,
                    pause_toggle,
                    cycle_game_state,
                    input_leniency_test_system,
                    box_visualization::visualize_hitboxes,
                    box_visualization::visualize_hurtboxes,
                    box_visualization::visualize_pushboxes,
                )
                    .chain()
                    .after(WAGStage::PlayerUpdates),
            );
    }
}

// TODO: There is probably a better way to do this
fn setup(mut config: ResMut<GizmoConfig>) {
    config.depth_bias = -1.0;
}

fn shader_test_system(keys: Res<Input<KeyCode>>, mut players: Query<&mut PlayerState>) {
    if keys.just_pressed(KeyCode::S) {
        println!("Playing shader flash");
        for mut player in &mut players {
            player.add_actions(vec![ActionEvent::Flash(FlashRequest {
                color: GI_PARRY_FLASH_COLOR,
                speed: 0.0,
                ..default()
            })])
        }
    }
}

fn audio_test_system(keys: Res<Input<KeyCode>>, mut sounds: ResMut<Sounds>) {
    if keys.just_pressed(KeyCode::A) {
        println!("Playing whoosh audio");
        sounds.play(SoundEffect::Whoosh);
    }
}

fn fullscreen_toggle(keys: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if keys.just_pressed(KeyCode::F) {
        let mut win = windows.get_single_mut().unwrap();
        println!("Fullscreen toggle");

        win.mode = match win.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            WindowMode::BorderlessFullscreen => WindowMode::Windowed,
            _ => win.mode,
        }
    }
}

fn pause_toggle(keys: Res<Input<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if keys.just_pressed(KeyCode::P) {
        println!("Pause toggle");
        let new_speed = 1.0 - time.relative_speed();
        time.set_relative_speed(new_speed);
    }
}

fn input_leniency_test_system(
    keys: Res<Input<KeyCode>>,
    pad_buttons: Res<Input<GamepadButton>>,
    clock: Res<Clock>,
    mut h_pressed: Local<Option<usize>>,
    mut j_pressed: Local<Option<usize>>,
    mut south_pressed: Local<Option<usize>>,
    mut east_pressed: Local<Option<usize>>,
) {
    if keys.just_pressed(KeyCode::H) {
        *h_pressed = Some(clock.frame);
    }
    if keys.just_pressed(KeyCode::J) {
        *j_pressed = Some(clock.frame);
    }
    log_diff(&mut h_pressed, "H", &mut j_pressed, "J");

    if pad_buttons.just_pressed(GamepadButton {
        gamepad: Gamepad { id: 0 },
        button_type: GamepadButtonType::South,
    }) {
        *south_pressed = Some(clock.frame);
    }
    if pad_buttons.just_pressed(GamepadButton {
        gamepad: Gamepad { id: 0 },
        button_type: GamepadButtonType::East,
    }) {
        *east_pressed = Some(clock.frame);
    }
    log_diff(&mut south_pressed, "A", &mut east_pressed, "B");
}

fn log_diff(
    a_status: &mut Option<usize>,
    a_name: &'static str,
    b_status: &mut Option<usize>,
    b_name: &'static str,
) {
    if let (Some(a_frame), Some(b_frame)) = (*a_status, *b_status) {
        match a_frame.cmp(&b_frame) {
            std::cmp::Ordering::Equal => {
                println!("{a_name} and {b_name} pressed on same frame ({a_frame})",)
            }
            std::cmp::Ordering::Less => {
                println!(
                    "{a_name} was pressed {} frames before {b_name}",
                    b_frame - a_frame
                )
            }
            std::cmp::Ordering::Greater => {
                println!(
                    "{a_name} was pressed {} frames after {b_name}",
                    a_frame - b_frame
                )
            }
        }

        *a_status = None;
        *b_status = None;
    }
}

fn cycle_game_state(
    keys: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut clock: ResMut<Clock>,
) {
    // Can be converted to a non-dev system eventually (to start game press start type of deal)
    if keys.just_pressed(KeyCode::Return) {
        if game_state.get() == &GameState::Combat {
            // Set clock to zero to go through the same route as time out
            clock.time_out();
        } else {
            next_state.set(game_state.get().next());
        }
    }
}
