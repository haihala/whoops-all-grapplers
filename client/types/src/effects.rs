#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum SoundEffect {
    Whoosh,
    Clash,
    Block,
    Hit,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisualEffect {
    Clash,
    Block,
    Hit,
}
