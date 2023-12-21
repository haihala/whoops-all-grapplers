use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use characters::{ActionEvent, FlashRequest};
use player_state::PlayerState;

// Extended Flash Material
pub type ExtendedFlashMaterial = ExtendedMaterial<StandardMaterial, FlashMaterial>;

pub fn handle_flash_events(
    mut materials: ResMut<Assets<ExtendedFlashMaterial>>,
    handles: Query<(Entity, &Handle<ExtendedFlashMaterial>)>,
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
        "shaders/flash_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/flash_material.wgsl".into()
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
