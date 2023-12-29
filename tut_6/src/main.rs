use std::io;
fn main() {
    let x: u8 = 25; //  u8 -> 0-255
    let y: u8 = 10; // i8 -> -128 - 127

    let a: f32 = 108.0;
    let b: f32 = 10.0;

    let z = x + y;
    let z1 = x / y;
    let z2 = x % y;
    println!("{}", z);
    println!("{}", z1);
    println!("{}", z2);

    let c = a + b;
    let c1 = a / b;
    let c2 = a % b;
    println!("{}", c);
    println!("{}", c1);
    println!("{}", c2);

    // Type Casting
    let x = 255u8;
    let y = 10.0_f32;
    let z = 127_000 as i64;
    let a = 10 as i32;

    let ans = z / (a as i64); // always  typecast lower range  to higher range to avoid overflow or underflow

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    println!("{}", ans);

    // String to integers
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("expected to read line");
    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 2);
}
