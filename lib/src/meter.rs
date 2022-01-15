use bevy_inspector_egui::Inspectable;

#[derive(Inspectable)]
pub struct Meter {
    // See Health comment
    ratio: f32,
    value: i32,
    max: i32,
}

impl Default for Meter {
    fn default() -> Self {
        Self {
            ratio: 1.0,
            value: 100,
            max: 100,
        }
    }
}
impl Meter {
    pub fn get_ratio(&self) -> f32 {
        self.ratio
    }
    pub fn reset(&mut self) {
        self.value = self.max;
        self.ratio = 1.0;
    }
    pub fn can_afford(&self, amount: i32) -> bool {
        self.value >= amount
    }
    pub fn pay(&mut self, amount: i32) {
        self.gain(-amount);
    }
    pub fn gain(&mut self, amount: i32) {
        self.value += amount;
        self.ratio = self.value as f32 / self.max as f32;
    }
}
