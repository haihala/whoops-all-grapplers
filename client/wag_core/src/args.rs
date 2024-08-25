use super::CharacterId;
use bevy::prelude::*;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug, Resource, Clone, Default)]
#[command(author, version, about, long_about = None)]
pub struct WagArgs {
    /// Dev mode (shows hitboxes and dev binds)
    #[arg(short, long, default_value_t = false)]
    pub dev: bool,
    #[arg(requires = "dev")]
    pub pad1: Option<usize>,
    #[arg(requires = "dev")]
    pub pad2: Option<usize>,
    #[arg(requires = "dev")]
    pub character1: Option<CharacterId>,
    #[arg(requires = "dev")]
    pub character2: Option<CharacterId>,
}
impl WagArgs {
    pub fn from_cli() -> Self {
        Self::parse()
    }
}
