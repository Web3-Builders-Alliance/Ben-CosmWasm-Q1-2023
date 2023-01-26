// Data types in Rust

/*
Scalar Types:
    - Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
    - Floats: f32, f64
    - Boolean: bool
    - Characters: char (a single character, signified by single quotes)
Compound Types:
    - Tuples
    - Arrays

The compiler must know the types of all variables at compile time,
but it can usually infer what type we want to use based on the value and how it is used.
*/

pub fn run() {
    // the default integer type is i32
    let x = 2;

    // the default float type is f64
    let y = 3.5;

    // add a variable with an explicit data type
    let z: i64 = 123456789;

    // find max size
    println!("Max i32 byte size: {}", std::i32::MAX);
    println!("Max i64 byte size: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = false;

    // get boolean from expression
    let is_greater: bool = 10 > 9;

    // define char variable
    let a2 = 'a';
    let emoji = '\u{1F601}';

    println!("{:?}", (x, y, z, is_active, is_greater, a2, emoji));
}
