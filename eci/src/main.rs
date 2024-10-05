#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use eci::{unit, Eci64};

fn main() -> Result<(),String> {
    println!("Intervals follow operations:");
    let x = unit!(0.1);
    let y = unit!(0.6);
    println!("  {x:?} + {y:?} = {:?}", x + y);
    println!("  {x:?} - {y:?} = {:?}", x - y);
    println!("  {x:?} * {y:?} = {:?}", x * y);

    println!("Signed intervals:");
    let a = Eci64::<-1, 1>::new(0.5)?;
    let b = Eci64::<-1, 1>::new(-0.5)?;
    println!("  {a:?} + {b:?} = {:?}", a + b);
    println!("  {a:?} - {b:?} = {:?}", a - b);
    println!("  {a:?} * {b:?} = {:?}", a * b);

    println!("Operations between elements on different intervals:");
    let m = Eci64::<4, 10>::new(8.0)?;
    let n = Eci64::<2, 20>::new(15.0)?;
    println!("  {m:?} + {n:?} = {:?}", m + n);
    println!("  {m:?} - {n:?} = {:?}", m - n);
    println!("  {m:?} * {n:?} = {:?}", m * n);

    println!("Trig:");
    let deg = Eci64::<-180, 180>::new(60.0)?;
    let oh = deg.cos_degrees();
    println!("  cos({deg:?}) = {:.1?}", oh);
    println!("  acos({oh:.1?}) = {:.1?}", oh.acos_degrees());
    println!("  asin({oh:.1?}) = {:.1?}", oh.asin_degrees());

    Ok(())
}
