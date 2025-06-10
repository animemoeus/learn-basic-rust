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

}