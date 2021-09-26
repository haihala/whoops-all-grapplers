use bevy::prelude::*;
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorPlugin};

use crate::{Clock, Health, Player};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut registry = app
            .add_plugin(WorldInspectorPlugin::new())
            .insert_resource(InspectableRegistry::default())
            .world_mut()
            .get_resource_mut::<InspectableRegistry>()
            .expect("InspectableRegistry not initiated");

        registry.register::<Health>();
        registry.register::<Player>();
        registry.register::<Clock>();
    }
}
