use bevy::prelude::*;
use characters::{GaugeType, Gauges};
use foundation::{Clock, Stats};

pub fn meter_over_time(clock: Res<Clock>, mut players: Query<(&Stats, &mut Gauges)>) {
    for (stats, mut resources) in &mut players {
        let mut gain = (stats.meter_per_second / 60.0).floor() as i32;

        let fraction = stats.meter_per_second % 60.0;
        if fraction != 0.0 {
            let interval = (60.0 / fraction).floor() as usize;
            gain += (clock.frame % interval == 0) as i32;
        }

        let meter = resources.get_mut(GaugeType::Meter).unwrap();
        meter.gain(gain);
    }
}
