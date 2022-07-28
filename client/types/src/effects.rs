#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
