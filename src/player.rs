use bevy::prelude::*;
use std::collections::HashMap;

use crate::input;
use crate::labels::{InputSystemLabel, SystemSetLabel};
use crate::Materials;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system_set(
                SystemSet::new()
                    .label(SystemSetLabel::Input)
                    .with_system(input::detect_new_pads.system())
                    .with_system(
                        input::cull_stick_input_buffer
                            .system()
                            .label(InputSystemLabel::Clear),
                    )
                    .with_system(
                        input::collect_input
                            .system()
                            .label(InputSystemLabel::Collect)
                            .after(InputSystemLabel::Clear),
                    )
                    .with_system(
                        input::interpret_stick_inputs
                            .system()
                            .label(InputSystemLabel::Parse)
                            .after(InputSystemLabel::Collect),
                    ),
            )
            .add_system_set(
                SystemSet::new()
                    .label(SystemSetLabel::Characters)
                    .after(SystemSetLabel::Input)
                    .with_system(crate::character::ryan.system()),
            );
    }
}

pub struct Player;

fn setup(mut commands: Commands, assets: Res<Materials>) {
    let button_mappings: HashMap<GamepadButtonType, input::ActionButton> = [
        (GamepadButtonType::South, input::ActionButton::Fast),
        (GamepadButtonType::West, input::ActionButton::Vicious),
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
        .insert(crate::character::Ryan)
        .insert(input::InputBuffer::default());
}
