use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Dev mode (shows hitboxes and dev binds)
    #[arg(short, long, default_value_t = false)]
    pub dev: bool,
}

pub fn parse() -> Args {
    Args::parse()
}
