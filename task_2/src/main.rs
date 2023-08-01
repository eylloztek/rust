use std::io;

fn main() {
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid input. Please enter a valid number.");

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation = match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            println!("Invalid operation. Please enter one of +, -, *, /.");
            return;
        }
    };

    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let second_number: f64 = input.trim().parse().expect("Invalid input. Please enter a valid number.");

    let operation_enum = operation(first_number, second_number);
    let result = calculate(operation_enum);
    println!("Result: {}", result);

}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}
