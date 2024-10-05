#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![allow(incomplete_features)]

use real_unit::{unit, Eci64};

fn main() {
    println!("Intervals follow operations:");
    let x = unit!(0.1);
    let y = unit!(0.6);
    println!("  {x:?} + {y:?} = {:?}", x + y);
    println!("  {x:?} - {y:?} = {:?}", x - y);
    println!("  {x:?} * {y:?} = {:?}", x * y);

    println!("Signed intervals:");
    let a = Eci64::<4, 10>::new(0.5).unwrap();
    let b = Eci64::<4, 10>::new(-0.5).unwrap();
    println!("  {a:?} + {b:?} = {:?}", a + b);
    println!("  {a:?} - {b:?} = {:?}", a - b);
    println!("  {a:?} * {b:?} = {:?}", a * b);

    println!("Operations between elements on different intervals:");
    let m = Eci64::<4, 10>::new(8.0).unwrap();
    let n = Eci64::<2, 20>::new(15.0).unwrap();
    println!("  {m:?} + {n:?} = {:?}", m + n);
    println!("  {m:?} - {n:?} = {:?}", m - n);
    println!("  {m:?} * {n:?} = {:?}", m * n);
}
