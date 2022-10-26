//prelude
use std::io;


fn main() {
    // println!("Hello, world!");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Failed to read!!");
    // println!("{}", input);


    // Arithmetic 
    // let x: u8 = 5;
    // let x: i8 = 5;
    // let y: i8 = 7;
    // let z = x/y;
    // println!("{}", z);

    // let xc: f32 = 5.0;
    // let yc: f32 = 7.0;
    // let zc = xc/yc;
    // println!("{}", zc);

    // let x2: f32 = 5.0;
    // let y2: f32 = 7.0;
    // let z2 = x2%y2;
    // println!("{}", z2);

    // let d = 4i8;
    // let d1 = 4_i8;
    // let d2 = 4 as i8;
    // let c5 = d1*(y2 as i8);

    // let i = i32::MAX;


    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed!!");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 5);
}
