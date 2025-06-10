fn main() {
    println!("Hello, world!");
    
    println!("Hello arter!")
}

#[test]
fn hello_test(){
    println!("Hello, test!");
}

#[test]
fn test_variable(){
    // Immutable variable
    let name = "Arter Tendean";
    println!("Hello {name}")
}

#[test]
fn test_mutable(){
    // Mutable variable
    let mut name = "Arter Tendean";
    println!("Hello {name}");

    name = "Tendean Arter";
    println!("Hello {name}!")
}

#[test]
fn shadowing(){
    // Shadowing variable
    let name = "Arter Tendean";
    println!("Hello {name}");

    let name = 1337;
    println!("Hello {name}");
}

#[test]
fn explicit(){
    let age :u8 = 100;
    println!("Your age is {age}");
}

#[test]
fn number(){
    // Default type is i32
    let a = 10;
    println!("a = {a}");

    // Default type is f64
    let b = 3.14;
    println!("b = {b}");
}

#[test]
fn number_conversion(){
    // Number conversion
    // i8, i16, i32, i64, i128

    let a :i8 = 10;
    println!("a = {a}");

    let b :i16 = a as i16;
    println!("b = {b}");

    let c :i32 = b as i32;
    println!("c = {c}");

    let d :i64 = c as i64;
    println!("d = {d}");

    let e :i128 = a as i128;
    println!("e = {e}");

    // Overflow example
    // let mut f :i16 = 32767;
    // println!("f = {f}");
    // f += 1; // This will cause an overflow in debug mode
    // println!("f after overflow = {f}");


    // Converting to smaller types
    let f :i16 = 32767;
    println!("f = {f}");

    let g :i8 = f as i8; // This will truncate the value
    println!("g = {g}"); // g will be -1 due to overflow
}

#[test]
fn numeric_operators() {
    let a = 10;
    let b = 3;

    // Addition
    let sum = a + b;
    println!("Sum: {sum}");

    // Subtraction
    let difference = a - b;
    println!("Difference: {difference}");

    // Multiplication
    let product = a * b;
    println!("Product: {product}");

    // Division
    let quotient = a / b;
    // let quotient:f64 = a as f64 / b as f64;
    println!("Quotient: {quotient}");

    // Remainder
    let remainder = a % b;
    println!("Remainder: {remainder}");
}