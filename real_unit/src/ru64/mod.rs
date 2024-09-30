use std::fmt;

mod ops;

#[derive(Clone, Copy, PartialEq)]
pub struct RealUnit64(u64);

impl RealUnit64 {
    pub fn new(value: f64) -> Result<Self, String> {
        if value >= 0.0 && value <= 1.0 {
            Ok(Self((value * u64::MAX as f64) as u64))
        } else {
            Err(format!(
                "Value {value:.1} must be on the closed unit interval [0.0, 1.0]"
            ))
        }
    }

    pub fn zero() -> Self {
        Self(u64::min_value())
    }

    pub fn one() -> Self {
        Self(u64::max_value())
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64 / u64::MAX as f64
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for RealUnit64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}

impl fmt::Debug for RealUnit64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RealUnit({})", self.as_f64())
    }
}
