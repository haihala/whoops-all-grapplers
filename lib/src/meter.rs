use bevy_inspector_egui::Inspectable;

#[derive(Inspectable)]
pub struct Meter {
    // See Health comment
    pub ratio: f32,
}

impl Default for Meter {
    fn default() -> Self {
        Self { ratio: 1.0 }
    }
}
