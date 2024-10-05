use std::fmt;

mod ops;

#[derive(Clone, Copy, PartialEq)]
pub struct Eci64<const A: i64, const B: i64>(f64);

impl<const A: i64, const B: i64> Eci64<A, B> {
    pub fn new(value: f64) -> Result<Self, String> {
        if value >= A as f64 && value <= B as f64 {
            Ok(Self(value))
        } else {
            Err(format!(
                "Value {value:.1} must be on the closed interval [{A}, {B}]"
            ))
        }
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }

    pub fn domain(&self) -> (i64, i64) {
        (A, B)
    }
}

impl<const A: i64, const B: i64> fmt::Display for Eci64<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}

impl<const A: i64, const B: i64> fmt::Debug for Eci64<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_f64().fmt(f)?;
        write!(f, "_∈[{A},{B}]")
    }
}
