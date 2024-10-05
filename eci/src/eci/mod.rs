use std::fmt;

mod ops;

#[derive(Clone, Copy, PartialEq)]
pub struct Eci64<const N: i64, const P: i64>(f64);

impl<const N: i64, const P: i64> Eci64<N, P> {
    pub fn new(value: f64) -> Result<Self, String> {
        if value >= N as f64 && value <= P as f64 {
            Ok(Self(value))
        } else {
            Err(format!(
                "Value {value:.1} must be on the closed unit interval [{N}, {P}]"
            ))
        }
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl<const N: i64, const P: i64> fmt::Display for Eci64<N, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}

impl<const N: i64, const P: i64> fmt::Debug for Eci64<N, P>  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_∈[{N},{P}]", self.as_f64())
    }
}
