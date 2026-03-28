use crate::functions::polynomial::Polynomial;

#[derive(Debug, Clone)]
pub struct Rational {
    /// Coefficients in descending degree order.
    /// Example: x^2 - 4x + 3 => vec![1.0, -4.0, 3.0]
    pub numerator: Polynomial,
    pub denominator: Polynomial,
}

impl Rational {
    pub fn new(numerator: Polynomial, denominator: Polynomial) -> Rational {
        Rational { numerator, denominator }
    }
}