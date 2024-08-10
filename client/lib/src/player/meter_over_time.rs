use bevy::prelude::*;
use characters::{ResourceType, WAGResources};
use wag_core::{Clock, Stats};

pub fn meter_over_time(clock: Res<Clock>, mut players: Query<(&Stats, &mut WAGResources)>) {
    for (stats, mut resources) in &mut players {
        let mut gain = (stats.meter_per_second / 60.0).floor() as i32;

        let fraction = stats.meter_per_second % 60.0;
        if fraction != 0.0 {
            let interval = (60.0 / fraction).floor() as usize;
            gain += (clock.frame % interval == 0) as i32;
        }

        let meter = resources.get_mut(ResourceType::Meter).unwrap();
        meter.gain(gain);
    }
}
