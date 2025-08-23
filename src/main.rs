fn main() {
    println!("Hello, world!");
    let x: i32 = -5;
    let y: u32 = 1000;
    let z: f32 = 1000.33;

    // addition
    let sum: i32 = 5 + 10;

    // subtraction
    let difference: f32 = 95.5 - 4.3;

    // multiplication
    let product: i32 = 4 * 30;

    // division
    let quotient: f32 = 56.7 / 32.2;
    let truncated: i32 = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // printing values
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("truncated: {}", truncated);
    println!("remainder: {}", remainder);
}

