mod charge_accumulator;
mod move_activation;
mod move_advancement;
mod move_state_manager;
mod movement;
mod recovery;
mod size_adjustment;

use input_parsing::InputParser;
#[cfg(not(test))]
use input_parsing::PadBundle;
use items::{ryan_inventory, Inventory};
use moves::{ryan_bank, MoveBank};
use player_state::PlayerState;
use time::{Clock, GameState, RoundResult};
use types::{Grabable, Hurtbox, LRDirection, Player};

use crate::{
    assets::Colors,
    damage::Health,
    physics::{PlayerVelocity, GROUND_PLANE_HEIGHT},
    resources::{Charge, GameResource, Meter},
    spawner::Spawner,
};

use bevy::prelude::*;

use self::move_activation::MoveBuffer;

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)
const PLAYER_SPAWN_HEIGHT: f32 = GROUND_PLANE_HEIGHT + 0.001;

#[derive(Debug, SystemLabel, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PlayerSystemLabel {
    Reset,
    SetFlags,
    MoveAdvancer,
    MoveActivator,
    StunRecovery,
    GroundRecovery,
    Movement,
    SizeAdjustment,
    Charge,
    Testing,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(
                reset
                    .with_run_criteria(State::on_update(GameState::Shop))
                    .label(PlayerSystemLabel::Reset),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Combat)
                    .with_system(
                        move_state_manager::set_flags
                            .label(PlayerSystemLabel::SetFlags)
                            .after(PlayerSystemLabel::Reset),
                    )
                    .with_system(
                        move_advancement::move_advancement
                            .label(PlayerSystemLabel::MoveAdvancer)
                            .after(PlayerSystemLabel::SetFlags),
                    )
                    .with_system(
                        move_activation::move_activator
                            .label(PlayerSystemLabel::MoveActivator)
                            .after(PlayerSystemLabel::MoveAdvancer),
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
                    )
                    .with_system(
                        charge_accumulator::manage_charge
                            .label(PlayerSystemLabel::Charge)
                            .after(PlayerSystemLabel::SizeAdjustment),
                    )
                    .with_system(
                        testing
                            .label(PlayerSystemLabel::Testing)
                            .after(PlayerSystemLabel::Charge),
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
    move_buffer: MoveBuffer,
}

fn spawn_player(commands: &mut Commands, colors: &Res<Colors>, offset: f32, player: Player) {
    let state = PlayerState::default();
    let bank = ryan_bank();

    #[cfg(not(test))]
    let inputs = PadBundle::new(bank.get_inputs());

    let mut spawn_handle = commands.spawn_bundle(SpriteBundle {
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
    });

    spawn_handle
        .insert_bundle(PlayerDefaults::default())
        .insert(LRDirection::from_flipped(offset.is_sign_positive()))
        .insert(bank)
        .insert(player)
        .insert(state)
        .insert(ryan_inventory())
        .insert(Charge::new(0.75));

    #[cfg(not(test))]
    spawn_handle.insert_bundle(inputs);
}

fn reset(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Health,
        &mut Meter,
        &mut Charge,
        &mut Transform,
        &Player,
        &mut PlayerState,
        &mut MoveBuffer,
    )>,
    mut game_state: ResMut<State<GameState>>,
    mut clock: ResMut<Clock>,
) {
    // Just pressed would be better, but it's difficult in tests and the difference is very minor.
    if keys.pressed(KeyCode::Return) {
        game_state.set(GameState::Combat).unwrap();
        clock.reset();
        commands.remove_resource::<RoundResult>();

        for (mut health, mut meter, mut charge, mut tf, player, mut player_state, mut buffer) in
            query.iter_mut()
        {
            health.reset();
            meter.reset();
            charge.reset();
            player_state.reset();
            buffer.clear();

            tf.translation.x = match *player {
                Player::One => -PLAYER_SPAWN_DISTANCE,
                Player::Two => PLAYER_SPAWN_DISTANCE,
            };
            tf.translation.y = PLAYER_SPAWN_HEIGHT + player_state.get_collider_size().y / 2.0;
        }
    }
}

fn testing(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Inventory, &mut MoveBank, &mut InputParser)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (mut inventory, mut bank, mut parser) in query.iter_mut() {
            if let Some(shopitem) = inventory.roll_shop(1).first() {
                let item = shopitem.item.clone();
                inventory.buy(item.clone());

                for (move_id, move_data) in item.new_moves {
                    if let Some(input) = move_data.input {
                        parser.register_input(move_id, input.into());
                    }
                    bank.register_move(move_id, move_data);
                }
            }
        }
    }
}
