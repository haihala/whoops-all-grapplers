use bevy_inspector_egui::Inspectable;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default, Inspectable)]
pub enum SoundEffect {
    Whoosh,
    Clash,
    Block,
    Hit,
    #[default]
    Silence,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisualEffect {
    Clash,
    Block,
    Hit,
}
