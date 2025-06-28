mod length;
mod temp;
mod weight;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ConvertArgs {
    #[command(subcommand)]
    pub command: ConvertCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConvertCommand {
    Temp(temp::TempConvertArgs),
    Weight(weight::WeightConvertArgs),
    Length(length::LengthConvertArgs),
}

impl ConvertCommand {
    pub fn run(&self) {
        match self {
            ConvertCommand::Temp(args) => args.run(),
            ConvertCommand::Weight(args) => args.run(),
            ConvertCommand::Length(args) => args.run(),
        }
    }
}
