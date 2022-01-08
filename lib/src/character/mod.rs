mod movement;
mod ryan;

use input_parsing::{InputParser, InputReader};
use movement::movement;
use moves::{ryan_bank, CancelLevel, Move, MoveBank};
use player_state::PlayerState;
use types::{Hurtbox, MoveId, Player};

use crate::{
    assets::Colors,
    damage::Health,
    game_flow::GameState,
    meter::Meter,
    physics::{PlayerVelocity, GROUND_PLANE_HEIGHT},
    spawner::Spawner,
};

use bevy::prelude::*;

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)
const PLAYER_SPAWN_HEIGHT: f32 = GROUND_PLANE_HEIGHT + 0.001;

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
    spawn_player(&mut commands, &colors, -PLAYER_SPAWN_DISTANCE, Player::One);
    spawn_player(&mut commands, &colors, PLAYER_SPAWN_DISTANCE, Player::Two);
}

fn spawn_player(commands: &mut Commands, colors: &Res<Colors>, offset: f32, player: Player) {
    let state = PlayerState::default();
    let bank = ryan_bank();

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: (
                    offset,
                    PLAYER_SPAWN_HEIGHT + state.get_collider_size().y / 2.0,
                    0.0,
                )
                    .into(),
                ..Default::default()
            },
            material: colors.collision_box.clone(),
            sprite: Sprite {
                size: state.get_collider_size(),
                resize_mode: SpriteResizeMode::Automatic,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(player)
        .insert(Health::default())
        .insert(Meter::default())
        .insert(InputReader::default())
        .insert(InputParser::load(bank.get_inputs()))
        .insert(Spawner::load(bank.get_hitboxes(), player))
        .insert(bank)
        .insert(PlayerVelocity::default())
        .insert(Hurtbox::new(state.get_collider_size()))
        .insert(state)
        .insert(ryan::Ryan);
}

fn reset(
    mut query: Query<(
        &mut Health,
        &mut Meter,
        &mut Transform,
        &Player,
        &PlayerState,
    )>,
) {
    for (mut health, mut meter, mut tf, player, state) in query.iter_mut() {
        health.reset();
        meter.reset();

        tf.translation.x = match *player {
            Player::One => -PLAYER_SPAWN_DISTANCE,
            Player::Two => PLAYER_SPAWN_DISTANCE,
        };
        tf.translation.y = PLAYER_SPAWN_HEIGHT + state.get_collider_size().y / 2.0;
    }
}

fn move_to_activate(
    options: Vec<MoveId>,
    bank: &MoveBank,
    cancel_requirement: CancelLevel,
) -> Option<(MoveId, Move)> {
    options
        .into_iter()
        .map(|id| (id, bank.get(id).to_owned()))
        .filter(|(_, action)| action.cancel_level >= cancel_requirement)
        .min_by(|(id1, _), (id2, _)| id1.cmp(id2))
}
