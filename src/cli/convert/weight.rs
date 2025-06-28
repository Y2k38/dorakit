use crate::convert::weight::{WeightUnit, convert_weight};
use clap::Args;

#[derive(Debug, Args)]
pub struct WeightConvertArgs {
    #[arg(long)]
    pub from: String,

    #[arg(long)]
    pub to: String,

    #[arg(long)]
    pub value: f64,
}

impl WeightConvertArgs {
    pub fn run(&self) {
        let from = WeightUnit::from_str(&self.from).expect("Invalid --from");
        let to = WeightUnit::from_str(&self.to).expect("Invalid --to");

        let result = convert_weight(self.value, from, to);
        println!("{:.2}{} = {:.2}{}", self.value, self.from, result, self.to);
    }
}
