#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![allow(incomplete_features)]

mod eci;

pub use eci::Eci64;

#[cfg(feature = "macros")]
pub use eci_macro::unit;
