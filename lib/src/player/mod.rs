mod charge_accumulator;
mod model_flipper;
mod move_activation;
mod move_advancement;
mod movement;
mod recovery;
mod size_adjustment;

#[cfg(not(test))]
use input_parsing::PadBundle;
use kits::{ryan_kit, Grabable, Hurtbox, Inventory, Kit, Resources};
use player_state::PlayerState;
use time::{Clock, GameState, RoundResult};
use types::{LRDirection, Player, Players};

use crate::{
    assets::{Colors, ModelRequest, Models},
    damage::Health,
    physics::{PlayerVelocity, GROUND_PLANE_HEIGHT},
    spawner::Spawner,
};

use bevy::{prelude::*, sprite::Anchor};

use self::{model_flipper::PlayerModel, move_activation::MoveBuffer};

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)
const PLAYER_SPAWN_HEIGHT: f32 = GROUND_PLANE_HEIGHT + 0.001;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(reset.with_run_criteria(State::on_update(GameState::Shop)))
            .add_system_set(
                SystemSet::on_update(GameState::Combat)
                    .with_system(move_advancement::move_advancement.after(reset))
                    .with_system(
                        move_activation::move_activator.after(move_advancement::move_advancement),
                    )
                    .with_system(recovery::stun_recovery.after(move_activation::move_activator))
                    .with_system(recovery::ground_recovery.after(recovery::stun_recovery))
                    .with_system(movement::movement.after(recovery::ground_recovery))
                    .with_system(size_adjustment::size_adjustment.after(movement::movement))
                    .with_system(
                        charge_accumulator::manage_charge.after(size_adjustment::size_adjustment),
                    )
                    .with_system(
                        model_flipper::model_flipper.after(charge_accumulator::manage_charge),
                    )
                    .with_system(testing.after(model_flipper::model_flipper)),
            );
    }
}

fn setup(mut commands: Commands, colors: Res<Colors>, models: Res<Models>) {
    let players = Players {
        one: spawn_player(
            &mut commands,
            &colors,
            &models,
            -PLAYER_SPAWN_DISTANCE,
            Player::One,
        ),
        two: spawn_player(
            &mut commands,
            &colors,
            &models,
            PLAYER_SPAWN_DISTANCE,
            Player::Two,
        ),
    };

    commands.insert_resource(players);
}

#[derive(Bundle, Default)]
struct PlayerDefaults {
    health: Health,
    resources: Resources,
    inventory: Inventory,
    spawner: Spawner,
    hurtbox: Hurtbox,
    grab_target: Grabable,
    player_velocity: PlayerVelocity,
    move_buffer: MoveBuffer,
}

fn spawn_player(
    commands: &mut Commands,
    colors: &Res<Colors>,
    models: &Res<Models>,
    offset: f32,
    player: Player,
) -> Entity {
    let state = PlayerState::default();
    let kit = ryan_kit();

    #[cfg(not(test))]
    let inputs = PadBundle::new(kit.get_inputs());

    let mut spawn_handle = commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: (
                offset,
                PLAYER_SPAWN_HEIGHT + state.get_collider_size().y / 2.0,
                0.0,
            )
                .into(),
            ..default()
        },
        sprite: Sprite {
            color: colors.collision_box,
            custom_size: Some(state.get_collider_size()),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        ..default()
    });

    spawn_handle
        .insert_bundle(PlayerDefaults::default())
        .insert(LRDirection::from_flipped(offset.is_sign_positive()))
        .insert(kit)
        .insert(player)
        .insert(state);

    #[cfg(not(test))]
    spawn_handle.insert_bundle(inputs);

    spawn_handle.with_children(|parent| {
        parent
            .spawn_bundle(TransformBundle::default())
            .insert(ModelRequest {
                model: models.ryan.clone(),
                animation: Some(("Idle", true)),
            })
            .insert(PlayerModel(player));
    });

    spawn_handle.id()
}

fn reset(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Health,
        &mut Resources,
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

        for (mut health, mut resources, mut tf, player, mut player_state, mut buffer) in
            query.iter_mut()
        {
            health.reset();
            resources.reset();
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

#[allow(clippy::too_many_arguments)]
fn testing(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Inventory, &Kit)>) {
    // B for Buy
    if keys.just_pressed(KeyCode::B) {
        for (mut inventory, kit) in query.iter_mut() {
            if let Some((id, _)) = kit.roll_items(1, &inventory).first() {
                inventory.add_item(*id);
            }
        }
    }
}
