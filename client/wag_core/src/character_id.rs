use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CharacterId {
    #[default]
    Dummy,
    Mizku,
}
impl FromStr for CharacterId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dummy" => Ok(Self::Dummy),
            "mizku" => Ok(Self::Mizku),
            _ => Err(format!("Unknown character: {}", s)),
        }
    }
}
impl std::fmt::Display for CharacterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterId::Dummy => write!(f, "dummy"),
            CharacterId::Mizku => write!(f, "mizku"),
        }
    }
}
