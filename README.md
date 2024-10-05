# Elements on Closed Intervals

An **experimental** library using Rust's type system to validate the numeric domain of values at compile time.

⚠️ **Warning:** this library is not suitable for real life use. It relies on experimental and incomplete Rust features. 

## Motivation

Rust helps us [make invalid states unrepresentable](https://www.youtube.com/watch?v=z-0-bbc80JM), but it is still possible if you try hard enough. For example, you might make the mistake that ``2_f64.acos()`` returns a number. However, only values on the closed interval ``[-1.0, 1.0]`` are valid arguments despite ``f64`` representing a much larger domain.

By specifying the valid domain of numbers using Rust's type system, the compiler can track these throughout our software and warn us of invalid operations before our software runs.

## Experiment

Can we give Rust's compiler knowledge of a number's domain and what would it look like? Here is my attempt:

```rs
pub struct Eci64<const A: i64, const B: i64>(f64);

let x: Eci64<0, 1> = Eci64::new(1.5)?;
// Error: "Value 1.5 must be on the closed unit interval [0, 1]"
```

That is a bit ugly for a few reasons. Ignoring the const i64 generics for now, let's make it less verbose with a macro for the unit interval [0,1]. A macro can also give compile time errors if try to initalize a value outside of the domain we specify:

```rs
use eci::{Eci64, unit};

let u = unit!(0.5);
println!("{u:?}"); // 0.5_∈[0,1]
```

This value ``u`` has been given a valid domain of ``[0, 1]`` which was enforced when we created the value and is now part of the value's type through the const generics <A, B>. This can be used to track valid domains of our calculations at compile time:

```rs
let x = unit!(0.5);
let y = unit!(0.5);
println!("{:?}", x + y); // 1_∈[0,2]

let m: Eci64<0, 10> = Eci64::new(5.0)?;
let n: Eci64<-2, 2> = Eci64::new(-1.0)?;
println!("{:?}", m * n); // -5_∈[-20,20]
```

We can also specify a finite input domain for functions like ``acos``:

```rs
impl Eci64<-1,1> {
    pub fn acos_degrees(&self) -> Eci64<0,180> {
        Eci64(self.0.acos().to_degrees())
    }
}

let oh: Eci64<-1, 1> = Eci64::new(0.5)?;
println!("{:.0?}", oh.acos_degrees()); // 60_∈[0,180]
```

## Thoughts

There is also an ugly side to validating domain in the type system. First off, the domain interval is defined by const generics ``<A, B>`` of type ``i64`` because Rust [does not support floating-point numbers](https://practice.course.rs/generics-traits/const-generics.html) in const generics. Unfortunately, this restricts our interval endpoints to integers between ±10<sup>19</sup> which omits ±∞ and useful non-integers like π. 

Stable Rust does not yet supprt generic parameters in const operations, which is useful for keeping track of a value's domain through operations. On unstable Rust we can use the ``generic_const_exprs`` feature to do some basic interval arithmetic, but with more complex operations it starts to feel hacky.

```rs
#![feature(generic_const_exprs)]

impl<const A1: i64, const B1: i64, const A2: i64, const B2: i64> Add<Eci64<A2, B2>> for Eci64<A1, B1>
where
    [(); { A1 + A2 } as usize]:,
    [(); { B1 + B2 } as usize]:,
{
    type Output = Eci64<{ A1 + A2 }, { B1 + B2 }>;

    fn add(self, other: Eci64<A2, B2>) -> Self::Output {
        let sum = self.0 + other.0;
        Eci64(sum)
    }
}
```

Several [language features](https://doc.rust-lang.org/stable/unstable-book/language-features.html) are being worked on that can help us make invalid state unrepresentable. Specifying numeric domain in a values's type will likely be feasible at some point in Rust's future. Whether one should is another discussion entirely...

## Building

The experimental features used in this library require the Nightly version of Rust: 

```sh
rustup install nightly
cargo +nightly run
```