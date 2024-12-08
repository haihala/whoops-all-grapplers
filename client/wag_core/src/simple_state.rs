#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SimpleState {
    Air,
    #[default]
    Stand,
    Crouch,
}
