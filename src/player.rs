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

// Tag
pub struct Player;

// Tracking for the players' state
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

fn setup(mut commands: Commands, assets: Res<Materials>) {
    let button_mappings: HashMap<GamepadButtonType, input::ActionButton> = [
        (GamepadButtonType::South, input::ActionButton::Fast),
        (GamepadButtonType::West, input::ActionButton::Heavy),
    ]
    .iter()
    .cloned()
    .collect();

    commands.insert_resource(button_mappings);
    commands.insert_resource(crate::input::special_moves::get_special_move_name_mappings());

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
        .insert(crate::physics::PhysicsObject::default())
        .insert(input::InputStore::default())
        .insert(PlayerState::default())
        .insert(crate::character::RyanMoveBuffer(None, None))
        .insert(crate::character::Ryan);
}
