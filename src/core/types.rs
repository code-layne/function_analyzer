#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bound {
    NegInfinity,
    Finite(f64),
    PosInfinity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub start: Bound,
    pub end: Bound,
    pub include_start: bool,
    pub include_end: bool,
}

impl Interval {
    pub fn new(start: Bound, end: Bound) -> Self {
        Self {
            start,
            end,
            include_start: false,
            include_end: false,
        }
    }

    pub fn closed(start: f64, end: f64) -> Self {
        Self {
            start: Bound::Finite(start),
            end: Bound::Finite(end),
            include_start: true,
            include_end: true,
        }
    }

    pub fn open(start: f64, end: f64) -> Self {
        Self {
            start: Bound::Finite(start),
            end: Bound::Finite(end),
            include_start: false,
            include_end: false,
        }
    }

    pub fn left_open_right_closed(start: f64, end: f64) -> Self {
        Self {
            start: Bound::Finite(start),
            end: Bound::Finite(end),
            include_start: false,
            include_end: true,
        }
    }

    pub fn left_closed_right_open(start: f64, end: f64) -> Self {
        Self {
            start: Bound::Finite(start),
            end: Bound::Finite(end),
            include_start: true,
            include_end: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Domain {
    AllRealNumbers,
    Intervals(Vec<Interval>),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Range {
    AllRealNumbers,
    Intervals(Vec<Interval>),
    Empty,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Monotonicity {
    Increasing,
    Decreasing,
    Constant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonotonicInterval {
    pub interval: Interval,
    pub behavior: Monotonicity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtremumKind {
    RelativeMax,
    RelativeMin,
    AbsoluteMax,
    AbsoluteMin,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extremum {
    pub point: Point,
    pub kind: ExtremumKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AsymptoteKind {
    Vertical,
    Horizontal,
    Slant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Asymptote {
    pub kind: AsymptoteKind,
    pub slope: Option<f64>,
    pub intercept: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl Asymptote {
    pub fn vertical(x: f64) -> Self {
        Self {
            kind: AsymptoteKind::Vertical,
            slope: None,
            intercept: None,
            x: Some(x),
            y: None,
        }
    }

    pub fn horizontal(y: f64) -> Self {
        Self {
            kind: AsymptoteKind::Horizontal,
            slope: None,
            intercept: None,
            x: None,
            y: Some(y),
        }
    }

    pub fn slant(m: f64, b: f64) -> Self {
        Self {
            kind: AsymptoteKind::Slant,
            slope: Some(m),
            intercept: Some(b),
            x: None,
            y: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAnalysis {
    pub domain: Domain,
    pub range: Range,
    pub x_intercepts: Vec<Point>,
    pub y_intercept: Option<Point>,
    pub monotonic_intervals: Vec<MonotonicInterval>,
    pub extrema: Vec<Extremum>,
    pub asymptotes: Vec<Asymptote>,
}

impl Default for FunctionAnalysis {
    fn default() -> Self {
        Self {
            domain: Domain::AllRealNumbers,
            range: Range::Unknown,
            x_intercepts: Vec::new(),
            y_intercept: None,
            monotonic_intervals: Vec::new(),
            extrema: Vec::new(),
            asymptotes: Vec::new(),
        }
    }
}
