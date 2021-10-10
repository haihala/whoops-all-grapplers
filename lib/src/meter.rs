use bevy_inspector_egui::Inspectable;

#[derive(Inspectable)]
pub struct Meter {
    // See Health comment
    ratio: f32,
}

impl Default for Meter {
    fn default() -> Self {
        Self { ratio: 1.0 }
    }
}
impl Meter {
    pub fn get_ratio(&self) -> f32 {
        self.ratio
    }
    pub fn reset(&mut self) {
        self.ratio = 1.0;
    }
}
