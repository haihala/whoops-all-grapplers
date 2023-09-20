use bevy::prelude::*;
use clap::Parser;
use wag_core::CharacterId;

/// Simple program to greet a person
#[derive(Parser, Debug, Resource, Default, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Dev mode (shows hitboxes and dev binds)
    #[arg(short, long, default_value_t = false)]
    pub dev: bool,
    pub character1: CharacterId,
    pub character2: CharacterId,
}

pub fn parse() -> CliArgs {
    CliArgs::parse()
}
