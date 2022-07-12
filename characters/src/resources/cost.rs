use bevy_inspector_egui::Inspectable;

#[derive(Debug, Default, Inspectable, Clone, Eq, PartialEq, Copy)]
pub struct Cost {
    pub meter: i32,
    pub charge: bool,
    pub bullet: bool,
}
