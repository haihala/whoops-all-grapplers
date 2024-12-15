use bevy::{
    pbr::ExtendedMaterial, prelude::*, render::view::NoFrustumCulling, scene::SceneInstance,
    utils::HashMap,
};
use foundation::{Clock, MatchState, Model};

use crate::event_spreading::ShakeCharacter;

use super::{ExtendedFlashMaterial, FlashMaterial};

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct Models(pub HashMap<Model, Handle<Scene>>);

#[derive(Debug, Component, Default)]
pub struct CharacterShake {
    amount: f32,
}

pub(super) fn model_paths() -> HashMap<Model, &'static str> {
    // TODO: This could use the bevy 0.14 typed asset handles instead of static strings
    // So far, I think that is a waste of effort.
    vec![
        (Model::Samurai, "samurai.glb#Scene0"),
        (Model::Fireball, "fireball.glb#Scene0"),
        (Model::Kunai, "kunai.glb#Scene0"),
        (Model::TrainingStage, "stage.glb#Scene0"),
    ]
    .into_iter()
    .collect()
}

#[derive(Component, Debug)]
pub struct PlayerModelHook(pub HashMap<&'static str, Color>);

#[allow(clippy::too_many_arguments)]
pub fn prep_player_gltf(
    unloaded_instances: Query<(Entity, &SceneInstance, &PlayerModelHook)>,
    material_handles: Query<(Entity, &MeshMaterial3d<StandardMaterial>, &Name)>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    scene_manager: Res<SceneSpawner>,
    mut materials: ResMut<Assets<ExtendedFlashMaterial>>,
    mut commands: Commands,
) {
    for (entity, instance, update_material) in &unloaded_instances {
        if scene_manager.instance_is_ready(**instance) {
            commands.entity(entity).remove::<PlayerModelHook>();
        }

        // Iterate over all entities in scene (once it's loaded)
        for (entity, material_handle, name) in
            material_handles.iter_many(scene_manager.iter_instance_entities(**instance))
        {
            commands.entity(entity).insert(NoFrustumCulling);

            let Some(old_material) = pbr_materials.get(material_handle) else {
                continue;
            };

            let mut base_material = old_material.clone();

            if let Some(color) = update_material.0.get(name.as_str()) {
                base_material.base_color = *color;
            }

            let material = materials.add(ExtendedMaterial {
                base: base_material,
                extension: FlashMaterial::default(),
            });

            commands
                .entity(entity)
                .remove::<MeshMaterial3d<StandardMaterial>>()
                .insert(MeshMaterial3d(material.clone()));
        }
    }
}

pub fn shake_character(
    trigger: Trigger<ShakeCharacter>,
    mut charshakes: Query<&mut CharacterShake>,
) {
    let ShakeCharacter(amount) = trigger.event();
    charshakes.get_mut(trigger.entity()).unwrap().amount = *amount;
}

const SHAKE_SPEED: f32 = 2.0;
const SHAKE_DECAY: f32 = 0.8;
const SHAKE_CUTOFF: f32 = 0.01;

pub fn do_character_shake(
    mut players: Query<(&mut CharacterShake, &Children)>,
    mut tfs: Query<&mut Transform>,
    clock: Res<Clock>,
    match_state: Res<State<MatchState>>,
) {
    let post_round = *match_state.get() == MatchState::PostRound;

    for (mut cs, children) in &mut players {
        let mut tf = tfs.get_mut(children[0]).unwrap();
        tf.translation.x = cs.amount * (clock.frame as f32 * SHAKE_SPEED).sin().signum();

        cs.amount = if cs.amount < SHAKE_CUTOFF || post_round {
            0.0
        } else {
            cs.amount * SHAKE_DECAY
        };
    }
}
