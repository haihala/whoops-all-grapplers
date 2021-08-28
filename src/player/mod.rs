use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::labels::{InputSystemLabel, SystemSetLabel};
use crate::Materials;

mod character;
mod inputparsing;

use inputparsing::{ActionButton, InputBuffer};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system_set(
                SystemSet::new()
                    .label(SystemSetLabel::Input)
                    .with_system(inputparsing::detect_new_pads.system())
                    .with_system(
                        inputparsing::cull_stick_input_buffer
                            .system()
                            .label(InputSystemLabel::Clear),
                    )
                    .with_system(
                        inputparsing::collect_input
                            .system()
                            .label(InputSystemLabel::Collect)
                            .after(InputSystemLabel::Clear),
                    )
                    .with_system(
                        inputparsing::interpret_stick_inputs
                            .system()
                            .label(InputSystemLabel::Parse)
                            .after(InputSystemLabel::Collect),
                    ),
            )
            .add_system_set(
                SystemSet::new()
                    .label(SystemSetLabel::Characters)
                    .after(SystemSetLabel::Input)
                    .with_system(character::ryan.system()),
            );
    }
}

pub struct Player;

fn setup(mut commands: Commands, assets: Res<Materials>) {
    let button_mappings: HashMap<GamepadButtonType, ActionButton> = [
        (GamepadButtonType::South, ActionButton::Fast),
        (GamepadButtonType::West, ActionButton::Vicious),
    ]
    .iter()
    .cloned()
    .collect();

    commands.insert_resource(button_mappings);

    commands
        .spawn_bundle(SpriteBundle {
            material: assets.collision_box_color.clone(),
            sprite: Sprite::new(Vec2::new(
                crate::constants::PLAYER_SPRITE_WIDTH,
                crate::constants::PLAYER_SPRITE_HEIGHT,
            )),
            ..Default::default()
        })
        .insert(Player)
        .insert(character::Ryan)
        .insert(InputBuffer {
            frames: VecDeque::new(),
            interpreted: Vec::new(),
        });
}
