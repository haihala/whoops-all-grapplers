#![feature(trivial_bounds)]
#![feature(iter_intersperse)]

mod assets;
mod camera;
mod damage;
mod dev;
mod entity_management;
mod event_spreading;
mod movement;
mod networking;
mod pickup_management;
mod player_state_management;
mod resources;
mod stage;
mod state_transitions;
mod ui;

use bevy::{app::PluginGroupBuilder, prelude::*};
use foundation::WagArgs;

// Only thing exported out of this crate
#[derive(Debug)]
pub struct Lib {
    args: WagArgs,
}

impl Lib {
    pub fn with_args(args: WagArgs) -> Self {
        Self { args }
    }
}

impl PluginGroup for Lib {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            .add(ArgsPlugin::new(self.args.clone()))
            .add(foundation::TimePlugin) // Has to be first, since it defines labels for ordering other systems
            .add(assets::AssetsPlugin) // Has to be before those assets are used
            .add(ui::UIPlugin)
            .add(camera::CustomCameraPlugin)
            .add(player_state_management::PlayerStateManagementPlugin)
            .add(resources::ResourcesPlugin)
            .add(damage::DamagePlugin)
            .add(movement::PhysicsPlugin)
            .add(input_parsing::InputParsingPlugin)
            .add(stage::StagePlugin)
            .add(state_transitions::StateTransitionPlugin)
            .add(networking::NetworkPlugin)
            .add(pickup_management::PickupPlugin)
            .add(entity_management::EntityManagementPlugin);

        if self.args.dev.is_some() {
            group = group.add(dev::DevPlugin);
        }
        group
    }
}

// This exists so we can make args to a resource, as you can't do that in the plugin group builder.
struct ArgsPlugin {
    args: WagArgs,
}
impl ArgsPlugin {
    fn new(args: WagArgs) -> Self {
        Self { args }
    }
}
impl Plugin for ArgsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.args.clone());
    }
}
