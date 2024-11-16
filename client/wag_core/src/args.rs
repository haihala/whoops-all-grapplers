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
    #[clap(long, default_value = "2")]
    pub input_delay: usize,
}
impl WagArgs {
    pub fn from_cli() -> Self {
        Self::parse()
    }

    pub fn extra_starting_money(&self) -> usize {
        if let Some(Dev::Local {
            pad1: _,
            pad2: _,
            character1: _,
            character2: _,
            starting_money,
        }) = self.dev
        {
            starting_money
        } else {
            0
        }
    }
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Dev {
    Online {
        local_controller: usize,
        local_character: CharacterId,
    },
    Synctest {
        local_controller: usize,
        local_character: CharacterId,
    },
    Local {
        pad1: usize,
        pad2: usize,
        character1: CharacterId,
        character2: CharacterId,
        starting_money: usize,
    },
}
