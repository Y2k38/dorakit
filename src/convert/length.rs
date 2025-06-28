#[derive(Debug, Clone, Copy)]
pub enum LengthUnit {
    Millimeter,
    Inch,
}

impl LengthUnit {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "mm" | "Millimeter" => Some(LengthUnit::Millimeter),
            "in" | "Inch" => Some(LengthUnit::Inch),
            _ => None,
        }
    }
}

pub fn convert_length(value: f64, from: LengthUnit, to: LengthUnit) -> f64 {
    match (from, to) {
        (LengthUnit::Millimeter, LengthUnit::Inch) => value / 25.4,
        (LengthUnit::Inch, LengthUnit::Millimeter) => value * 25.4,
        _ => value,
    }
}
