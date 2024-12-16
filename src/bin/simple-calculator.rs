// bootcamp-tasks/src/bin/simple-calculator.rs
// # Simple Calculator
// This program allows the user to perform basic arithmetic operations (+, -, *, /) on two numbers.
// 
// Author: Sefa Osmanoğlu

use std::{io, io::Write};

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

/// Calculates the given operation and returns the result.
///
/// # Errors
///
/// Returns an error if the operation is division by zero.
fn calculate(operation: Operation) -> Result<f64, String> {
    match operation {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0.0 {
                Err("Cannot divide by zero!".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
}

/// Read a number from user input with error handling
fn read_number(prompt: &str) -> Result<f64, String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|_| "Error flushing stdout".to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read line".to_string())?;

    input
        .trim()
        .parse()
        .map_err(|_| "Invalid number entered".to_string())
}

fn main() -> Result<(), String> {
    // Get user inputs with error handling
    let num1 = read_number("Enter first number: ")?;
    let operation = {
        print!("Enter the operation (+, -, *, /): ");
        io::stdout().flush().map_err(|_| "Error flushing stdout")?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| "Failed to read line")?;
        input.trim().to_string()
    };
    let num2 = read_number("Enter second number: ")?;

    // Match the operation and check if it is valid
    let calc_operation = match operation.as_str() {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),    
        _ => return Err("Invalid operation!".to_string()),
    };

    // Get the result and print
    match calculate(calc_operation) {
        Ok(result) => {
            println!("{} {} {} = {}", num1, operation, num2, result);
            Ok(())
        }
        Err(e) => Err(e),
    }
}