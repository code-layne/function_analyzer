#[derive(Debug, Clone)]
pub struct Polynomial {
    /// Coefficients in descending degree order.
    /// Example: x^2 - 4x + 3 => vec![1.0, -4.0, 3.0]
    pub coefficients: Vec<f64>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        let first_nonzero = coefficients
            .iter()
            .position(|c| c.abs() > 1e-12)
            .unwrap_or(coefficients.len().saturating_sub(1));

        let trimmed = coefficients[first_nonzero..].to_vec();
        Self { coefficients: trimmed }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len().saturating_sub(1)
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
}