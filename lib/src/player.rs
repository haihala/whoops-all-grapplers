use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::character::{movement_executor, register_ryan_moves, ryan_executor, Ryan};
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
    pub drag_multiplier: f32,
    pub flipped: bool,
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            grounded: true,
            drag_multiplier: 1.0,
            flipped: false,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(ryan_executor.system())
            .add_system(movement_executor.system());
    }
}

fn setup(mut commands: Commands, assets: Res<Colors>) {
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
        .insert(PlayerState::default())
        .insert(register_ryan_moves(input_parsing::InputReader::default()))
        .insert(Ryan);
}
