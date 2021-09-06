use bevy::prelude::*;
pub struct Clock(pub i32);

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Clock(0))
            .add_system_to_stage(CoreStage::First, tick.system());
    }
}

fn tick(mut clock: ResMut<Clock>) {
    clock.0 += 1;
}
