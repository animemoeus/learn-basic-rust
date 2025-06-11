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

#[test]
fn augmented_assignment() {
    let mut a = 10;
    let b = 3;

    // Augmented assignment
    a += b; // a = a + b
    println!("a after +=: {a}");

    a -= b; // a = a - b
    println!("a after -=: {a}");

    a *= b; // a = a * b
    println!("a after *=: {a}");

    a /= b; // a = a / b
    println!("a after /=: {a}");

    a %= b; // a = a % b
    println!("a after %=: {a}");
}

#[test]
fn boolean(){
    // Boolean values
    let a = true;
    let b :bool = false;
    println!("a = {a}, b = {b}");
}

#[test]
fn comparison(){
    // Comparison operators
    let a = 10;
    let b = 20;
    println!("a = {a}, b = {b}");

    println!("a == b: {}", a == b); // Equal
    println!("a != b: {}", a != b); // Not equal
    println!("a < b: {}", a < b);   // Less than
    println!("a > b: {}", a > b);   // Greater than
    println!("a <= b: {}", a <= b); // Less than or equal to
    println!("a >= b: {}", a >= b); // Greater than or equal to
}

#[test]
fn boolean_operators() {
    // Boolean operators
    let a = true;
    let b = false;
    println!("a = {a}, b = {b}");

    // Logical AND
    let and_result = a && b;
    println!("a && b: {and_result}");

    // Logical OR
    let or_result = a || b;
    println!("a || b: {or_result}");

    // Logical NOT
    let not_a = !a;
    println!("!a: {not_a}");
}

#[test]
fn char_type() {
    // Character type
    let c: char = 'A';
    println!("Character: {c}");

    // Unicode characters
    let heart: char = '♥';
    println!("Heart character: {heart}");

    // Escape sequences
    let newline: char = '\n';
    println!("Newline character: {newline}");
}

#[test]
fn tuple(){
    // Tuple type
    let data :(isize, f64, char) = (42, 3.14, 'A');
    println!("Tuple data: {:?}", data);
    println!("First element: {}", data.0);
    println!("Second element: {}", data.1);

    let c = data.2;
    println!("Third element: {c}");

    // Destructuring a tuple
    let (x, y, z) = data;
    // Use _ to ignore a value
    // let (x, _, z) = data;
    println!("Destructured: x = {x}, y = {y}, z = {z}");

    // Mutable tuple
    let mut mutable_data = (1, 2.5, 'B');
    println!("Mutable tuple data: {:?}", mutable_data);

    mutable_data.0 = 10; // Change first element
    mutable_data.1 = 3.14; // Change second element
    mutable_data.2 = 'C'; // Change third element
    println!("Modified mutable tuple data: {:?}", mutable_data);
}

#[test]
fn unit(){
    println!("Hello, unit!");
}
#[test]
fn test_unit(){
    // Test empty tuple
    let result = unit();
    println!("Result of unit function: {:?}", result);
}

#[test]
fn array(){
    // Array

    let array :[i32; 5] = [1, 2, 3, 4, 5];
    println!("Array data: {:?}", array);

    let first_element = array[0];
    println!("First element: {}", first_element);

    let second_element = array[1];
    println!("Second element: {}", second_element);

    
    // Mutable array
    let mut mutable_array :[i8; 3] = [1, 2, 3];
    println!("Mutable array data: {:?}", mutable_array);
    
    mutable_array[0] = 10; // Change first element
    mutable_array[1] = 20; // Change second element
    println!("Mutable array data: {:?}", mutable_array);
    
    // Get length of array
    let length = array.len();
    println!("Array length: {length}");
}

#[test]
fn two_dimensional_array(){
    let array :[[u8; 2]; 2] = [[1, 2], [3, 4]];
    println!("2D Array data: {:?}", array);
    
    // Accessing elements
    let first_row = array[0];
    let second_row = array[1];
    println!("First row: {:?}", first_row);
    println!("Second row: {:?}", second_row);
    
    let zero_zero = array[0][0];
    let zero_one = array[0][1];
    println!("First zero: {:?}", zero_zero);
    println!("First one: {:?}", zero_one);
    
}