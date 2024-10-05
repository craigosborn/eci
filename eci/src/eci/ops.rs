// https://en.wikipedia.org/wiki/Interval_arithmetic

use super::Eci64;
use core::ops::Mul;
use core::ops::{Add, Sub};

impl<const N1: i64, const P1: i64, const N2: i64, const P2: i64> Add<Eci64<N2, P2>>
    for Eci64<N1, P1>
where
    [(); { N1 + N2 } as usize]:,
    [(); { P1 + P2 } as usize]:,
{
    type Output = Eci64<{ N1 + N2 }, { P1 + P2 }>;

    fn add(self, other: Eci64<N2, P2>) -> Self::Output {
        let sum = self.as_f64() + other.as_f64();
        Eci64(sum)
    }
}

impl<const N1: i64, const P1: i64, const N2: i64, const P2: i64> Sub<Eci64<N2, P2>>
    for Eci64<N1, P1>
where
    [(); { N1 - P2 } as usize]:,
    [(); { P1 - N2 } as usize]:,
{
    type Output = Eci64<{ N1 - P2 }, { P1 - N2 }>;

    fn sub(self, other: Eci64<N2, P2>) -> Self::Output {
        let diff = self.as_f64() - other.as_f64();
        Eci64(diff)
    }
}

pub const fn mul_max(n1: i64, p1: i64, n2: i64, p2: i64) -> i64 {
    let nn = n1 * n2;
    let np = n1 * p2;
    let pn = p1 * n2;
    let pp = p1 * p2;
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

pub const fn mul_min(n1: i64, p1: i64, n2: i64, p2: i64) -> i64 {
    let nn = n1 * n2;
    let np = n1 * p2;
    let pn = p1 * n2;
    let pp = p1 * p2;
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

impl<const N1: i64, const P1: i64, const N2: i64, const P2: i64> Mul<Eci64<N2, P2>>
    for Eci64<N1, P1>
where
    [(); { mul_max(N1, P2, N2, P2) } as usize]:,
    [(); { mul_min(N1, P2, N2, P2) } as usize]:,
{
    type Output = Eci64<{ mul_min(N1, P2, N2, P2) }, { mul_max(N1, P2, N2, P2) }>;

    fn mul(self, other: Eci64<N2, P2>) -> Self::Output {
        let prod = self.as_f64() * other.as_f64();
        Eci64(prod)
    }
}

impl<const N: i64, const P: i64> Eci64<N,P> {

}