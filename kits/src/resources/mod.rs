mod bullets;
mod charge;
mod cost;
mod meter;

pub use bullets::Bullets;
pub use charge::Charge;
pub use cost::Cost;
pub use meter::Meter;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Debug, Component, Clone, Copy, Default, PartialEq)]
pub struct Resources {
    pub bullets: Bullets,
    pub charge: Charge,
    pub meter: Meter,
}
impl Resources {
    pub fn reset(&mut self) {
        self.bullets.reset();
        self.charge.reset();
        self.meter.reset();
    }

    pub fn can_afford(&self, cost: &Option<Cost>) -> bool {
        if let Some(costs) = cost {
            self.meter.can_afford(costs.meter)
                && (!costs.charge || self.charge.is_charged())
                && (!costs.bullet || self.bullets.has_one())
        } else {
            true
        }
    }

    pub fn pay(&mut self, cost: Option<Cost>) {
        if let Some(costs) = cost {
            self.meter.pay(costs.meter);

            if costs.charge {
                self.charge.consume_charge();
            }

            if costs.bullet {
                self.bullets.use_one();
            }
        }
    }
}
