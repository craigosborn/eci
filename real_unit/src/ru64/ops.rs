use super::RealUnit64;
use core::ops::{Add, Div, Mul, Sub};

impl Add for RealUnit64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for RealUnit64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 +rhs.0)
    }
}

impl Add<f64> for RealUnit64 {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        self.as_f64() + rhs
    }
}

impl Sub<f64> for RealUnit64 {
    type Output = f64;

    fn sub(self, rhs: f64) -> Self::Output {
        self.as_f64() - rhs
    }
}

impl Mul<f64> for RealUnit64 {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        self.as_f64() * rhs
    }
}

impl Div<f64> for RealUnit64 {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.as_f64() / rhs
    }
}

// Again
impl Add<RealUnit64> for f64 {
    type Output = f64;

    fn add(self, rhs: RealUnit64) -> Self::Output {
        self + rhs.as_f64()
    }
}

impl Sub<RealUnit64> for f64 {
    type Output = f64;

    fn sub(self, rhs: RealUnit64) -> Self::Output {
        self - rhs.as_f64()
    }
}

impl Mul<RealUnit64> for f64 {
    type Output = f64;

    fn mul(self, rhs: RealUnit64) -> Self::Output {
        self * rhs.as_f64()
    }
}

impl Div<RealUnit64> for f64 {
    type Output = f64;

    fn div(self, rhs: RealUnit64) -> Self::Output {
        self / rhs.as_f64()
    }
}