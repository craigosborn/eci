// https://en.wikipedia.org/wiki/Interval_arithmetic

use super::Eci64;
use core::ops::Mul;
use core::ops::{Add, Sub};

impl<const A1: i64, const B1: i64, const A2: i64, const B2: i64> Add<Eci64<A2, B2>>
    for Eci64<A1, B1>
where
    [(); { A1 + A2 } as usize]:,
    [(); { B1 + B2 } as usize]:,
{
    type Output = Eci64<{ A1 + A2 }, { B1 + B2 }>;

    fn add(self, other: Eci64<A2, B2>) -> Self::Output {
        let sum = self.as_f64() + other.as_f64();
        Eci64(sum)
    }
}

impl<const A1: i64, const B1: i64, const A2: i64, const B2: i64> Sub<Eci64<A2, B2>>
    for Eci64<A1, B1>
where
    [(); { A1 - B2 } as usize]:,
    [(); { B1 - A2 } as usize]:,
{
    type Output = Eci64<{ A1 - B2 }, { B1 - A2 }>;

    fn sub(self, other: Eci64<A2, B2>) -> Self::Output {
        let diff = self.as_f64() - other.as_f64();
        Eci64(diff)
    }
}

pub const fn mul_max(a1: i64, b1: i64, a2: i64, b2: i64) -> i64 {
    let nn = a1 * a2;
    let np = a1 * b2;
    let pn = b1 * a2;
    let pp = b1 * b2;
    if nn > np && nn > pn && nn > pp {
        nn
    } else if np > pn && np > pp {
        np
    } else if pn > pp {
        pn
    } else {
        pp
    }
}

pub const fn mul_min(a1: i64, b1: i64, a2: i64, b2: i64) -> i64 {
    let nn = a1 * a2;
    let np = a1 * b2;
    let pn = b1 * a2;
    let pp = b1 * b2;
    if nn < np && nn < pn && nn < pp {
        nn
    } else if np < pn && np < pp {
        np
    } else if pn < pp {
        pn
    } else {
        pp
    }
}

impl<const A1: i64, const B1: i64, const A2: i64, const B2: i64> Mul<Eci64<A2, B2>>
    for Eci64<A1, B1>
where
    [(); { mul_max(A1, B1, A2, B2) } as usize]:,
    [(); { mul_min(A1, B1, A2, B2) } as usize]:,
{
    type Output = Eci64<{ mul_min(A1, B1, A2, B2) }, { mul_max(A1, B1, A2, B2) }>;

    fn mul(self, other: Eci64<A2, B2>) -> Self::Output {
        let prod = self.as_f64() * other.as_f64();
        Eci64(prod)
    }
}

impl<const N: i64, const P: i64> Eci64<N, P> {
    pub fn cos_degrees(&self) -> Eci64<-1, 1> {
        Eci64(self.0.to_radians().cos())
    }

    pub fn sin_degrees(&self) -> Eci64<-1, 1> {
        Eci64(self.0.to_radians().sin())
    }
}

impl Eci64<-1, 1> {
    pub fn acos_degrees(&self) -> Eci64<0, 180> {
        Eci64(self.0.acos().to_degrees())
    }

    pub fn asin_degrees(&self) -> Eci64<-90, 90> {
        Eci64(self.0.asin().to_degrees())
    }
}
