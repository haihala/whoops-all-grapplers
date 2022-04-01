mod charge;
mod meter;

pub trait GameResource<T>: Default {
    fn can_afford(&self, amount: T) -> bool;
    fn pay(&mut self, amount: T);
    fn get_ratio(&self) -> f32;
    fn reset(&mut self) {
        *self = Self::default();
    }
}

pub use charge::*;
pub use meter::*;
