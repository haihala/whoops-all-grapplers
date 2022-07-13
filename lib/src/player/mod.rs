mod charge_accumulator;
mod model_flipper;
mod move_activation;
mod move_advancement;
mod movement;
mod recovery;
mod size_adjustment;
mod update_animation;

use characters::{dummy, Character, Grabable, Hurtbox, Inventory, Resources};
use input_parsing::InputParser;
#[cfg(not(test))]
use input_parsing::PadBundle;
use player_state::PlayerState;
use time::{once_per_combat_frame, Clock, GameState, RoundResult};
use types::{Facing, Model, Player, Players};

use crate::{
    assets::{AnimationHelperSetup, ModelRequest},
    damage::{Health, HitboxSpawner},
    physics::{PlayerVelocity, Pushbox, GROUND_PLANE_HEIGHT},
};

use bevy::{ecs::query::WorldQuery, prelude::*};

use self::{model_flipper::PlayerModel, move_activation::MoveBuffer};

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)
const PLAYER_SPAWN_HEIGHT: f32 = GROUND_PLANE_HEIGHT + 0.001;

#[derive(WorldQuery)]
#[world_query(mutable)]
struct PlayerQuery<'a> {
    state: &'a mut PlayerState,
    spawner: &'a mut HitboxSpawner,
    character: &'a Character,
    tf: &'a Transform,
    grabbable: &'a mut Grabable,
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
            .add_system(model_flipper::model_flipper)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(once_per_combat_frame)
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
                        update_animation::update_animation.after(charge_accumulator::manage_charge),
                    ),
            );
    }
}

fn setup(mut commands: Commands) {
    let players = Players {
        one: spawn_player(&mut commands, -PLAYER_SPAWN_DISTANCE, Player::One),
        two: spawn_player(&mut commands, PLAYER_SPAWN_DISTANCE, Player::Two),
    };

    commands.insert_resource(players);
}

#[derive(Bundle, Default)]
struct PlayerDefaults {
    health: Health,
    resources: Resources,
    inventory: Inventory,
    spawner: HitboxSpawner,
    grab_target: Grabable,
    player_velocity: PlayerVelocity,
    move_buffer: MoveBuffer,
}

fn spawn_player(commands: &mut Commands, offset: f32, player: Player) -> Entity {
    let state = PlayerState::default();
    let character = dummy();

    #[cfg(not(test))]
    let inputs = PadBundle::new(character.get_inputs());

    let mut spawn_handle = commands.spawn_bundle(TransformBundle {
        local: Transform::from_translation((offset, PLAYER_SPAWN_HEIGHT, 0.0).into()),
        ..default()
    });

    spawn_handle
        .insert_bundle(PlayerDefaults::default())
        .insert(Name::new(format!("Player {}", player)))
        .insert(AnimationHelperSetup)
        .insert(Facing::from_flipped(offset.is_sign_positive()))
        .insert(Hurtbox(character.get_hurtbox(false)))
        .insert(Pushbox(character.get_pushbox(false)))
        .insert(character)
        .insert(player)
        .insert(state);

    #[cfg(not(test))]
    spawn_handle.insert_bundle(inputs);

    spawn_handle.with_children(|parent| {
        parent
            .spawn_bundle(TransformBundle::default())
            .insert(ModelRequest(Model::Dummy))
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
        &mut InputParser,
    )>,
    mut game_state: ResMut<State<GameState>>,
    mut clock: ResMut<Clock>,
) {
    // Just pressed would be better, but it's difficult in tests and the difference is very minor.
    if keys.pressed(KeyCode::Return) {
        game_state.set(GameState::Combat).unwrap();
        clock.reset();
        commands.remove_resource::<RoundResult>();

        for (mut health, mut resources, mut tf, player, mut player_state, mut buffer, mut parser) in
            query.iter_mut()
        {
            health.reset();
            resources.reset();
            player_state.reset();
            buffer.clear();
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
