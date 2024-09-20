use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn main() {
    let mut input = String::new();
    println!("Enter first number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid input");

    let mut sign = String::new();
    println!("Enter operation: ");
    io::stdin()
        .read_line(&mut sign)
        .expect("Failed to read input");

    input.clear();
    println!("Enter second number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let second_number: f64 = input.trim().parse().expect("Invalid input");

    let expression = match sign.trim() {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid sign");
            return;
        }
    };

    let result = calculate(expression);

    println!("Result: {}", result);
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if y == 0.0 {
                println!("Can't divide by zero");
                return 0.0;
            }
            x / y
        }
    }
}
