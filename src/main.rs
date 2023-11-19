use std::io;

fn main() {
    calculator();
}

fn calculator() {
    println!("Enter first number: ");
    let mut input1 = String::new();
    
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter second number: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    // Conversion of string to int
    let aint: i32 = input1.trim().parse().ok().expect("Program only expected numbers");
    let bint: i32 = input2.trim().parse().ok().expect("Program only expected numbers");

    // Math calculations
    println!("Sum is: {}", aint + bint);
    println!("Subtraction is: {}", aint - bint);
    println!("Multiplication is: {}", aint * bint);
    println!("Division is: {}", aint / bint);
}
