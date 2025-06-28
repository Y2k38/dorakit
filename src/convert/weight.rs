#[derive(Debug, Clone, Copy)]
pub enum WeightUnit {
    Kilograms,
    Pounds,
}

impl WeightUnit {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "kg" | "kilograms" => Some(WeightUnit::Kilograms),
            "lb" | "pounds" => Some(WeightUnit::Pounds),
            _ => None,
        }
    }
}

pub fn convert_weight(value: f64, from: WeightUnit, to: WeightUnit) -> f64 {
    match (from, to) {
        (WeightUnit::Kilograms, WeightUnit::Pounds) => value * 2.20462,
        (WeightUnit::Pounds, WeightUnit::Kilograms) => value / 2.20462,
        _ => value,
    }
}
