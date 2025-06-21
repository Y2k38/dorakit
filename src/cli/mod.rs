mod convert;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(name = "dorakit")]
#[command(about = "A Doraemon-style pocket of tools")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Convert(convert::ConvertArgs),
}
