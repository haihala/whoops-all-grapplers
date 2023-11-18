#![feature(trivial_bounds)]
#![feature(extract_if)]
#![feature(exclusive_range_pattern)]
#![feature(iter_intersperse)]

mod assets;
mod camera;
mod damage;
mod dev;
mod economy;
mod physics;
mod player;
mod stage;
mod state_transitions;
mod ui;

use bevy::{app::PluginGroupBuilder, prelude::*};
use wag_core::WagArgs;

// Only thing exported out of this crate
#[derive(Debug)]
pub struct WAGLib {
    enable_hanabi: bool,
    args: WagArgs,
}

impl WAGLib {
    pub fn integration() -> Self {
        Self {
            args: WagArgs::default(),
            enable_hanabi: false,
        }
    }
    pub fn with_args(args: WagArgs) -> Self {
        Self {
            args,
            enable_hanabi: true,
        }
    }
}

impl PluginGroup for WAGLib {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Order matters here, loaded in the defined order
        if self.enable_hanabi {
            group = group.add(bevy_hanabi::HanabiPlugin);
        }

        group = group
            .add(wag_core::TimePlugin) // Has to be first, since it defines labels for ordering other systems
            .add(assets::AssetsPlugin) // Has to be before those assets are used
            .add(ui::UIPlugin)
            .add(camera::CustomCameraPlugin)
            .add(player::PlayerPlugin)
            .add(economy::EconomyPlugin)
            .add(damage::DamagePlugin)
            .add(physics::PhysicsPlugin)
            .add(input_parsing::InputParsingPlugin)
            .add(stage::StagePlugin)
            .add(state_transitions::StateTransitionPlugin)
            .add(ArgsPlugin::new(self.args.clone()));

        if self.args.dev {
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
