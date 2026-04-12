#[derive(Debug, Clone)]
pub struct Polynomial {
    /// Coefficients in descending degree order.
    /// Example: x^2 - 4x + 3 => vec![1.0, -4.0, 3.0]
    coefficients: Vec<f64>,
}
#[derive(Debug)]
pub enum PolynomialError {
    DivisionByZeroPolynomial,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        let first_nonzero = coefficients
            .iter()
            .position(|c| c.abs() > 1e-12)
            .unwrap_or(coefficients.len().saturating_sub(1));

        let trimmed = coefficients[first_nonzero..].to_vec();
        Self {
            coefficients: trimmed,
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len().saturating_sub(1)
    }

    pub fn negate(&self) -> Polynomial {
        let negated_coefficients: Vec<f64> = self.coefficients.iter().map(|c| -c).collect();

        Polynomial::new(negated_coefficients)
    }

    pub fn subtract(&self, other: &Polynomial) -> Polynomial {
        let neg_other = other.negate();
        self.add(&neg_other)
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let max_len = self.coefficients.len().max(other.coefficients.len());

        let mut result = vec![0.0; max_len];

        let offset_self = max_len - self.coefficients.len();
        let offset_other = max_len - other.coefficients.len();

        for index in 0..max_len {
            let a = if index >= offset_self {
                self.coefficients[index - offset_self]
            } else {
                0.0
            };

            let b = if index >= offset_other {
                other.coefficients[index - offset_other]
            } else {
                0.0
            };

            result[index] = a + b;
        }

        Polynomial::new(result)
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        // Horner's method
        self.coefficients.iter().fold(0.0, |acc, &c| acc * x + c)
    }

    pub fn derivative(&self) -> Self {
        let degree = self.degree();

        if degree == 0 {
            return Self::new(vec![0.0]);
        }

        let first_derivative = self
            .coefficients
            .iter()
            .enumerate()
            .take(self.coefficients.len() - 1)
            .map(|(i, &c)| {
                let power = (degree - i) as f64;
                c * power
            })
            .collect();

        Self::new(first_derivative)
    }

    pub fn y_intercept(&self) -> f64 {
        self.evaluate(0.0)
    }

    pub fn leading_coefficient(&self) -> f64 {
        self.coefficients[0]
    }

    pub fn format_pretty(&self) -> String {
        let degree = self.degree();
        let mut parts = Vec::new();

        for (i, &c) in self.coefficients.iter().enumerate() {
            if c.abs() < 1e-12 {
                continue;
            }

            let power = degree - i;
            let sign = if c < 0.0 { "-" } else { "+" };
            let abs_c = c.abs();

            let term = match power {
                0 => format!("{abs_c}"),
                1 => {
                    if (abs_c - 1.0).abs() < 1e-12 {
                        "x".to_string()
                    } else {
                        format!("{abs_c}x")
                    }
                }
                _ => {
                    if (abs_c - 1.0).abs() < 1e-12 {
                        format!("x^{power}")
                    } else {
                        format!("{abs_c}x^{power}")
                    }
                }
            };

            parts.push((sign.to_string(), term));
        }

        if parts.is_empty() {
            return "0".to_string();
        }

        let mut out = String::new();
        for (idx, (sign, term)) in parts.into_iter().enumerate() {
            if idx == 0 {
                if sign == "-" {
                    out.push_str("- ");
                }
                out.push_str(&term);
            } else {
                out.push_str(&format!(" {} {}", sign, term));
            }
        }

        out
    }

    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let mut result = vec![0.0; self.coefficients.len() + other.coefficients.len() - 1];

