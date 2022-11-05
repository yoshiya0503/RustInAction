use std::convert::TryInto;
use num::complex::Complex;

fn main() {
    let a = 10;
    let aa = 1_i16;
    let b: i32 = 123456789;
    let c = 30_i32;
    let d = 120.23103_f32;
    let aa_: i32 = aa.try_into().unwrap();
    println!("{}, {}, {}", a, b, c);
    println!("{}", add(a, d));
    if aa_ < a {
        println!("call");
    }
    let complex1 = Complex{re: 10, im: -10};
    let complex2 = Complex::new(10, -10);
    println!("{}", complex1 + complex2)
}

fn add(a: i32, b: f32) -> f32 {
    return (a as f32) + b;
}
