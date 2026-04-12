use crate::functions::polynomial::Polynomial;

#[derive(Debug, Clone)]
pub struct Radical {
    /// Coefficients in descending degree order.
    /// Example: x^2 - 4x + 3 => vec![1.0, -4.0, 3.0]
    pub index: u32,
    pub radicand: Polynomial,
}

impl Radical {
    pub fn new(index: u32, radicand: Polynomial) -> Radical {
        Radical { index, radicand }
    }
}
