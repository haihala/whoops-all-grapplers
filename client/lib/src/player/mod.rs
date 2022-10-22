mod asset_updater;
mod charge_accumulator;
mod condition_management;
mod move_activation;
mod move_advancement;
mod movement;
mod recovery;
mod size_adjustment;

use characters::{dummy, Character, Hurtbox, Inventory, Resources};
use core::{Facing, Player, Players};
use input_parsing::{InputParser, PadBundle};
use player_state::PlayerState;
use time::{once_per_combat_frame, Clock, GameState, RoundResult};

use crate::{
    assets::{AnimationHelperSetup, Models},
    damage::{Defense, Health, HitboxSpawner},
    physics::{PlayerVelocity, Pushbox, GROUND_PLANE_HEIGHT},
};

use bevy::{ecs::query::WorldQuery, prelude::*};

use self::move_activation::MoveBuffer;

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)
const PLAYER_SPAWN_HEIGHT: f32 = GROUND_PLANE_HEIGHT + 0.001;

#[derive(WorldQuery)]
#[world_query(mutable)]
struct PlayerQuery<'a> {
    state: &'a mut PlayerState,
    spawner: &'a mut HitboxSpawner,
    character: &'a Character,
    tf: &'a Transform,
    buffer: &'a mut MoveBuffer,
    resources: &'a mut Resources,
    inventory: &'a mut Inventory,
    input_parser: &'a mut InputParser,
    player: &'a Player,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(reset.with_run_criteria(State::on_update(GameState::Shop)))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(once_per_combat_frame)
                    .with_system(move_activation::manage_buffer.after(reset))
                    .with_system(
                        move_activation::move_continuation.after(move_activation::manage_buffer),
                    )
                    .with_system(
                        move_activation::raw_or_link.after(move_activation::move_continuation),
                    )
                    .with_system(
                        move_activation::special_cancel.after(move_activation::raw_or_link),
                    )
                    .with_system(
                        move_activation::move_activator.after(move_activation::special_cancel),
                    )
                    .with_system(
                        move_advancement::move_advancement.after(move_activation::move_activator),
                    )
                    .with_system(recovery::stun_recovery.after(move_advancement::move_advancement))
                    .with_system(recovery::ground_recovery.after(recovery::stun_recovery))
                    .with_system(movement::movement.after(recovery::ground_recovery))
                    .with_system(size_adjustment::size_adjustment.after(movement::movement))
                    .with_system(
                        charge_accumulator::manage_charge.after(size_adjustment::size_adjustment),
                    )
                    .with_system(
                        condition_management::manage_conditions
                            .after(charge_accumulator::manage_charge),
                    )
                    .with_system(
                        asset_updater::update_animation
                            .after(condition_management::manage_conditions),
                    )
                    .with_system(
                        asset_updater::update_audio.after(asset_updater::update_animation),
                    ),
            );
    }
}

fn setup(mut commands: Commands, models: Res<Models>) {
    let players = Players {
        one: spawn_player(&mut commands, &models, -PLAYER_SPAWN_DISTANCE, Player::One),
        two: spawn_player(&mut commands, &models, PLAYER_SPAWN_DISTANCE, Player::Two),
    };

    commands.insert_resource(players);
}

#[derive(Bundle, Default)]
struct PlayerDefaults {
    defense: Defense,
    health: Health,
    resources: Resources,
    inventory: Inventory,
    spawner: HitboxSpawner,
    player_velocity: PlayerVelocity,
    move_buffer: MoveBuffer,
}

fn spawn_player(commands: &mut Commands, models: &Models, offset: f32, player: Player) -> Entity {
    let state = PlayerState::default();
    let character = dummy();

    let mut spawn_handle = commands.spawn_bundle(SpatialBundle {
        transform: Transform::from_translation((offset, PLAYER_SPAWN_HEIGHT, 0.0).into()),
        ..default()
    });

    spawn_handle
        .insert_bundle(PlayerDefaults::default())
        .insert_bundle(PadBundle::new(character.get_inputs()))
        .insert(Name::new(format!("Player {player}")))
        .insert(AnimationHelperSetup)
        .insert(Facing::from_flipped(offset.is_sign_positive()))
        .insert(Hurtbox(character.get_hurtbox(false)))
        .insert(Pushbox(character.get_pushbox(false)))
        .insert(character.clone())
        .insert(player)
        .insert(state);

    spawn_handle.with_children(|parent| {
        parent.spawn_bundle(SceneBundle {
            scene: models[&character.model].clone(),
            ..default()
        });
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
        &mut InputParser,
    )>,
    mut game_state: ResMut<State<GameState>>,
    mut clock: ResMut<Clock>,
    bevy_time: Res<Time>,
) {
    // Just pressed would be better, but it's difficult in tests and the difference is very minor.
    if keys.pressed(KeyCode::Return) {
        game_state.set(GameState::Combat).unwrap();
        clock.reset(bevy_time.seconds_since_startup());
        commands.remove_resource::<RoundResult>();

        for (mut health, mut resources, mut tf, player, mut player_state, mut buffer, mut parser) in
            &mut query
        {
            health.reset();
            resources.reset();
            player_state.reset();
            buffer.clear_all();
            parser.clear();

            tf.translation = Vec3::new(
                match *player {
                    Player::One => -PLAYER_SPAWN_DISTANCE,
                    Player::Two => PLAYER_SPAWN_DISTANCE,
                },
                PLAYER_SPAWN_HEIGHT,
                0.0,
            );
        }
    }
}
