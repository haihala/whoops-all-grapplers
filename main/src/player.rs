use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use std::collections::HashMap;

use crate::input;
use crate::labels::{InputSystemLabel, SystemSetLabel};
use crate::Colors;

// Tag
#[derive(Inspectable, Default)]
pub struct Player(pub i32);

#[derive(Inspectable, Default)]
pub struct Health {
    // For rendering purposes, max health=1 and store only the ratio.
    // Different characters ought to have a scalar scale for incoming damage
    // This won't be communicated to the player.
    pub ratio: f32,
}

#[derive(Inspectable, Default)]
pub struct Meter {
    // See Health comment
    pub ratio: f32,
}

pub struct PlayerState {
    pub grounded: bool,
    pub decelerating: bool,
    pub flipped: bool,
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            grounded: true,
            decelerating: true,
            flipped: false,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system_set(
                SystemSet::new()
                    .label(SystemSetLabel::Input)
                    .with_system(input::detect_new_pads.system())
                    .with_system(
                        input::cull_diff_buffer
                            .system()
                            .label(InputSystemLabel::Clear),
                    )
                    .with_system(
                        input::collect_input
                            .system()
                            .label(InputSystemLabel::Collect)
                            .after(InputSystemLabel::Clear),
                    ),
            )
            .add_system_set(
                SystemSet::new()
                    .label(SystemSetLabel::Characters)
                    .after(SystemSetLabel::Input)
                    .with_system(
                        crate::character::ryan_parser
                            .system()
                            .label(InputSystemLabel::Parse),
                    )
                    .with_system(
                        crate::character::ryan_executor
                            .system()
                            .label(InputSystemLabel::Execute)
                            .after(InputSystemLabel::Parse),
                    ),
            );
    }
}

fn setup(mut commands: Commands, assets: Res<Colors>) {
    let button_mappings: HashMap<GamepadButtonType, input::ActionButton> = [
        (GamepadButtonType::South, input::ActionButton::Fast),
        (GamepadButtonType::West, input::ActionButton::Heavy),
    ]
    .iter()
    .cloned()
    .collect();

    commands.insert_resource(button_mappings);
    commands.insert_resource(crate::input::special_moves::get_special_move_name_mappings());

    spawn_player(&mut commands, &assets, 2.0, 1);
    spawn_player(&mut commands, &assets, -2.0, 2);
}

fn spawn_player(commands: &mut Commands, assets: &Res<Colors>, offset: f32, player_number: i32) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: (offset, 0.0, 0.0).into(),
                ..Default::default()
            },
            material: assets.collision_box.clone(),
            sprite: Sprite::new(Vec2::new(
                crate::PLAYER_SPRITE_WIDTH,
                crate::PLAYER_SPRITE_HEIGHT,
            )),
            ..Default::default()
        })
        .insert(Player(player_number))
        .insert(Health { ratio: 1.0 })
        .insert(Meter { ratio: 1.0 })
        .insert(crate::physics::PhysicsObject::default())
        .insert(input::InputStore::default())
        .insert(PlayerState::default())
        .insert(crate::character::RyanMoveBuffer(None, None))
        .insert(crate::character::Ryan);
}
