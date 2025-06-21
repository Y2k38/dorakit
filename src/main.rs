mod cli;
mod convert;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    match Cli::parse().command {
        Commands::Convert(args) => args.command.run(),
    }
}
