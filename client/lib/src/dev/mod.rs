use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use characters::{Hitbox, Hurtbox, Inventory, Resources};
use player_state::PlayerState;
use wag_core::{Clock, GameState, Player, SoundEffect, Stats};

use crate::{
    assets::Sounds,
    damage::Health,
    physics::{ConstantVelocity, PlayerVelocity, Pushbox},
    player::MoveBuffer,
};

mod box_visualization;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin)
            .register_type::<Player>()
            .register_type::<Resources>()
            .register_type::<Health>()
            .register_type::<PlayerState>()
            .register_type::<Clock>()
            .register_type::<PlayerVelocity>()
            .register_type::<ConstantVelocity>()
            .register_type::<Pushbox>()
            .register_type::<Hurtbox>()
            .register_type::<Hitbox>()
            .register_type::<MoveBuffer>()
            .register_type::<Inventory>()
            .register_type::<Stats>()
            .add_system(generic_test_system)
            .add_system(cycle_game_state.after(generic_test_system))
            .add_system(input_leniency_test_system.after(cycle_game_state))
            .add_system(box_visualization::spawn_boxes.after(input_leniency_test_system))
            .add_system(box_visualization::size_adjustment.after(box_visualization::spawn_boxes));
    }
}

fn generic_test_system(keys: Res<Input<KeyCode>>, mut sounds: ResMut<Sounds>) {
    if keys.just_pressed(KeyCode::S) {
        dbg!("Playing");
        sounds.play(SoundEffect::Whoosh)
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
    mut game_state: ResMut<State<GameState>>,
    mut clock: ResMut<Clock>,
) {
    // Can be converted to a non-dev system eventually (to start game press start type of deal)
    if keys.just_pressed(KeyCode::Return) {
        if *game_state.current() == GameState::Combat {
            // Set clock to zero to go through the same route as time out
            clock.time_out();
        } else {
            let next_state = game_state.current().next();
            game_state.set(next_state).unwrap();
        }
    }
}
