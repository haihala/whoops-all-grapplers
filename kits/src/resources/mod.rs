mod charge;
mod cost;
mod meter;

pub use charge::Charge;
pub use cost::Cost;
pub use meter::Meter;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Debug, Component, Clone, Copy, Default, PartialEq)]
pub struct Resources {
    pub charge: Charge,
    pub meter: Meter,
}
impl Resources {
    pub fn reset(&mut self) {
        self.charge.reset();
        self.meter.reset();
    }

    pub fn can_afford(&self, cost: &Option<Cost>) -> bool {
        if let Some(costs) = cost {
            self.charge.can_afford(costs.charge) && self.meter.can_afford(costs.meter)
        } else {
            true
        }
    }

    pub fn pay(&mut self, cost: Option<Cost>) {
        if let Some(costs) = cost {
            self.charge.pay(costs.charge);
            self.meter.pay(costs.meter);
        }
    }
}
