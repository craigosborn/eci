#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod eci;

pub use eci::Eci64;

#[cfg(feature = "macros")]
pub use eci_macro::unit;
