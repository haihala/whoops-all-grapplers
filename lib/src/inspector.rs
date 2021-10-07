use bevy::prelude::*;
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorPlugin};
use types::Player;

use crate::{character::PlayerState, physics::PhysicsObject, Clock, Health, Meter};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut registry = app
            .add_plugin(WorldInspectorPlugin::new())
            .insert_resource(InspectableRegistry::default())
            .world_mut()
            .get_resource_mut::<InspectableRegistry>()
            .expect("InspectableRegistry not initiated");

        registry.register::<Player>();
        registry.register::<Meter>();
        registry.register::<Health>();
        registry.register::<PlayerState>();
        registry.register::<Clock>();
        registry.register::<PhysicsObject>();
    }
}
