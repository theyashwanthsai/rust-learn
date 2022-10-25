fn main() {
    // println!("Hello, world!");

    // Integers: 
    // i8 0 - 2^8 -1 || u8 -2^7 to 2^7 -1
    // i16 u16
    // i32 u32
    // i64 u64
    // i128 u128
    let intnum: i8 = 56;
    // or
    let intnum1 = 56;


    // Floating-point:
    // f32 || f64
    let floatnum: f32 = 78.7;
    // or
    let floatnum1 = 78.7; // default f64

    
    // Boolean:
    // bool: true or 1 || false or 2
    let booll: bool = true;
    // or
    let bool2 = true;


    // Characters:
    // char
    let letter: char = 'a';
    // or
    let letter1 = 'b';

    // Tuples
    // let tup: (i32, bool, char) = (34, true, 'a');
    // println!("{}", tup.2);

    let mut tup: (i32, bool, char) = (34, true, 'a');
    // tup.1 = 45;
    tup.1 = false;
    println!("{}", tup.1);

    // Arrays
    // let arr: [i32; 4] = [1, 2, 3, 4];
    let arr = [1, 2, 3, 4];
    println!("{}", arr[0]);

}
