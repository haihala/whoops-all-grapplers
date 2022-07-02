#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SoundEffect {
    Whoosh,
    Block,
    Hit,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisualEffect {
    Block,
    Hit,
}
