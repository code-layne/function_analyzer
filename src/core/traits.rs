use crate::core::types::{Domain, FunctionAnalysis, Point, Range};

pub trait Function {
    fn evaluate(&self, x: f64) -> f64;

    fn domain(&self) -> Domain;
    fn range(&self) -> Range;
}

pub trait AnalyzableFunction: Function {
    fn analyze(&self) -> FunctionAnalysis;

    fn y_intercept(&self) -> Option<Point> {
        match self.domain() {
            Domain::AllRealNumbers => Some(Point::new(0.0, self.evaluate(0.0))),
            Domain::Intervals(intervals) => {
                let zero_in_domain = intervals.iter().any(interval_contains_zero);
                if zero_in_domain {
                    Some(Point::new(0.0, self.evaluate(0.0)))
                } else {
                    None
                }
            }
            Domain::Empty => None,
        }
    }
}

fn interval_contains_zero(interval: &crate::core::types::Interval) -> bool {
    use crate::core::types::Bound;

    let left_ok = match interval.start {
        Bound::NegInfinity => true,
        Bound::Finite(x) => {
            if interval.include_start {
                x <= 0.0
            } else {
                x < 0.0
            }
        }
        Bound::PosInfinity => false,
    };

    let right_ok = match interval.end {
        Bound::PosInfinity => true,
        Bound::Finite(x) => {
            if interval.include_end {
                0.0 <= x
            } else {
                0.0 < x
            }
        }
        Bound::NegInfinity => false,
    };

    left_ok && right_ok
}
