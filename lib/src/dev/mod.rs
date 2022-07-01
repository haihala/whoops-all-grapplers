use bevy::prelude::*;
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorPlugin};

use kits::{Inventory, Kit, Resources};
use player_state::PlayerState;
use time::Clock;
use types::{Player, SoundEffect};

use crate::{
    assets::Sounds,
    damage::Health,
    physics::{ConstantVelocity, PlayerVelocity},
};

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        let mut registry = app
            .add_plugin(WorldInspectorPlugin::new())
            .insert_resource(InspectableRegistry::default())
            .add_system(test_system)
            .world
            .resource_mut::<InspectableRegistry>();

        registry.register::<Player>();
        registry.register::<Resources>();
        registry.register::<Health>();
        registry.register::<PlayerState>();
        registry.register::<Clock>();
        registry.register::<PlayerVelocity>();
        registry.register::<ConstantVelocity>();
    }
}

fn test_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Inventory, &Kit)>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    // B for Buy
    if keys.just_pressed(KeyCode::B) {
        for (mut inventory, kit) in query.iter_mut() {
            if let Some((id, _)) = kit.roll_items(1, &inventory).first() {
                inventory.add_item(*id);
            }
        }
    } else if keys.just_pressed(KeyCode::S) {
        dbg!("Playing");
        audio.play(sounds.get(SoundEffect::Whoosh));
    }
}
