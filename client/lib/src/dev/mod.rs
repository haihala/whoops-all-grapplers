use bevy::prelude::*;
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorPlugin};

use characters::{Character, Hitbox, Hurtbox, Inventory, Resources};
use player_state::PlayerState;
use time::Clock;
use wag_core::{Player, SoundEffect};

use crate::{
    assets::Sounds,
    damage::Health,
    physics::{ConstantVelocity, PlayerVelocity, Pushbox},
};

mod box_visualization;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        let mut registry = app
            .add_plugin(WorldInspectorPlugin::new())
            .insert_resource(InspectableRegistry::default())
            .add_system(generic_test_system)
            .add_system(input_leniency_test_system.after(generic_test_system))
            .add_system(box_visualization::spawn_boxes.after(input_leniency_test_system))
            .add_system(box_visualization::size_adjustment.after(box_visualization::spawn_boxes))
            .world
            .resource_mut::<InspectableRegistry>();

        registry.register::<Player>();
        registry.register::<Resources>();
        registry.register::<Health>();
        registry.register::<PlayerState>();
        registry.register::<Clock>();
        registry.register::<PlayerVelocity>();
        registry.register::<ConstantVelocity>();
        registry.register::<Pushbox>();
        registry.register::<Hurtbox>();
        registry.register::<Hitbox>();
    }
}

fn generic_test_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Inventory, &Character)>,
    mut sounds: ResMut<Sounds>,
) {
    // B for Buy
    if keys.just_pressed(KeyCode::B) {
        for (mut inventory, character) in &mut query {
            if let Some((id, _)) = character.roll_items(1, &inventory).first() {
                inventory.add_item(*id);
            }
        }
    } else if keys.just_pressed(KeyCode::S) {
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
