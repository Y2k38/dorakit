use crate::convert::length::{LengthUnit, convert_length};
use clap::Args;

#[derive(Debug, Args)]
pub struct LengthConvertArgs {
    #[arg(long)]
    pub from: String,

    #[arg(long)]
    pub to: String,

    #[arg(
        long,
        help = "Single or 3D value. e.g. 100, or 100x200x300, 100*200*300, 100,200,300"
    )]
    pub value: String,
}

impl LengthConvertArgs {
    pub fn run(&self) {
        let from = LengthUnit::from_str(&self.from).expect("Invalid --from");
        let to = LengthUnit::from_str(&self.to).expect("Invalid --to");
        let values = parse_length_value(&self.value)
            .unwrap_or_else(|err| panic!("Failed to parse --value '{}': {}", self.value, err));

        let mut converted = Vec::new();
        for &val in &values {
            converted.push(convert_length(val, from, to));
        }

        match values.len() {
            1 => println!(
                "{:.2} {} = {:.2} {}",
                values[0], self.from, converted[0], self.to
            ),
            3 => println!(
                "{:.2}x{:.2}x{:.2} {} = {:.2}x{:.2}x{:.2} {}",
                values[0],
                values[1],
                values[2],
                self.from,
                converted[0],
                converted[1],
                converted[2],
                self.to
            ),
            _ => eprintln!("--value must be a single number or exactly 3 values."),
        }
    }
}

fn parse_length_value(input: &str) -> Result<Vec<f64>, String> {
    let delimiters = ['x', '*', ',', ' '];

    let parts: Vec<&str> = input
        .split(|c| delimiters.contains(&c))
        .filter(|s| !s.trim().is_empty())
        .collect();

    match parts.len() {
        1 | 3 => {
            let mut values = Vec::new();
            for part in parts {
                let val = part
                    .trim()
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid number '{}': {}", part, e))?;
                values.push(val);
            }
            Ok(values)
        }
        other => Err(format!(
            "Expected 1 or 3 numeric values, got {}: '{}'",
            other, input
        )),
    }
}
