use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    scene::SceneInstance,
};
use characters::{ActionEvent, FlashRequest};
use player_state::PlayerState;

// Extended Flash Material
type EFM = ExtendedMaterial<StandardMaterial, FlashMaterial>;

pub fn handle_flash_events(
    mut materials: ResMut<Assets<EFM>>,
    handles: Query<(Entity, &Handle<EFM>)>,
    parents: Query<&Parent>,
    mut players: Query<(Entity, &mut PlayerState)>,
    time: Res<Time>,
) {
    for (root, mut state) in &mut players {
        for flash_request in state.drain_matching_actions(|action| {
            if let ActionEvent::Flash(flash_request) = action {
                Some(flash_request.to_owned())
            } else {
                None
            }
        }) {
            for (material_entity, handle) in &handles {
                let mut parent = parents.get(material_entity).unwrap();

                while let Ok(next) = parents.get(**parent) {
                    parent = next;
                }

                // Root level parent ought to be the player
                if **parent != root {
                    continue;
                }

                let material = materials.get_mut(handle).unwrap();
                material.extension =
                    FlashMaterial::from_request(flash_request, time.elapsed_seconds());
            }
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct FlashMaterial {
    // Start at a high binding number to ensure bindings don't conflict
    // with the base material
    #[uniform(100)]
    pub color: Color,
    #[uniform(101)]
    pub speed: f32,
    #[uniform(102)]
    pub depth: f32, // How far into the flash to go? 1 means go full monochrome color, 0 means no change
    #[uniform(103)]
    pub duration: f32,
    #[uniform(104)]
    pub start_time: f32,
}
impl MaterialExtension for FlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
}
impl FlashMaterial {
    pub fn from_request(request: FlashRequest, time: f32) -> Self {
        Self {
            color: request.color,
            speed: request.speed,
            depth: request.depth,
            duration: request.duration,
            start_time: time,
        }
    }
}

#[derive(Component)]
pub struct UpdateMaterial;

// From https://github.com/bevyengine/bevy/discussions/8533
pub fn customize_scene_materials(
    unloaded_instances: Query<(Entity, &SceneInstance), With<UpdateMaterial>>,
    handles: Query<(Entity, &Handle<StandardMaterial>)>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    scene_manager: Res<SceneSpawner>,
    mut materials: ResMut<Assets<EFM>>,
    mut cmds: Commands,
) {
    for (entity, instance) in &unloaded_instances {
        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).remove::<UpdateMaterial>();
        }

        // Iterate over all entities in scene (once it's loaded)
        let handles = handles.iter_many(scene_manager.iter_instance_entities(**instance));
        for (entity, material_handle) in handles {
            let Some(old_material) = pbr_materials.get(material_handle) else {
                continue;
            };
            let material = materials.add(ExtendedMaterial {
                base: old_material.clone(),
                extension: FlashMaterial::from_request(FlashRequest::default(), 0.0),
            });

            cmds.entity(entity)
                .insert(material)
                .remove::<Handle<StandardMaterial>>();
        }
    }
}
