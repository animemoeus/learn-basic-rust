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
    let name = "Arter Tendean";
    println!("Hello {name}");

    let name = 1337;
    println!("Hello {name}");
}
