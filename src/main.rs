fn main() {
    println!("Hello, world!");

    println!("Hello arter!")
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn test_variable() {
    // Immutable variable
    let name = "Arter Tendean";
    println!("Hello {name}")
}

#[test]
fn test_mutable() {
    // Mutable variable
    let mut name = "Arter Tendean";
    println!("Hello {name}");

    name = "Tendean Arter";
    println!("Hello {name}!")
}

#[test]
fn shadowing() {
    // Shadowing variable
    let name = "Arter Tendean";
    println!("Hello {name}");

    let name = 1337;
    println!("Hello {name}");
}

#[test]
fn explicit() {
    let age: u8 = 100;
    println!("Your age is {age}");
}

#[test]
fn number() {
    // Default type is i32
    let a = 10;
    println!("a = {a}");

    // Default type is f64
    let b = 3.14;
    println!("b = {b}");
}

#[test]
fn number_conversion() {
    // Number conversion
    // i8, i16, i32, i64, i128

    let a: i8 = 10;
    println!("a = {a}");

    let b: i16 = a as i16;
    println!("b = {b}");

    let c: i32 = b as i32;
    println!("c = {c}");

    let d: i64 = c as i64;
    println!("d = {d}");

    let e: i128 = a as i128;
    println!("e = {e}");

    // Overflow example
    // let mut f :i16 = 32767;
    // println!("f = {f}");
    // f += 1; // This will cause an overflow in debug mode
    // println!("f after overflow = {f}");

    // Converting to smaller types
    let f: i16 = 32767;
    println!("f = {f}");

    let g: i8 = f as i8; // This will truncate the value
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
fn boolean() {
    // Boolean values
    let a = true;
    let b: bool = false;
    println!("a = {a}, b = {b}");
}

#[test]
fn comparison() {
    // Comparison operators
    let a = 10;
    let b = 20;
    println!("a = {a}, b = {b}");

    println!("a == b: {}", a == b); // Equal
    println!("a != b: {}", a != b); // Not equal
    println!("a < b: {}", a < b); // Less than
    println!("a > b: {}", a > b); // Greater than
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
fn tuple() {
    // Tuple type
    let data: (isize, f64, char) = (42, 3.14, 'A');
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
fn unit() {
    println!("Hello, unit!");
}
#[test]
fn test_unit() {
    // Test empty tuple
    let result = unit();
    println!("Result of unit function: {:?}", result);
}

#[test]
fn array() {
    // Array

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array data: {:?}", array);

    let first_element = array[0];
    println!("First element: {}", first_element);

    let second_element = array[1];
    println!("Second element: {}", second_element);

    // Mutable array
    let mut mutable_array: [i8; 3] = [1, 2, 3];
    println!("Mutable array data: {:?}", mutable_array);

    mutable_array[0] = 10; // Change first element
    mutable_array[1] = 20; // Change second element
    println!("Mutable array data: {:?}", mutable_array);

    // Get length of array
    let length = array.len();
    println!("Array length: {length}");
}

#[test]
fn two_dimensional_array() {
    let array: [[u8; 2]; 2] = [[1, 2], [3, 4]];
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

const PI: f64 = 3.14159;
#[test]
fn constant_example() {
    // Using a constant
    println!("The value of PI is: {}", PI);

    // Constants must have a type annotation
    const MAX_SIZE: usize = 100;
    println!("The maximum size is: {}", MAX_SIZE);
}

#[test]
fn variable_scope() {
    let name = "Arter Tendean";

    println!("The value of name is: {}", name);

    {
        let name = "Tendean Arter"; // Shadowing in inner scope
        println!("The value of name in inner scope is: {}", name);
    }
    println!("The value of name after inner scope is: {}", name);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Hello");

    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Hello");

    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}

#[test]
fn string() {
    let name: &str = "  Arter Tendean  ";
    let trim = name.trim();
    println!("Name: {trim}");
}

#[test]
fn string_type() {
    let mut name: String = String::from("Arter");
    name.push_str(" Tendean");
    println!("Name: {name}");

    let name2 = name.replace("Arter Tendean", "arter tendean");
    println!("Name2: {name2}");
}

#[test]
fn ownership_movement() {
    let name_1 = String::from("Arter Tendean");
    println!("Name_1: {name_1}");

    let name_2 = name_1; // Ownership is moved, name_1 is no longer valid
    // println!("Name_1 after move: {name_1}"); // This will cause a compile error
    println!("Name_2: {name_2}");
}

#[test]
fn clone() {
    let name_1 = String::from("Arter Tendean");
    println!("Name_1: {name_1}");

    let name_2 = name_1.clone(); // Cloning creates a deep copy
    println!("Name_1 after clone: {name_1}");
    println!("Name_2: {name_2}");
}

#[test]
fn if_expression() {
    let waifu = "tkg";
    let sentence: &str;

    if waifu.to_lowercase() == "Takagi".to_lowercase() {
        println!("My waifu is Takagi!");
    } else if waifu.to_lowercase() == "Shinomiya".to_lowercase() {
        println!("My waifu is Shinomiya!");
    } else {
        println!("I don't know who my waifu is.");
    }

    // Using if as an expression
    sentence = if waifu.to_lowercase() == "Takagi".to_lowercase() {
        "My waifu is Takagi!"
    } else if waifu.to_lowercase() == "Shinomiya".to_lowercase() {
        "My waifu is Shinomiya!"
    } else {
        "I don't know who my waifu is."
    };
    println!("Sentence: {sentence}");
}

#[test]
fn loop_expresion() {
    let mut counter: i64 = 0;

    loop {
        println!("Counter: {counter}");

        if counter == 100 {
            println!("Counter reached 100, breaking the loop.");
            break; // Break the loop when counter reaches 100
        }

        counter += 1; // Increment the counter
    }
}

#[test]
fn loop_return_value() {
    let mut counter: i64 = 0;

    let result = loop {
        println!("Counter: {counter}");

        if counter == 100000 {
            println!("Counter reached 100, breaking the loop.");
            break "Loop finished"; // Return a value when breaking the loop
        }

        counter += 1; // Increment the counter
    };

    println!("Result of loop: {result}");
}

#[test]
fn nested_loop() {
    let mut counter: i64 = 0;
    loop {
        let mut counter2: i64 = 0;

        loop {
            print!("*");
            if counter2 == counter {
                println!();
                break; // Break the inner loop when counter2 reaches 10
            }

            counter2 += 1; // Increment the inner counter
        }

        if counter == 10 {
            println!("Counter reached 10, breaking the outer loop.");
            break; // Break the outer loop when counter reaches 10
        }

        counter += 1; // Increment the outer counter
    }
}

#[test]
fn loop_label() {
    let mut i = 1;
    'arter: loop {
        let mut j = 1;
        loop {
            println!("{i} x {j} = {}", i * j);
            j += 1; // Increment inner loop counter
            if j > 10 {
                break; // Break the inner loop when j exceeds 10
            }

            //  break 'arter; // Break the outer loop using label
            if j == 7 {
                break 'arter;
            }
        }
        i += 1; // Increment outer loop counter
        if i > 10 {
            println!("Outer loop finished.");
            break; // Break the outer loop when i exceeds 10
        }
    }
}

#[test]
fn while_loop(){
    let mut counter = 0;
    
    while counter < 1000000 {
        println!("Counter: {counter}");
        counter += 1; // Increment the counter
    }
}