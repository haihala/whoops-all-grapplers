use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CharacterId {
    #[default]
    Dummy,
}
impl FromStr for CharacterId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dummy" => Ok(Self::Dummy),
            _ => Err(format!("Unknown character: {}", s)),
        }
    }
}