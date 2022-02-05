mod move_activation;
mod move_advancement;
mod movement;
mod recovery;
mod size_adjustment;

use input_parsing::PadBundle;
use moves::ryan_bank;
use player_state::PlayerState;
use time::GameState;
use types::{Grabable, Hurtbox, LRDirection, Player};

use crate::{
    assets::Colors,
    damage::Health,
    meter::Meter,
    physics::{PlayerVelocity, GROUND_PLANE_HEIGHT},
    spawner::Spawner,
};

use bevy::prelude::*;

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)
const PLAYER_SPAWN_HEIGHT: f32 = GROUND_PLANE_HEIGHT + 0.001;

#[derive(Debug, SystemLabel, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PlayerSystemLabel {
    Reset,
    MoveActivator,
    MoveAdvancer,
    StunRecovery,
    GroundRecovery,
    Movement,
    SizeAdjustment,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(
                reset
                    .with_run_criteria(State::on_enter(GameState::Combat))
                    .label(PlayerSystemLabel::Reset),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Combat)
                    .with_system(
                        move_activation::move_activator
                            .label(PlayerSystemLabel::MoveActivator)
                            .after(PlayerSystemLabel::Reset),
                    )
                    .with_system(
                        move_advancement::move_advancement
                            .label(PlayerSystemLabel::MoveAdvancer)
                            .after(PlayerSystemLabel::MoveActivator),
                    )
                    .with_system(
                        recovery::stun_recovery
                            .label(PlayerSystemLabel::StunRecovery)
                            .after(PlayerSystemLabel::MoveAdvancer),
                    )
                    .with_system(
                        recovery::ground_recovery
                            .label(PlayerSystemLabel::GroundRecovery)
                            .after(PlayerSystemLabel::StunRecovery),
                    )
                    .with_system(
                        movement::movement
                            .label(PlayerSystemLabel::Movement)
                            .after(PlayerSystemLabel::GroundRecovery),
                    )
                    .with_system(
                        size_adjustment::size_adjustment
                            .label(PlayerSystemLabel::SizeAdjustment)
                            .after(PlayerSystemLabel::Movement),
                    ),
            );
    }
}

fn setup(mut commands: Commands, colors: Res<Colors>) {
    spawn_player(&mut commands, &colors, -PLAYER_SPAWN_DISTANCE, Player::One);
    spawn_player(&mut commands, &colors, PLAYER_SPAWN_DISTANCE, Player::Two);
}

#[derive(Bundle, Default)]
struct PlayerDefaults {
    health: Health,
    meter: Meter,
    spawner: Spawner,
    hurtbox: Hurtbox,
    grab_target: Grabable,
    player_velocity: PlayerVelocity,
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
            sprite: Sprite {
                color: colors.collision_box,
                custom_size: Some(state.get_collider_size()),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(PlayerDefaults::default())
        .insert_bundle(PadBundle::new(bank.get_inputs()))
        .insert(LRDirection::from_flipped(offset.is_sign_positive()))
        .insert(bank)
        .insert(player)
        .insert(state);
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