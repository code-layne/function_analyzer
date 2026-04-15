use crate::functions::polynomial::Polynomial;

#[derive(Debug, Clone)]
pub struct Rational {
    /// Coefficients in descending degree order.
    /// Example: x^2 - 4x + 3 => vec![1.0, -4.0, 3.0]
    numerator: Polynomial,
    denominator: Polynomial,
}

impl Rational {
    pub fn new(numerator: Polynomial, denominator: Polynomial) -> Rational {
        Rational {
            numerator,
            denominator,
        }
    }

    pub fn asymptote(&self) -> Polynomial {
        let numerator = &self.numerator;
        let denominator = &self.denominator;
        if numerator.degree() == denominator.degree() {
            let ratio = numerator.leading_coefficient() / denominator.leading_coefficient();
            return Polynomial::new(vec![ratio]);
        } else if numerator.degree() > denominator.degree() {
            return Polynomial::new(vec![]);
        } else {
            return Polynomial::new(vec![0.0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_horizontal_asymptote_zero() {
        let numerator = Polynomial::new(vec![1.0, 1.0]);
        let denominator = Polynomial::new(vec![1.0, 1.0, 1.0]);
        let rational = Rational::new(numerator, denominator);
        let asymptote: Polynomial = rational.asymptote();

        let horizontal_expected: Polynomial = Polynomial::new(vec![0.0]);
        assert_eq!(asymptote, horizontal_expected);
    }
}
