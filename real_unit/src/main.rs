use real_unit::{ ru64, RealUnit64};

fn main() {
    let x = ru64!(0.5);
    println!("x: {x}");

    let y = ru64!(0.3);
    println!("y: {y}");

    println!("x + y = {}", x + y);

    println!("x * 100.0 = {}%", x * 100.0);
 }
