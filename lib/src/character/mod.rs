mod movement;
mod ryan;

use input_parsing::InputReader;
use movement::movement;
use moves::{ryan_frames, ryan_hitboxes, ryan_normals, ryan_specials};
use types::Player;

use crate::{
    damage::{HitboxManager, Hurtbox},
    frame_data_manager::FrameDataManager,
    game_flow::GameState,
    physics::PhysicsObject,
};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::fmt::Debug;

use crate::{Colors, Health, Meter};

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
            .add_system_set(
                SystemSet::on_update(GameState::Combat)
                    .with_system(ryan::move_starter.system())
                    .with_system(movement.system()),
            )
            .add_system_set(SystemSet::on_enter(GameState::Combat).with_system(reset.system()));
    }
}

fn setup(mut commands: Commands, colors: Res<Colors>) {
    spawn_player(
        &mut commands,
        &colors,
        -crate::PLAYER_SPAWN_DISTANCE,
        Player::One,
    );
    spawn_player(
        &mut commands,
        &colors,
        crate::PLAYER_SPAWN_DISTANCE,
        Player::Two,
    );
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
        .insert(InputReader::load(ryan_specials(), ryan_normals()))
        .insert(FrameDataManager::load(ryan_frames()))
        .insert(HitboxManager::load(ryan_hitboxes()))
        .insert(Hurtbox::new(Vec2::new(
            crate::PLAYER_SPRITE_WIDTH,
            crate::PLAYER_SPRITE_HEIGHT,
        )))
        .insert(ryan::Ryan);
}

fn reset(mut query: Query<(&mut Health, &mut Transform, &Player)>) {
    for (mut health, mut tf, player) in query.iter_mut() {
        health.ratio = 1.0;
        tf.translation.x = match *player {
            Player::One => -crate::PLAYER_SPAWN_DISTANCE,
            Player::Two => crate::PLAYER_SPAWN_DISTANCE,
        };
    }
}
