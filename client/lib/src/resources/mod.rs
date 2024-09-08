use bevy::prelude::*;
use wag_core::RollbackSchedule;

mod charge_accumulator;
mod economy;
mod meter_over_time;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (economy::modify_properties, economy::manage_item_consumption),
        )
        .add_systems(
            RollbackSchedule,
            (
                charge_accumulator::manage_charge,
                meter_over_time::meter_over_time,
            ),
        );
    }
}
