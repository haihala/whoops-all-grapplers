use bevy::prelude::*;

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system_to_stage(crate::stages::PRE_UPDATE, tick.system());
    }
}

pub struct Clock(pub i32);

fn setup(mut commands: Commands) {
    commands.insert_resource(Clock(0));
}

fn tick(mut clock: ResMut<Clock>) {
    clock.0 += 1;
}
