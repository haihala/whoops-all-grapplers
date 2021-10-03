mod movement;
mod ryan;

use movement::movement;

use crate::{damage::Hurtbox, physics::PhysicsObject};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{Colors, Health, Meter};

#[derive(Inspectable, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    One,
    Two,
}

#[derive(Inspectable, PartialEq, Eq, Clone, Copy, Debug)]
pub enum PlayerState {
    Startup,
    Active,
    Recovery,
    Standing,
    Air,
}
impl PlayerState {
    pub fn land(&mut self) {
        // Depending on current state, could either:
        // - Air -> Standing
        // - Freefall -> Grounded
        *self = PlayerState::Standing;
    }
    pub fn recover(&mut self) {
        *self = PlayerState::Standing;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(ryan::move_starter.system())
            .add_system(movement.system());
    }
}

fn setup(mut commands: Commands, colors: Res<Colors>) {
    spawn_player(&mut commands, &colors, -2.0, Player::One);
    spawn_player(&mut commands, &colors, 2.0, Player::Two);
}

fn spawn_player(commands: &mut Commands, colors: &Res<Colors>, offset: f32, player: Player) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: (offset, 0.0, 0.0).into(),
                ..Default::default()
            },
            material: colors.collision_box.clone(),
            sprite: Sprite::new(Vec2::new(
                crate::PLAYER_SPRITE_WIDTH,
                crate::PLAYER_SPRITE_HEIGHT,
            )),
            ..Default::default()
        })
        .insert(player)
        .insert(Health::default())
        .insert(Meter::default())
        .insert(PhysicsObject::default())
        .insert(PlayerState::Standing)
        .insert(ryan::inputs())
        .insert(ryan::animations())
        .insert(ryan::hitboxes())
        .insert(Hurtbox::new(Vec2::new(
            crate::PLAYER_SPRITE_WIDTH,
            crate::PLAYER_SPRITE_HEIGHT,
        )))
        .insert(ryan::Ryan);
}
