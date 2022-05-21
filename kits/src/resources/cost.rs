use bevy_inspector_egui::Inspectable;

#[derive(Debug, Default, Inspectable, Clone, PartialEq)]
pub struct Cost {
    pub meter: i32,
    pub charge: bool,
    pub bullet: bool,
}
