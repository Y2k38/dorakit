mod temp;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ConvertArgs {
    #[command(subcommand)]
    pub command: ConvertCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConvertCommand {
    Temp(temp::TempConvertArgs),
}

impl ConvertCommand {
    pub fn run(&self) {
        match self {
            ConvertCommand::Temp(args) => args.run(),
        }
    }
}
