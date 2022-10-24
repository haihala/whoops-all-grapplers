use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ItemId {
    Gi,
    Gun,
    HandMeDownKen,
    Drugs,

    #[default]
    Default,
}
