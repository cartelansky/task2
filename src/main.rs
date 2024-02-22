use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(x: Operation) -> f64 {
    match x {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(c, d) => c - d,
        Operation::Multiply(e, f) => e * f,
        Operation::Divide(q, w) => q / w,
    }
}

fn main() {
    println!("Enter first number:");
    let mut num1_str = String::new();
    io::stdin()
        .read_line(&mut num1_str)
        .expect("Failed to read input");
    let num1 = num1_str
        .trim()
        .parse::<f64>()
        .expect("Invalid number format");

    println!("Enter operation (+, -, *, /):");
    let mut op_str = String::new();
    io::stdin()
        .read_line(&mut op_str)
        .expect("Failed to read input");
    let op = op_str.trim().chars().next().expect("Invalid operation");

    println!("Enter second number:");
    let mut num2_str = String::new();
    io::stdin()
        .read_line(&mut num2_str)
        .expect("Failed to read input");
    let num2 = num2_str
        .trim()
        .parse::<f64>()
        .expect("Invalid number format");

    let operation = match op {
        '+' => Operation::Add(num1, num2),
        '-' => Operation::Subtract(num1, num2),
        '*' => Operation::Multiply(num1, num2),
        '/' => Operation::Divide(num1, num2),
        _ => panic!("Invalid operation"),
    };

    let result = calculate(operation);
    println!("Result: {}", result);
}
