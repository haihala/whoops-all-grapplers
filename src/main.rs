use bevy::prelude::*;

mod hello;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(hello::HelloPlugin)
        .run();
}
