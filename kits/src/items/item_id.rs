use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemId {
    Default,
    Gi,
    Gun,
    HandMeDownKen,
    Drugs,
}

impl Default for ItemId {
    fn default() -> Self {
        Self::Default
    }
}
