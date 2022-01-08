use bevy_inspector_egui::Inspectable;

mod air_activity;
mod ground_activity;

pub use air_activity::AirActivity;
pub use ground_activity::GroundActivity;

#[derive(Inspectable, PartialEq, Clone, Debug)]
pub enum PrimaryState {
    Air(AirActivity),
    Ground(GroundActivity),
}
