use super::CharacterId;
use bevy::prelude::*;
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug, Resource, Clone, Default)]
#[command(author, version, about, long_about = None)]
pub struct WagArgs {
    /// Dev mode (shows hitboxes and dev binds)
    #[command(subcommand)]
    pub dev: Option<Dev>,
}
impl WagArgs {
    pub fn from_cli() -> Self {
        Self::parse()
    }
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Dev {
    Online {
        local_controller: usize,
        local_character: CharacterId,
    },

    Local {
        pad1: usize,
        pad2: usize,
        character1: CharacterId,
        character2: CharacterId,
    },
}
