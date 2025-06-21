use crate::convert::temp::{TempUnit, convert_temperature};
use clap::Args;

#[derive(Debug, Args)]
pub struct TempConvertArgs {
    #[arg(long)]
    pub from: String,

    #[arg(long)]
    pub to: String,

    #[arg(long)]
    pub value: Option<f64>,

    #[arg(long, value_names = ["START", "END"])]
    pub range: Option<Vec<f64>>,

    #[arg(long)]
    pub step: Option<f64>,
}

impl TempConvertArgs {
    pub fn run(&self) {
        let from = TempUnit::from_str(&self.from).expect("Invalid --from");
        let to = TempUnit::from_str(&self.to).expect("Invalid --to");

        match (&self.range, self.value) {
            (Some(range), _) => {
                if range.len() != 2 {
                    eprintln!("--range must provide exactly two numbers: START END");
                    std::process::exit(1);
                }

                let step = self.step.unwrap_or(1.0);

                let start = range[0];
                let end = range[1];

                let mut current = start;
                while current <= end {
                    let result = convert_temperature(current, from, to);
                    println!("{:.1}°{} = {:.1}°{}", current, self.from, result, self.to);
                    current += step;
                }
            }

            (None, Some(val)) => {
                let result = convert_temperature(val, from, to);
                println!("{:.1}°{} = {:.1}°{}", val, self.from, result, self.to);
            }

            _ => {
                eprintln!("Either --value or --range must be provided");
                std::process::exit(1);
            }
        }
    }
}
