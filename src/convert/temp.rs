#[derive(Debug, Clone, Copy)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
}

impl TempUnit {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "c" | "celsius" => Some(TempUnit::Celsius),
            "f" | "fahrenheit" => Some(TempUnit::Fahrenheit),
            _ => None,
        }
    }
}

pub fn convert_temperature(value: f64, from: TempUnit, to: TempUnit) -> f64 {
    match (from, to) {
        (TempUnit::Celsius, TempUnit::Fahrenheit) => value * 9.0 / 5.0 + 32.0,
        (TempUnit::Fahrenheit, TempUnit::Celsius) => (value - 32.0) * 5.0 / 9.0,
        _ => value,
    }
}