        for i in 0..self.coefficients.len() {
            for j in 0..other.coefficients.len() {
                result[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }

        Polynomial::new(result)
    }

    pub fn divide(
        &self,
        divisor: &Polynomial,
    ) -> Result<(Polynomial, Polynomial), PolynomialError> {
        if divisor.is_zero() {
            return Err(PolynomialError::DivisionByZeroPolynomial);
        }

        if self.degree() < divisor.degree() {
            return Ok((Polynomial::new(vec![0.0]), self.clone()));
        }

        if divisor.degree() == 1 && divisor.leading_coefficient() == 1.0 {
            let root = self.coefficients.last().unwrap();
            let (quotient, remainder) = self.synthetic_divide(-root);
            return Ok((quotient, Polynomial::new(vec![remainder])));
        }

        let mut remainder = self.clone();
        let quotient_len = self.degree() - divisor.degree() + 1;
        let mut quotient = vec![0.0; quotient_len];

        while !remainder.is_zero() && remainder.degree() >= divisor.degree() {
            let coefficient = remainder.coefficients[0] / divisor.coefficients[0];
            let power_diff = remainder.degree() - divisor.degree();

            let q_index = quotient_len - power_diff - 1;
            quotient[q_index] = coefficient;

            let term = Polynomial::monomial(coefficient, power_diff);
            let product = divisor.multiply(&term);
            remainder = remainder.subtract(&product);
        }

        Ok((Polynomial::new(quotient), remainder))
    }

    pub fn monomial(coefficient: f64, power: usize) -> Polynomial {
        let mut coefficients = vec![coefficient];
        coefficients.extend(std::iter::repeat_n(0.0, power));
        Polynomial::new(coefficients)
    }

    pub fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|c| c.abs() < 1e-12)
    }

    fn synthetic_divide(&self, r: f64) -> (Polynomial, f64) {
        if self.degree() == 0 {
            return (Polynomial::new(vec![0.0]), self.coefficients[0]);
        }

        let mut result = Vec::with_capacity(self.coefficients.len());
        result.push(self.coefficients[0]);

        for i in 1..self.coefficients.len() {
            let next = self.coefficients[i] + result[i - 1] * r;
            result.push(next);
        }

        let remainder = result.pop().unwrap();
        let quotient = Polynomial::new(result);

        (quotient, remainder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_fx_test() {
        let null_fx = Polynomial::new(Vec::new());
        assert_eq!(null_fx.degree(), 0);
        assert_eq!(null_fx.y_intercept(), 0.0);
        assert!(null_fx.is_zero());
    }

    #[test]
    fn add_equal_degrees() {
        let p1 = Polynomial::new(vec![1.0, 2.0]);
        let p2 = Polynomial::new(vec![2.0, 3.0]);
        let result = p1.add(&p2);
        assert_eq!(result.degree(), 1);
        assert_eq!(result.coefficients, vec!(3.0, 5.0));
    }

    #[test]
    fn add_different_degrees() {
        let p1 = Polynomial::new(vec![1.0, 2.0]);
        let p2 = Polynomial::new(vec![2.0, 3.0, 4.0]);
        let result = p1.add(&p2);
        assert_eq!(result.degree(), 2);
        assert_eq!(result.coefficients, vec!(2.0, 4.0, 6.0));
    }

    #[test]
    fn subtract_same_degree() {
        let p1 = Polynomial::new(vec![1.0, 2.0]);
        let p2 = Polynomial::new(vec![2.0, 3.0]);
        let result = p2.subtract(&p1);
        assert_eq!(result.degree(), 1);
        assert_eq!(result.coefficients, vec!(1.0, 1.0));
    }
    #[test]
    fn subtract_inverse() {
        let p1 = Polynomial::new(vec![1.0, 2.0]);
        let p2 = Polynomial::new(vec![1.0, 2.0]);
        let result = p2.subtract(&p1);
        assert_eq!(result.degree(), 0);
        assert!(result.is_zero());
    }

    #[test]
    fn multiply_monomial_binomial() {
        let monomial = Polynomial::new(vec![2.0, 0.0]);
        assert_eq!(monomial.degree(), 1);
        let binomial = Polynomial::new(vec![2.0, 3.0]);
        assert_eq!(binomial.degree(), 1);
        let product = monomial.multiply(&binomial);
        assert_eq!(product.degree(), 2);
        assert_eq!(product.coefficients, vec!(4.0, 6.0, 0.0));
    }

    #[test]
    fn multiply_binomial_binomial() {
        let binomial1 = Polynomial::new(vec![1.0, -3.0]);
        let binomial2 = Polynomial::new(vec![1.0, 2.0]);
        let product = binomial1.multiply(&binomial2);
        assert_eq!(product.degree(), 2);
        assert_eq!(product.coefficients, vec!(1.0, -1.0, -6.0));
    }
    #[test]
    fn multiply_binomial_trinomial() {
        let binomial = Polynomial::new(vec![1.0, -3.0]);
        let trinomial = Polynomial::new(vec![1.0, 2.0, 3.0]);
        let product = binomial.multiply(&trinomial);
        assert_eq!(product.degree(), 3);
        assert_eq!(product.coefficients, vec!(1.0, -1.0, -3.0, -9.0));
    }

    #[test]
    fn multiply_trinomial_trinomial() {
        let trinomial1 = Polynomial::new(vec![1.0, 2.0, 1.0]);
        let trinomial2 = Polynomial::new(vec![1.0, 4.0, 4.0]);
        let product = trinomial1.multiply(&trinomial2);
        assert_eq!(product.degree(), 4);
        assert_eq!(product.coefficients, vec!(1.0, 6.0, 13.0, 12.0, 4.0));
    }

    #[test]
    fn synthetic_division() {
        let trinomial = Polynomial::new(vec![2.0, -2.0, 1.0]);
        let binomial = Polynomial::new(vec![1.0, 3.0]);
        let root = binomial.coefficients.last().unwrap();

        let (dividend, remainder) = trinomial.synthetic_divide(-root);
        assert_eq!(dividend.degree(), 1);
        // todo: fix this remainder should be 0
        assert_eq!(remainder, 25.0);
    }

    #[test]
    fn divides_no_remainder() {
        //todo
    }
    #[test]
    fn divides_with_remainder() {
        //todo
    }
    #[test]
    fn returns_error_divide_by_zero() {
        //todo
    }
}
