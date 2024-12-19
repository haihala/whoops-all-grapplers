mod condition_management;
mod force_state;
mod move_activation;
mod move_advancement;
mod player_flash;
mod recovery;
mod side_switcher;
mod size_adjustment;

use characters::{samurai, Gauges, Hurtboxes, Inventory};
use foundation::{
    AnimationType, CharacterClock, CharacterFacing, CharacterId, Characters, Clock, Combo, Facing,
    InMatch, MatchState, Player, Players, RollbackSchedule, Sound, Stats, SystemStep, WagArgs,
};
use input_parsing::{InputParser, PadBundle};
use player_state::PlayerState;

use crate::{
    assets::{
        AnimationHelper, AnimationHelperSetup, CharacterShake, Models, Music, PlayerModelHook,
    },
    damage::HitboxSpawner,
    event_spreading,
    movement::{PlayerVelocity, Pushbox, GROUND_PLANE_HEIGHT},
};

use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

pub use move_activation::MoveBuffer;

const PLAYER_SPAWN_DISTANCE: f32 = 2.5; // Distance from x=0(middle)

pub struct PlayerStateManagementPlugin;

impl Plugin for PlayerStateManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MatchState::Loading), setup_players)
            .add_systems(OnEnter(MatchState::PreRound), setup_combat)
            .add_systems(
                RollbackSchedule,
                side_switcher::sideswitcher
                    .run_if(in_state(InMatch))
                    .in_set(SystemStep::SideSwitch),
            )
            .add_systems(
                RollbackSchedule,
                (
                    condition_management::expire_conditions,
                    condition_management::update_combined_status_effect,
                )
                    .chain()
                    .in_set(SystemStep::Conditions),
            )
            .add_systems(
                RollbackSchedule,
                (
                    move_activation::manage_buffer,
                    move_activation::move_activator,
                    move_advancement::move_advancement,
                )
                    .chain()
                    .in_set(SystemStep::MovePipeline),
            )
            .add_systems(
                RollbackSchedule,
                (
                    recovery::stun_recovery,
                    recovery::ground_recovery,
                    size_adjustment::update_box_sizes_from_state,
                    size_adjustment::remove_old_hurtbox_expansions,
                )
                    .chain()
                    .in_set(SystemStep::PlayerUpdates),
            );
    }
}

fn setup_players(
    mut commands: Commands,
    characters: Res<Characters>,
    models: Res<Models>,
    args: Res<WagArgs>,
    mut music: ResMut<Music>,
) {
    music.push(match characters.p1 {
        CharacterId::Samurai => Sound::Motivation,
    });
    let players = Players {
        one: spawn_player(
            &mut commands,
            &models,
            -PLAYER_SPAWN_DISTANCE,
            Player::One,
            characters.p1,
            &args,
        ),
        two: spawn_player(
            &mut commands,
            &models,
            PLAYER_SPAWN_DISTANCE,
            Player::Two,
            characters.p2,
            &args,
        ),
    };

    commands.insert_resource(players);
}

#[derive(Bundle, Default)]
struct PlayerDefaults {
    character_clock: CharacterClock,
    character_shake: CharacterShake,
    move_buffer: MoveBuffer,
    player_velocity: PlayerVelocity,
    spawner: HitboxSpawner,
    state: PlayerState,
    status_effects: Stats,
    visibility: Visibility,
}

fn spawn_player(
    commands: &mut Commands,
    models: &Models,
    offset: f32,
    player: Player,
    character_id: CharacterId,
    args: &WagArgs,
) -> Entity {
    let character = match character_id {
        CharacterId::Samurai => samurai(),
    };

    let colors = character.colors[&player].clone();
    let model = character.model;

    commands
        .spawn((
            Transform::from_translation(Vec3::new(offset, GROUND_PLANE_HEIGHT, 0.0)),
            Gauges::from_stats(&character.base_stats, character.special_properties.clone()),
            PlayerDefaults::default(),
            PadBundle::new(character.get_inputs()),
            Name::new(format!("Player {player}")),
            AnimationHelperSetup(character.generic_animations[&AnimationType::Default]),
            CharacterFacing::from(Facing::from_flipped(offset.is_sign_positive())),
            Pushbox(character.boxes.standing.pushbox),
            Hurtboxes::from(character.boxes.standing),
            character,
            player,
            StateScoped(InMatch),
            {
                let mut inv = Inventory::default();
                inv.money += args.extra_starting_money();
                inv
            },
        ))
        .with_children(move |parent| {
            // Root bone of the model moves with the animation (resets position)
            // Have this as an intermediate layer for when we want to offset the animation
            parent
                .spawn((Transform::default(), Visibility::default()))
                .with_children(|model_pivot| {
                    model_pivot.spawn((
                        PlayerModelHook(colors.clone()),
                        SceneRoot(models[&model].clone()),
                    ));
                });
        })
        .add_rollback()
        .observe(event_spreading::spread_events)
        .observe(condition_management::activate_conditions)
        .observe(condition_management::clear_conditions)
        .observe(force_state::force_state)
        .observe(move_activation::automatic_activation)
        .observe(move_advancement::end_moves)
        .observe(player_flash::handle_flash_events)
        .observe(player_flash::handle_color_shift)
        .observe(side_switcher::flip_visuals)
        .observe(size_adjustment::expand_hurtboxes)
        .observe(crate::assets::start_animation)
        .observe(crate::assets::start_relative_vfx)
        .observe(crate::assets::play_voiceline)
        .observe(crate::assets::shake_character)
        .observe(crate::camera::tilt_camera)
        .observe(crate::damage::snap_and_switch)
        .observe(crate::damage::hitstun_events)
        .observe(crate::damage::blockstun_events)
        .observe(crate::damage::launch_events)
        .observe(crate::damage::spawn_hitbox)
        .observe(crate::movement::clear_movement)
        .observe(crate::movement::add_movement)
        .observe(crate::movement::handle_teleports)
        .observe(crate::pickup_management::spawn_pickups)
        .observe(crate::resources::modify_properties)
        .observe(crate::resources::clear_properties)
        .id()
}

#[allow(clippy::type_complexity)]
fn setup_combat(
    mut query: Query<(
        &Player,
        &Stats,
        &mut Gauges,
        &mut Transform,
        &mut PlayerState,
        &mut MoveBuffer,
        &mut InputParser,
        &mut PlayerVelocity,
        &mut AnimationHelper,
        &mut Hurtboxes,
        &mut CharacterClock,
        Entity,
    )>,
    mut commands: Commands,
    mut clock: ResMut<Clock>,
) {
    info!("Round start reset");
    clock.reset();

    for (
        player,
        stats,
        mut resources,
        mut tf,
        mut player_state,
        mut buffer,
        mut parser,
        mut velocity,
        mut animation_helper,
        mut hurtboxes,
        mut char_clock,
        entity,
    ) in &mut query
    {
        resources.reset(stats);
        player_state.reset();
        buffer.reset();
        parser.clear();
        velocity.reset();
        animation_helper.reset();
        hurtboxes.reset();
        char_clock.reset();
        commands.entity(entity).remove::<Combo>();

        tf.translation = Vec3::new(
            match *player {
                Player::One => -PLAYER_SPAWN_DISTANCE,
                Player::Two => PLAYER_SPAWN_DISTANCE,
            },
            GROUND_PLANE_HEIGHT,
            0.0,
        );
    }
}
