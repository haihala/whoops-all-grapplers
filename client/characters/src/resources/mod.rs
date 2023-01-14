mod bullets;
mod charge;
mod cost;
mod meter;

pub use bullets::Bullets;
pub use charge::Charge;
pub use cost::Cost;
pub use meter::Meter;

use bevy::prelude::*;

#[derive(Reflect, Debug, Component, Clone, Copy, Default, PartialEq, Eq)]
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

    pub fn can_afford(&self, cost: Cost) -> bool {
        self.meter.can_afford(cost.meter)
            && (!cost.charge || self.charge.is_charged())
            && (!cost.bullet || self.bullets.has_one())
    }

    pub fn pay(&mut self, cost: Cost) {
        self.meter.pay(cost.meter);

        if cost.charge {
            self.charge.consume_charge();
        }

        if cost.bullet {
            self.bullets.use_one();
        }
    }
}
