use super::CharacterId;
use bevy::prelude::*;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug, Resource, Default, Clone)]
#[command(author, version, about, long_about = None)]
pub struct WagArgs {
    /// Dev mode (shows hitboxes and dev binds)
    #[arg(short, long, default_value_t = false)]
    pub dev: bool,
    #[arg(default_value_t = CharacterId::Mizku)]
    pub character1: CharacterId,
    #[arg(default_value_t = CharacterId::Mizku)]
    pub character2: CharacterId,
}
impl WagArgs {
    pub fn from_cli() -> Self {
        Self::parse()
    }
}
